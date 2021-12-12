use itertools::Itertools;
use aoc2021::lines_from_file;

pub fn part1() -> usize {
    let vec = lines_from_file("src/ressources/DAY1.txt");

    vec
        .iter()
        .tuple_windows()
        .filter(|(cur, prev)| cur < prev)
        .count()
}


pub fn part2() -> usize {
    let vec = lines_from_file("src/ressources/DAY1.txt");

    vec
        .iter()
        .tuple_windows()
        .filter(|(one, _, _, four)| one < four)
        .count()
}