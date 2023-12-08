pub struct Day20;

impl super::DayModule for Day20 {
    fn run(&self) -> (usize, usize) {
        let file_content = read_file::read("./inputs/day20.txt");
        let part1_res = utils::time_function!(part1, &file_content);
        let part2_res = utils::time_function!(part2, &file_content);
        (part1_res, part2_res)
    }
}
fn part1(_file_content: &String) -> usize {
    0
}
fn part2(_file_content: &String) -> usize {
    0
}
