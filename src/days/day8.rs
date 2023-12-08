use std::collections::HashMap;

pub struct Day8;

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
    let first = "AAA";
    let mut current = map[&first][*directions.next().unwrap()];
    let mut steps = 1;
    while current != "ZZZ" {
        current = map[current][*directions.next().unwrap()];
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
    ).map(|s|s.1[*first_step]).collect();
    let mut result = 1;
    for i in 0..current.len() {
        let mut steps = 1;
        directions = directions_immut.clone();
        directions.next();
        while !current[i].ends_with("Z") {
            let next_step = directions.next().unwrap();
            current[i] = map[current[i]][*next_step];
            steps += 1;
        }
        result = lcm(result, steps);
    }
    println!("Part1: {result}");
    result
}
fn get_dirs_and_map<'a>(_file_content: &'a String) -> (Vec<usize>, HashMap<&'a str, [&'a str;2]>) {
    let directions: Vec<usize> = _file_content.lines().next().unwrap()
        .chars().map(|c|
            match c { 'L' => 0, 'R' => 1, _ => panic!("Why? ({})", c)}
        ).collect();
    let map: HashMap<&str, [&str; 2]> = _file_content.split("\n\n").last().unwrap().lines().map(|line| {
        let first_splited: Vec<&str> = line.split(" = ").collect();
        let splited: Vec<&str> = first_splited[1][1..9].split(", ").collect();
        (first_splited[0], [splited[0], splited[1]])
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
