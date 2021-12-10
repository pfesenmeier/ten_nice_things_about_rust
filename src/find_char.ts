import { assertEquals } from "https://deno.land/std@0.117.0/testing/asserts.ts";

function findChar(string: string, c: string): number | null {
    let result = null;

    for (let i = 0; i < string.length; i++) {
       if (string[i] === c) {
          result = i;
       }
    }

    return result;
}

Deno.test("test findChar", () => {
  assertEquals(findChar("hello world", 'w'), 6);
})