pub struct Day9;

impl super::DayModule for Day9 {
    fn run(&self) -> (usize, usize) {
        let file_content = read_file::read("./inputs/day9.txt");
        let part1_res = utils::time_function!(part1, &file_content);
        let part2_res = utils::time_function!(part2, &file_content);
        (part1_res, part2_res)
    }
}
fn part1(file_content: &String) -> usize {
    0
}
fn part2(file_content: &String) -> usize {
    0
}
