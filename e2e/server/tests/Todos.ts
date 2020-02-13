
import { Step, DataStoreFactory, BeforeSpec, BeforeScenario } from "gauge-ts";
import { strict } from "assert";
import fetch from "node-fetch";
import { Config } from "./Config";
import { readFile } from "fs";
const { deepEqual } = strict;

export default class TodosStep {

  @BeforeSpec({ tags: ["register"] })
  async registerTestData() {
    await fetch(`${Config.baseUrl}/v1/test/register`, { method: "post" });
  }

  @BeforeScenario({ tags: ["clear"] })
  async clearTestData() {
    await fetch(`${Config.baseUrl}/v1/test/clear`, { method: "post" });
  }

  @Step("Todoリストを取得するリクエストを実行する")
  async fetchTodoList() {
    const result = await fetch(`${Config.baseUrl}/v1/todos`);
    DataStoreFactory.getScenarioDataStore().put("status", result.status);
    DataStoreFactory.getScenarioDataStore().put("body", await result.json());
  }

  @Step("Todoリストを登録するリクエスト<file>を実行する")
  async registerTodo(file: string) {
    const result = await fetch(`${Config.baseUrl}/v1/todos`, {
      method: "post",
      headers: {
        "content-type": "application/json"
      },
      body: await readFileAsync(`resources/setup/${file}.json`)
    });
    DataStoreFactory.getScenarioDataStore().put("status", result.status);
    DataStoreFactory.getScenarioDataStore().put("body", await result.json());
  }

  @Step("Todoのid<id>を<file>で更新するリクエストを実行する")
  public async updateTodo(id: string, file: any) {
    const result = await fetch(`${Config.baseUrl}/v1/todos/${id}`, {
      method: "put",
      headers: {
        "content-type": "application/json"
      },
      body: await readFileAsync(`resources/setup/${file}.json`)
    });
    DataStoreFactory.getScenarioDataStore().put("status", result.status);
    DataStoreFactory.getScenarioDataStore().put("body", await result.json());
  }

	@Step("ステータスコードが<status>であること")
	async verifyStatusCode(status: string) {
    deepEqual(DataStoreFactory.getScenarioDataStore().get("status"), parseInt(status, 10));
	}
	@Step("レスポンスボディが<file>と一致すること")
	async verifyBody(file: string) {
    const body = await readFileAsync(`resources/expected/${file}.json`);
    deepEqual(DataStoreFactory.getScenarioDataStore().get("body"), JSON.parse(body));
	}
}


async function readFileAsync(path: string): Promise<string> {
  return new Promise((ok, ng) => readFile(path, (err, data) => err ? ng(err) : ok(data.toString())));
}
