pub struct Day9;

impl super::DayModule for Day9 {
    fn run(&self) -> (usize, usize) {
        let file_content = read_file::read("./inputs/day9.txt");
        let part1_res = utils::time_function!(part1, &file_content);
        let part2_res = utils::time_function!(part2, &file_content);
        (part1_res, part2_res)
    }
}
fn part1(_file_content: &String) -> usize {
    parse_and_iterate_through_with_edit_fn(get_with_one_more_zero_after, _file_content, "Part1")
}
fn part2(_file_content: &String) -> usize {
    parse_and_iterate_through_with_edit_fn(get_with_one_more_zero_before, _file_content, "Part2")
}
fn parse_and_iterate_through_with_edit_fn(edit_fn: fn(Vec<Vec<i128>>) -> i128, _file_content: &String, part_name: &str) -> usize {
    let mut overall_result = 0;
    for arr in parse(_file_content) {
        let diff = get_diff_chain(vec![arr]);
        overall_result += edit_fn(diff);
    }
    println!("{part_name}: {overall_result}");
    overall_result as usize
}
fn get_with_one_more_zero_after(arr: Vec<Vec<i128>>) -> i128 {
    let mut last = 0;
    for run in arr.iter().rev() {
        last = run.iter().last().unwrap_or(&0) + last;
    }
    last
}
fn get_with_one_more_zero_before(arr: Vec<Vec<i128>>) -> i128 {
    let mut last = 0;
    for run in arr.iter().rev() {
        last = run.iter().next().unwrap_or(&0) - last;
    }
    last
}
fn get_diff_chain(mut arr: Vec<Vec<i128>>) -> Vec<Vec<i128>> {
    let difference = get_diff(arr.iter().last().unwrap().clone());
    arr.push(difference.clone());
    match is_finished(difference) {
        true => arr,
        false => get_diff_chain(arr)
    }
}
fn is_finished(arr: Vec<i128>) -> bool {
    arr.iter().all(|&e| e == 0)
}
fn get_diff(arr: Vec<i128>) -> Vec<i128> {
    arr.iter().skip(1).enumerate().map(|(index, value)|{
        let other = arr[index];
        value - other
    }).collect()
}
fn parse(file_content: &String) -> Vec<Vec<i128>> {
    file_content.lines().map(|line| {
        line.split(" ").map(|s|s.parse().unwrap_or(0)).collect()
    }).collect()
}
