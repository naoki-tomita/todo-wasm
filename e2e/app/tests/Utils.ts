import { deepStrictEqual } from "assert";

export async function sleep(ms: number) {
  return new Promise(ok => setTimeout(ok, ms));
}

export async function waitFor(cb: () => Promise<boolean>, ms: number = 2000) {
  for (let i = 0; i < ms / 100; i++) {
    if (await cb()) {
      return;
    }
    await sleep(100);
  }
}

export async function waitAndAssert<T>(cb: () => Promise<T>, expected: T, ms: number = 2000) {
  for (let i = 0; i < ms / 100; i++) {
    if ((await cb()) === expected) {
      return;
    }
    await sleep(100);
  }
  deepStrictEqual(await cb(), expected);
}
