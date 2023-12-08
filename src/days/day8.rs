use std::collections::HashMap;

pub struct Day8;

const START_NAME: &str = "AAA";
const END_NAME: &str = "ZZZ";

impl super::DayModule for Day8 {
    fn run(&self) -> (usize, usize) {
        let file_content = read_file::read("./inputs/day8.txt");
        let part1_res = utils::time_function!(part1, &file_content);
        let part2_res = utils::time_function!(part2, &file_content);
        (part1_res, part2_res)
    }
}
fn part1(file_content: &String) -> usize {
    let (directions, map) = get_dirs_and_map(&file_content);
    let mut directions = directions.iter().cycle();
    let first = START_NAME;
    let mut current = match directions.next().unwrap() {
        Direction::Right => map[&first].right,
        Direction::Left => map[&first].left,
    };
    let mut steps = 1;
    while current != END_NAME {
        current = match directions.next().unwrap() {
            Direction::Left => map[current].left,
            Direction::Right => map[current].right,
        };
        steps += 1;
    }
    let result = steps;
    println!("Part1: {result}");
    result
}
fn part2(_file_content: &String) -> usize {
    let (directions, map) = get_dirs_and_map(&_file_content);
    let directions_immut = directions.iter().cycle();
    let mut directions = directions_immut.clone();
    let first_step = directions.next().unwrap();
    let mut current: Vec<&str> = map.iter().filter(|s|
        s.0.chars().last() == Some('A')
    ).map(|s| match first_step { Direction::Left => s.1.left, Direction::Right => s.1.right }).collect();
    let mut result = 1;
    for i in 0..current.len() {
        let mut steps = 1;
        directions = directions_immut.clone();
        directions.next();
        while !current[i].ends_with("Z") {
            let next_step = directions.next().unwrap_or(&Direction::Left);
            current[i] = match next_step {
                Direction::Left =>map[current[i]].left,
                Direction::Right =>map[current[i]].right,
            };
            steps += 1;
        }
        result = lcm(result, steps);
    }
    println!("Part1: {result}");
    result
}
enum Direction {
    Left,
    Right,
}
struct Piece<'a> {
    left: &'a str,
    right: &'a str,
}
fn get_dirs_and_map<'a>(_file_content: &'a String) -> (Vec<Direction>, HashMap<&'a str, Piece>) {
    let directions: Vec<Direction> = _file_content.lines().next().unwrap()
        .chars().map(|c|
            match c { 'L' => Direction::Left, 'R' => Direction::Right, _ => panic!("Why? ({})", c)}
        ).collect();
    let map: HashMap<&str, Piece> = _file_content.split("\n\n").last().unwrap().lines().map(|line| {
        let name_and_children: Vec<&str> = line.split(" = ").collect();
        let splited: Vec<&str> = name_and_children[1][1..=8].split(", ").collect();
        (name_and_children[0], Piece { left: splited[0], right: splited[1] })
    }).collect();
    (directions, map)
}
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}
