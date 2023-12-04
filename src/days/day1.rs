pub struct Day1;
const TO_REPLACE: [&str;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
impl super::DayModule for Day1 {
    fn run(&self) -> (usize, usize) {
        let file_input: String = read_file::read("./inputs/day1.txt");
        let part1_res = utils::time_function!(part1, &file_input);
        let part2_res = utils::time_function!(part2, &file_input);
        (part1_res, part2_res)
    }
}
fn part1(file_input: &String) -> usize {
    let result = find_numbers(file_input);
    println!("Part1: {result}");
    result as usize
}
fn part2(file_input: &String) -> usize {
    let file_input = utils::transform_lines(file_input.clone(), &Part2TransformStruct);
    let file_input = file_input.join("\n");
    let result = find_numbers(&file_input);
    println!("Part2: {result}");
    result as usize
}
fn find_numbers(file_input: &String) -> u32 {
    let line_array_of_numbers = utils::transform_lines(file_input.clone(), &GeneralTransformStruct);
    let numbers: Vec<u32> = line_array_of_numbers.iter().map(|nums| {
        ( nums.first().unwrap_or(&0) * 10 + nums.last().unwrap_or(&0) ) as u32
    }).collect();
    let result: u32 = numbers.iter().sum();
    result
}
struct GeneralTransformStruct;
impl utils::TransformFunction<Vec<u8>> for GeneralTransformStruct {
    fn transform(&self, line: &str) -> Vec<u8> {
        let numbers = line.chars().filter(|c|c.is_digit(10)).map(|c|c.to_digit(10).unwrap() as u8).collect();
        numbers
    }
}
struct Part2TransformStruct;
impl utils::TransformFunction<String> for Part2TransformStruct {
    fn transform(&self, line: &str) -> String {
        let mut line = line.to_string();
        for start in 0..(line.len()) {
            let mut found = false;
            for len in 3..=5 {
                if start+len > line.len() { continue; }
                let new_str = line[start..(start+len)].to_string();
                if TO_REPLACE.contains(&new_str.as_str()) {
                    let mut mut_str = String::with_capacity(line.len()-new_str.len()+1);
                    mut_str.push_str(&line[..start]);
                    mut_str.push_str(&format!("{}", TO_REPLACE.iter().position(|&x| x == new_str.as_str()).unwrap_or(0)));
                    mut_str.push_str(&line[(start+len)..(line.len())]);
                    line = mut_str;
                    found = true;
                }
            }
            if found {break;}
        }
        for start in 3..(line.len()) {
            let start = line.len() - start;
            let mut found = false;
            for len in 3..=5 {
                if start+len > line.len() { continue; }
                let new_str = line[start..(start+len)].to_string();
                if TO_REPLACE.contains(&new_str.as_str()) {
                    let mut mut_str = String::with_capacity(line.len()-new_str.len()+1);
                    mut_str.push_str(&line[..start]);
                    mut_str.push_str(&format!("{}", TO_REPLACE.iter().position(|&x| x == new_str.as_str()).unwrap_or(0)));
                    mut_str.push_str(&line[(start+len)..(line.len())]);
                    line = mut_str;
                    found = true;
                }
            }
            if found {break;}
        }
        line
    }
}
