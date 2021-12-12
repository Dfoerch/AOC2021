pub fn part1() -> i32{
    let input = include_str!("../ressources/DAY2.txt");
    let mut xpos = 0;
    let mut ypos = 0;
    let cmds: Vec<&str> = input.lines().collect();

    for cmd in cmds {
        let mut split = cmd.split_whitespace();
        let movement = split.next().unwrap();
        let val: i32 = split.next().unwrap().parse().unwrap();
        match movement {
            "down" => {
                ypos += val
            }
            "up" => {
                ypos -= val
            }
            "forward" => {
                xpos += val
            }

            _ => {}
        }

    }
    xpos * ypos
}

pub fn part2() -> i32{
    let input = include_str!("../ressources/DAY2.txt");
    let mut xpos = 0;
    let mut ypos = 0;
    let mut aim = 0;
    let cmds: Vec<&str> = input.lines().collect();

    for cmd in cmds {
        let mut split = cmd.split_whitespace();
        let movement = split.next().unwrap();
        let val: i32 = split.next().unwrap().parse().unwrap();
        match movement {
            "down" => {
                aim += val
            }
            "up" => {
                aim -= val
            }
            "forward" => {
                xpos += val;
                ypos += aim * val
            }

            _ => {}
        }

    }
    xpos * ypos
}