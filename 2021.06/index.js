import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";
import { parseFish, simulateDays, countFish } from './fish.js';

let input = '';
const decoder = new TextDecoder();
for await (const line of readline(Deno.stdin)) {
  input += decoder.decode(line);
}
let fish = parseFish(input)
console.dir(`DAY 0: ${countFish(fish)}`);
fish = simulateDays(fish, 80);
console.dir(`DAY 80: ${countFish(fish)}`);
fish = simulateDays(fish, 176);
console.dir(`DAY 256: ${countFish(fish)}`);
