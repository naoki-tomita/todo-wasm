
import { Step, BeforeScenario, AfterScenario, AfterStep } from "gauge-ts";
import fetch from "node-fetch";
import { Config } from "./Config";
import { TodoApp } from "./PageObjects/TodoApp";
import { waitAndAssert } from "./Utils";

interface GaugeContext {
  currentSpec: {
    tags: string[];
    name: string;
    fileName: string;
    isFailed: boolean;
  };
  currentScenario: { tags: string[], name: string, isFailed: boolean };
  currentStep: {
    step: {
      parameters: any[],
      actualStepText: string,
      parsedStepText: string;
    },
    isFailed: boolean;
    stackTrace: string;
    errorMessage: string;
  };
}

const Stat: { [key: string]: boolean } = {
  "完了": true,
  "未完了": false
}

export default class TodosSteps {
  app: TodoApp;

  @BeforeScenario({ tags: ["register"] })
  async registerTestData() {
    await fetch(`${Config.baseUrl}/v1/test/register`, { method: "post" });
  }

  @AfterScenario()
  async closeApp() {
    this.app.close();
  }

  @AfterStep()
  async afterStep(context: GaugeContext) {
    if (context.currentStep.isFailed) {
      await this.captureScreen(
        `${context.currentSpec.name}_${context.currentScenario.name}_${context.currentStep.step.actualStepText}`
      );
    }
  }

  async captureScreen(name: string) {
    await this.app.page.screenshot({ path: `./artifacts/${name}.png`.replace(/\"/g, "") });
  }

  @Step("アプリを起動する")
  public async openApp() {
    this.app = await TodoApp.open();
  }

  @Step("Todoが<count>件表示されている")
  public async appShowsTodos(countText: string) {
    const count = parseInt(countText, 10);
    await waitAndAssert(async () => (await this.app.getTodoList()).length, count);
  }

  @Step("Todoの<index>件目のテキストが<text>と表示されている")
  public async todoShowsText(indexText: string, text: string) {
    const index = parseInt(indexText);
    await waitAndAssert(async () => (await this.app.getTodoTexts())[index - 1], text);
  }

  @Step("Todoの<index>件目のステータスが<status>状態となっている")
  public async todoStatusIs(indexText: string, status: string) {
    const index = parseInt(indexText);
    await waitAndAssert(async () => (await this.app.getTodoStats())[index - 1], Stat[status]);
  }

	@Step("Todoの入力欄に<text>と入力する")
	public async inputNewTodoText(text: string) {
    await this.app.inputTodoText(text);
	}

  @Step("ADD TODOを押下する")
	public async clickAddTodo() {
    await this.app.completeTodoText();
  }

	@Step("Todoの<index>件目のステータスを押下する")
	public async switchStatus(indexText: string) {
    const index = parseInt(indexText, 10);
    await this.app.switchStatus(index);
  }
}
