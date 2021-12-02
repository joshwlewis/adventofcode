fn main() {
    let directions = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|t| (t.0, t.1.parse::<i32>().unwrap()));

    let mut x = 0;
    let mut y = 0;
    for d in directions.clone() {
        match d.0 {
            "forward" => x = x + d.1,
            "down" => y = y + d.1,
            "up" => y = y - d.1,
            _ => panic!("got weird direction: {}", d.0)
        }
    };
    println!("Part 1: target: (x: {}, y: {}) checksum: {}", x, y, x*y);

    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;
    for d in directions {
        match d.0 {
            "forward" => {
                horiz = horiz + d.1; 
                depth = depth + (aim * d.1);
            },
            "down" => aim = aim + d.1,
            "up" => aim = aim - d.1,
            _ => panic!("got weird direction: {}", d.0)
        }
    };
    println!("Part 2: aim: {}, horiz: {}, depth: {}, checksum: {}", aim, horiz, depth, horiz*depth);
}
