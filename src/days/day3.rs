use std::collections::HashSet;

pub struct Day3;

const INPUT_HEIGHT: usize = 140;
const INPUT_WIDTH: usize = 140;

impl super::DayModule for Day3 {
    fn run(&self) -> (usize, usize) {
        let file_input = read_file::read("./inputs/day3.txt");
        let part1_result = utils::time_function!(part1, &file_input);
        let part2_result = utils::time_function!(part2, &file_input);
        (part1_result, part2_result)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Number {
    number: usize,
    start: [usize;2],
    len: u8,
}
impl Number {
    fn hits(&self, x: usize, y: usize) -> bool {
        let range = self.start[0]..(self.start[0]+self.len as usize);
        self.start[1] == y && range.contains(&x)
    }
}
fn part1(file_input: &str) -> usize {
    let numbers = get_numbers(file_input);
    let mut set: HashSet<Number> = HashSet::new();
    for (y, line) in file_input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) || c == '.' { continue; }
            let adjacents = get_adjacent(x, y);
            for adj in adjacents {
                for num in numbers.clone() {
                    if num.hits(adj[0], adj[1]) {
                        set.insert(num);
                    }
                }
            }
        }
    }
    let result: usize = set.iter().map(|num|num.number).sum();
    println!("Part1: {result}");
    result
}
fn part2(file_input: &str) -> usize {
    let numbers = get_numbers(file_input);
    let mut res_set = Vec::new();
    for (y, line) in file_input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut set: HashSet<Number> = HashSet::new();
            if c.is_digit(10) || c == '.' { continue; }
            for adj in get_adjacent(x, y) {
                for num in numbers.clone() {
                    if num.hits(adj[0], adj[1]) {
                        set.insert(num);
                    }
                }
            }
            if set.len() == 2 {
                res_set.push(set.iter().next().unwrap().number * set.iter().last().unwrap().number);
            }
        }
    }
    let result: usize = res_set.iter().sum();
    println!("Part1: {result}");
    result
}
fn get_adjacent(x: usize, y: usize) -> Vec<[usize;2]> {
    let mut map = Vec::new();
    if x > 0 {
        map.push([x-1, y]);
        if y > 0 { map.push([x-1, y-1]) }
        if y < INPUT_HEIGHT { map.push([x-1, y+1]) }
    }
    if x < INPUT_WIDTH {
        map.push([x+1, y]);
        if y > 0 { map.push([x+1, y-1]) }
        if y < INPUT_HEIGHT { map.push([x+1, y+1]) }
    }
    if y > 0 { map.push([x, y-1]) }
    if y < INPUT_HEIGHT { map.push([x, y+1]) }
    map
}
fn get_numbers(file_input: &str) -> Vec<Number> {
    let mut numbers = Vec::new();
    for (y, line) in file_input.lines().enumerate() {
        let mut chars = line.chars().into_iter();
        let ( mut x, mut start, mut len, mut number ) = (0, 0, 0, 0);
        let mut last_number = false;
        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                if !last_number {
                    last_number = true;
                    start = x;
                }
                number *= 10;
                number += c.to_digit(10).unwrap() as usize;
                len += 1;
            }
            else {
                if last_number {
                    numbers.push(Number { number , start: [start, y], len });
                    last_number = false;
                    number = 0;
                    len = 0;
                }
            }
            x+=1;
        }
        if last_number {
            numbers.push(Number { number , start: [start, y], len })
        }
    }
    numbers
}
