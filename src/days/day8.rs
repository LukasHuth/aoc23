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
    let directions: Vec<usize> = file_content.lines().next().unwrap()
        .chars().map(|c|
            match c { 'L' => 0, 'R' => 1, _ => panic!("Why? ({})", c)}
        ).collect();
    let mut directions = directions.iter().map(|&e|e).cycle();
    let map: HashMap<&str, [&str; 2]> = file_content.split("\n\n").last().unwrap().lines().map(|line| {
        let first_splited: Vec<&str> = line.split(" = ").collect();
        let splited: Vec<&str> = first_splited[1][1..9].split(", ").collect();
        (first_splited[0], [splited[0], splited[1]])
    }).collect();
    let first = "AAA";
    let mut current = map[&first][directions.next().unwrap()];
    let mut steps = 1;
    while current != "ZZZ" {
        current = map[current][directions.next().unwrap()];
        steps += 1;
    }
    // println!("{:?}", map);
    let result = steps;
    println!("Part1: {result}");
    result
}
#[allow(unreachable_code)]
fn part2(_file_content: &String) -> usize {
    println!("I guess this run indefinitely?");
    return 0;
    let directions: Vec<usize> = _file_content.lines().next().unwrap()
        .chars().map(|c|
            match c { 'L' => 0, 'R' => 1, _ => panic!("Why? ({})", c)}
        ).collect();
    let mut directions = directions.iter().map(|&e|e).cycle();
    let map: HashMap<&str, [&str; 2]> = _file_content.split("\n\n").last().unwrap().lines().map(|line| {
        let first_splited: Vec<&str> = line.split(" = ").collect();
        let splited: Vec<&str> = first_splited[1][1..9].split(", ").collect();
        (first_splited[0], [splited[0], splited[1]])
    }).collect();
    // let first = "AAA";
    let first_step = directions.next().unwrap();
    let mut current: Vec<&str> = map.iter().filter(|s|s.0.chars().last().unwrap() == 'A').map(|s|s.1[first_step]).collect();
    let mut steps = 1;
    while !is_valid(&current) {
        let next_step = directions.next().unwrap();
        for i in 0..current.len() {
            current[i] = map[current[i]][next_step];
        }
        steps += 1;
    }
    // println!("{:?}", map);
    let result = steps;
    println!("Part1: {result}");
    result
}
fn is_valid(vec: &Vec<&str>) -> bool {
    vec.iter().all(|e|e.ends_with("Z"))
}
