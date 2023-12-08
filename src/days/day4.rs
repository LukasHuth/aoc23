use std::collections::LinkedList;

pub struct Day4;
impl super::DayModule for Day4  {
    fn run(&self) -> (usize, usize){
        let file_input = read_file::read("./inputs/day4.txt");
        let part1_res = utils::time_function!(part1, &file_input);
        let part2_res = utils::time_function!(part2, &file_input);
        (part1_res, part2_res)
    }
}
#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    owning: Vec<u32>,
}
fn part1(file_input: &String) -> usize {
    let mut cards = utils::transform_lines(file_input.clone(), &Part1Generator);
    cards.iter_mut().for_each(|s| s.owning = s.owning.iter().filter(|e|s.winning.contains(e)).map(|e|*e).collect());
    let winning_cards_amounts: Vec<usize> = cards.iter().map(|card|card.owning.len()).collect();
    let winning_cards_values: Vec<usize> = winning_cards_amounts.iter().map(|amount| if *amount == 0 { 0 } else { 1 << (amount - 1)}).collect();
    let result: usize = winning_cards_values.iter().sum();
    println!("Part1: {result}");
    result
}
fn part2(file_input: &str) -> usize {
    let mut total_scratch_cards = 0;
    let mut copies: LinkedList<usize> = LinkedList::new();
    copies.push_back(0);
    for line in file_input.lines() {
        let copy_amount = copies.pop_front().unwrap_or(0) + 1;
        total_scratch_cards += copy_amount;
        let mut card = get_card(line);
        card.owning = card.owning.iter().filter(|e|card.winning.contains(e)).map(|&e|e).collect();
        let new_copy_amount = card.owning.len();
        let missing_copy_entries = new_copy_amount as i64 - copies.len() as i64;
        if missing_copy_entries > 0 {
            for _ in 0..missing_copy_entries {
                copies.push_back(0);

            }
        }
        for _ in 0..copy_amount {
            let mut current_index = 0;
            for node in copies.iter_mut() {
                if current_index == new_copy_amount {
                    break; 
                }
                *node += 1;
                current_index += 1;
            }
        }
    }
    println!("Part2: {total_scratch_cards}");
    total_scratch_cards
}
struct Part1Generator;
impl utils::TransformFunction<Card> for Part1Generator {
    fn transform(&self, line: &str) -> Card {
        get_card(line)
    }
}
fn get_card(line: &str) -> Card {
    let last_part = line[5..].trim().split(":").last().unwrap().trim();
    let winning = last_part.split("|").next().unwrap().trim();
    let winning = winning.split(" ").filter(|s|!s.is_empty()).map(|s|s.parse().unwrap()).collect();
    let owning = last_part.split("|").last().unwrap().trim();
    let owning = owning.split(" ").filter(|s|!s.is_empty()).map(|s|s.parse().unwrap()).collect();
    Card { winning, owning }
}
