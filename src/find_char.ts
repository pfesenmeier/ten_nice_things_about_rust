import { assertEquals } from "https://deno.land/std@0.117.0/testing/asserts.ts";

function findLetter(word: string, letter: string): number | null {
    let result = null;

    for (let i = 0; i < word.length; i++) {
       if (word[i] === letter) {
          result = i;
       }
    }

    return result;
}

Deno.test("test findChar", () => {
  assertEquals(findLetter("hello world", 'w'), 6);
})

