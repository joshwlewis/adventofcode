import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";
import { parseFish, simulateDays } from './fish.js';

let input = '';
const decoder = new TextDecoder();
for await (const line of readline(Deno.stdin)) {
  input += decoder.decode(line);
}
const fish = parseFish(input)
simulateDays(fish, 80);
console.dir(`DAY 80: ${fish.length}`);
// simulateDays(fish, 176);
// console.dir(`DAY 256: ${fish.length}`);
