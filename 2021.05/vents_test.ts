import { assertEquals } from "https://deno.land/std@0.117.0/testing/asserts.ts";
import { parseVents, filterStraight, buildFloor, getDangerCount } from './vents.ts';

const sampleData = `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`

Deno.test("straight vents", () => {
  const lines = parseVents(sampleData);
  const straightLines = filterStraight(lines);
  const floor = buildFloor(straightLines);
  assertEquals(21, floor.size);
  const dangerCount = getDangerCount(floor);
  assertEquals(5, dangerCount);
});

Deno.test("all vents", () => {
  const lines = parseVents(sampleData);
  const floor = buildFloor(lines);
  assertEquals(39, floor.size);
  const dangerCount = getDangerCount(floor);
  assertEquals(12, dangerCount);
});
