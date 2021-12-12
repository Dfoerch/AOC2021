pub fn part1() -> usize{

    let mut gamma : String = "".to_owned();
    let mut epsilon : String = "".to_owned();
    let input: Vec<&str> = include_str!("../ressources/DAY3.txt").lines().collect();
    let mut ones = 0;
    let mut zero = 0;

    for i in 0..12 {
        zero = 0;
        ones = 0;
      for line in &input {
        let char = line.chars().nth(i).unwrap();
            if char == '0' {
                zero += 1
            } else {
                ones += 1
            }
      }

      if zero > ones {
          gamma.push_str("0");
          epsilon.push_str("1")
      } else {
          gamma.push_str("1");
          epsilon.push_str("0")
      }
    }

    usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap()
}

pub fn part2() -> usize{
    0
}