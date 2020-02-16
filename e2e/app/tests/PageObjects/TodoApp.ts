import { launch, Page } from "puppeteer";
import { Config } from "../Config";

export class TodoApp {
  page: Page;

  constructor(page: Page) {
    this.page = page;
  }

  static async open() {
    const browser = await launch();
    const page = await browser.newPage();
    await page.goto(Config.baseUrl, { waitUntil: "load" });
    return new TodoApp(page);
  }

  inputTodoText(text: string) {
    return this.page.type(`input[type="text"]`, text);
  }

  completeTodoText() {
    return this.page.click(`button`);
  }

  async getTodoList() {
    return await Promise.all((await this.page.$$("li")).map(async it => {
      const [text, checked] = await Promise.all([
        it.$("span").then(it => it.getProperty("textContent")).then(it => it.jsonValue()) as Promise<string>,
        it.$("input").then(it => it.getProperty("checked")).then(it => it.jsonValue()) as Promise<boolean>,
      ]);
      return { text, checked };
    }));
  }

  async getTodoTexts() {
    return (await this.getTodoList()).map(it => it.text);
  }

  async getTodoStats() {
    return (await this.getTodoList()).map(it => it.checked);
  }

  async switchStatus(index: number) {
    await this.page.click(`li:nth-child(${index}) input`);
  }

  close() {
    return this.page.close();
  }
}
