pub struct Day6;
impl super::DayModule for Day6 {
    fn run(&self) -> (usize, usize) {
        let file_content = read_file::read("./inputs/day6.txt");
        let part1_res = utils::time_function!(part1, &file_content);
        let part2_res = utils::time_function!(part2, &file_content);
        (part1_res, part2_res)
    }
}
fn boat_distance(hold_time: usize, total_time: usize) -> usize {
    let distance_with_full_speed = (total_time - hold_time) * hold_time;
    distance_with_full_speed
}
fn find_hold_time(total_time: usize, distance: usize) -> usize {
    let mut res = 0;
    for time in 0..=total_time {
        let result = boat_distance(time, total_time);
        if result > distance { res += 1; }
        if result < distance && res != 0 { break; }
    }
    res
}
fn part1(file_content: &String) -> usize {
    let lines: Vec<&str> = file_content.lines().collect();
    let hold_times_as_str: Vec<&str> = lines[0][6..].trim().split(" ").filter(|s|!s.is_empty()).collect();
    let hold_times: Vec<usize> = hold_times_as_str.iter().map(|s|s.parse().unwrap()).collect();
    let distances_as_str: Vec<&str> = lines[1][9..].trim().split(" ").filter(|s|!s.is_empty()).collect();
    let distances: Vec<usize> = distances_as_str.iter().map(|s|s.parse().unwrap()).collect();
    let mut result = 1;
    for i in 0..hold_times.len() {
        result *= find_hold_time(hold_times[i], distances[i]);
    }
    println!("{:?}", result);
    result
}
fn part2(file_content: &String) -> usize {
    let lines: Vec<&str> = file_content.lines().collect();
    let hold_time_as_str: String = lines[0][6..].trim().replace(" ", "");
    let hold_time: usize = hold_time_as_str.parse().unwrap();
    let distance_as_str: String = lines[1][9..].trim().replace(" ", "");
    let distance: usize = distance_as_str.parse().unwrap();
    let result = find_hold_time(hold_time, distance);
    println!("{:?}", result);
    result
}
