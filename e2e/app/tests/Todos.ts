
import { Step, BeforeScenario, AfterScenario, AfterStep } from "gauge-ts";
import { equal } from "assert";
import { Browser, launch, Page } from "puppeteer";
import fetch from "node-fetch";
import { Config } from "./Config";
import { writeFile } from "fs";

interface Context {
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

export default class Todos {
  browser: Browser;
  page: Page;

  @BeforeScenario({ tags: ["register"] })
  async registerTestData() {
    await fetch(`${Config.baseUrl}/v1/test/register`, { method: "post" });
  }

  @AfterScenario()
  async closeApp() {
    await this.page.close();
  }

  @AfterStep()
  async afterStep(context: Context) {
    if (context.currentStep.isFailed) {
      await this.captureScreen(`${context.currentSpec.name}_${context.currentScenario.name}_${context.currentStep.step.actualStepText}`);
    }
  }

  async captureScreen(name: string) {
    await this.page.screenshot({ path: `./artifacts/${name}.png`.replace(/\"/g, "") });
  }

  @Step("アプリを起動する")
  public async openApp() {
    this.browser = await launch();
    this.page = await this.browser.newPage();
    await this.page.goto("http://localhost");
  }

  @Step("Todoが<count>件表示されている")
  public async appShowsTodos(countText: string) {
    const count = parseInt(countText, 10);
    await this.page.waitFor(count => document.querySelectorAll("li").length === count, {}, count);
    equal((await this.page.$$("li")).length, count);
  }

  @Step("Todoの<index>件目のテキストが<text>と表示されている")
  public async todoShowsText(indexText: string, text: string) {
    const index = parseInt(indexText);
    const texts = await Promise.all((await this.page.$$("li span")).map(async it => await (await it.getProperty("textContent")).jsonValue()));
    equal(texts[index - 1], text);
  }

  @Step("Todoの<index>件目のステータスが<status>状態となっている")
  public async todoStatusIs(indexText: string, status: string) {
    const index = parseInt(indexText);
    const checkeds = await Promise.all((await this.page.$$("li input")).map(async it => await (await it.getProperty("checked")).jsonValue()));
    equal(checkeds[index - 1], "完了" == status);
  }

	@Step("Todoの入力欄に<text>と入力する")
	public async inputNewTodoText(text: string) {
		await this.page.type(`input[type="text"]`, text);
	}

  @Step("ADD TODOを押下する")
	public async clickAddTodo() {
		await this.page.click(`a.btn`);
  }

	@Step("Todoの<index>件目のステータスを押下する")
	public async switchStatus(indexText: string) {
    const index = parseInt(indexText, 10);
    await this.page.click(`li:nth-child(${index}) input`);
	}
}
