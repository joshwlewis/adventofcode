export function parseVents(input: string): Line[] {
  return input.split('\n').map((l) => {
    const [start, end] = l.split(/\w*->\w*/).map((ps) => {
      const [x, y] = ps.split(',').map(p => parseInt(p));
      return new Point(x, y)
    });
    return new Line(start, end);
  });
}

export function filterStraight(lines: Line[]): Line[] {
  return lines.filter((line) => line.horizontal || line.vertical);
}

export function buildFloor(lines: Line[]): Floor {
  const map: Floor = new Map();
  lines.forEach((l) => {
    const rise = l.end.y-l.start.y
    const run = l.end.x-l.start.x
    const dy = rise === 0 ? 0 : (rise/Math.abs(rise))
    const dx = run === 0 ? 0 : (run/Math.abs(run))
    let y = l.start.y
    let x = l.start.x
    while (x != l.end.x+dx || y != l.end.y+dy) {
      const key = new Point(x,y).key;
      map.set(key, (map.get(key) || 0) + 1)
      y += dy
      x += dx
    }
  });
  return map;
}

export function getDangerCount(map: Floor): number {
  return Array.from(map.values()).filter((c) => c >= 2).length
}

export type Floor = Map<string, number>

export class Line {
  start: Point;
  end: Point;
  constructor(start: Point, end: Point) {
    this.start = start;
    this.end = end;
  }
  get horizontal(): boolean {
    return this.start.y === this.end.y;
  }
  get vertical(): boolean {
    return this.start.x === this.end.x;
  }
}

export class Point {
  x: number;
  y: number;
  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
  get key(): string {
    return `<${this.x},${this.y}>`
  }
}

