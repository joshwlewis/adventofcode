import { parseVents, filterStraight, buildFloor, getDangerCount } from './vents.ts'

const input = await Deno.readTextFile("./input.txt");
const lines = parseVents(input);
const straightLines = filterStraight(lines);
const straightFloor = buildFloor(straightLines);
const straightDanger = getDangerCount(straightFloor);
console.log(`straight danger count: ${straightDanger}`);
const floor = buildFloor(lines);
const danger = getDangerCount(floor);
console.log(`danger count: ${danger}`);
