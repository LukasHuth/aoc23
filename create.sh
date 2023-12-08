#!/bin/bash

# Specify the range of days for which files need to be created
start_day=9
end_day=25

# Loop through the range and create files
for ((day=start_day; day<=end_day; day++))
do
    filename="src/days/day${day}.rs"
    rm "$filename"
    touch "$filename"
    
    # Populate the file with the provided text, replacing Day8 with the current day
    echo "pub struct Day${day};" > "$filename"
    echo "" >> "$filename"
    echo "impl super::DayModule for Day${day} {" >> "$filename"
    echo "    fn run(&self) -> (usize, usize) {" >> "$filename"
    echo "        let file_content = read_file::read(\"./inputs/day${day}.txt\");" >> "$filename"
    echo "        let part1_res = utils::time_function!(part1, &file_content);" >> "$filename"
    echo "        let part2_res = utils::time_function!(part2, &file_content);" >> "$filename"
    echo "        (part1_res, part2_res)" >> "$filename"
    echo "    }" >> "$filename"
    echo "}" >> "$filename"
    echo "fn part1(_file_content: &String) -> usize {" >> "$filename"
    echo "    0" >> "$filename"
    echo "}" >> "$filename"
    echo "fn part2(_file_content: &String) -> usize {" >> "$filename"
    echo "    0" >> "$filename"
    echo "}" >> "$filename"

    echo "File created: $filename"
done

