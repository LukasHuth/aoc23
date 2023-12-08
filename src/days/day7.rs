use std::collections::HashMap;

pub struct Day7;

impl super::DayModule for Day7 {
    fn run(&self) -> (usize, usize) {
        let file_content = read_file::read("./inputs/day7.txt");
        let part1_res = utils::time_function!(part1, &file_content);
        let part2_res = utils::time_function!(part2, &file_content);
        (part1_res, part2_res)
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(usize)]
enum HandVariants {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(Clone, Copy, PartialEq, Debug)]
struct Card {
    cards: [char; 5],
    variant: HandVariants,
    bid: usize,
}
fn part1(file_content: &String) -> usize {
    let cards: Vec<Card> = utils::transform_lines(file_content.clone(), &Part1Parse);
    let mut card_values: Vec<(usize, Card)> = cards.iter().map(|card| {
        let chars = card.cards.map(|c|get_card_value_part1(c));
        ( (card.variant.get_value() << 20) + (chars[0] << 16) + (chars[1] << 12) + (chars[2] << 8) + (chars[3] << 4) + chars[4], *card)
    }).collect();
    card_values.sort_by(|a, b|a.0.cmp(&b.0));
    let result: usize = card_values.iter().enumerate().map(|(i, (_, card))| {
        (i+1) * card.bid
    }).sum();
    println!("Part1: {result}");
    result
}
fn part2(file_content: &String) -> usize {
    let cards: Vec<Card> = utils::transform_lines(file_content.clone(), &Part2Parse);
    let mut card_values: Vec<(usize, Card)> = cards.iter().map(|card| {
        let chars = card.cards.map(|c|get_card_value_part2(c));
        ( (card.variant.get_value() << 20) + (chars[0] << 16)+ (chars[1] << 12) + (chars[2] << 8) + (chars[3] << 4) + chars[4], *card)
    }).collect();
    card_values.sort_by(|a, b|a.0.cmp(&b.0));
    let result: usize = card_values.iter().enumerate().map(|(i, (_, card))| {
        (i+1) * card.bid
    }).sum();
    println!("Part2: {result}");
    result
}
fn get_cards_and_bid(line: &str) -> ([char;5], String, usize) {
    let splited: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
    let cards_str = splited[0].clone();
    let cards = cards_str.chars().collect::<Vec<_>>().try_into().unwrap();
    let bid: usize = splited[1].parse().unwrap();
    (cards, cards_str, bid)
}
struct Part1Parse;
impl utils::TransformFunction<Card> for Part1Parse {
    fn transform(&self, line: &str) -> Card {
        let (cards, cards_str, bid) = get_cards_and_bid(line);
        Card { cards , variant: HandVariants::get_variant(cards_str.as_str()), bid }
    }
}
struct Part2Parse;
impl utils::TransformFunction<Card> for Part2Parse {
    fn transform(&self, line: &str) -> Card {
        let mut best_variant: Option<HandVariants> = None;
        for letter in ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"] {
            let splited: Vec<String> = line.replace("J", letter).split(" ").map(|s| s.to_string()).collect();
            let hv = HandVariants::get_variant(&splited[0].clone());
            if let Some(bv) = best_variant{
                if hv.get_value() > bv.get_value() { best_variant = Some(hv); }
            } else { best_variant = Some(hv); }
        }
        let ( cards, _, bid ) = get_cards_and_bid(line);
        Card { cards , variant: best_variant.unwrap(), bid }
    }
}
fn get_card_value_part1(c: char) -> usize {
    const MAP: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    MAP.iter().position(|&e| e == c).unwrap_or(0)
}
fn get_card_value_part2(c: char) -> usize {
    const MAP: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
    MAP.iter().position(|&e| e == c).unwrap_or(0) + 1
}
impl HandVariants {
    fn get_value(&self) -> usize {
        *self as usize + 1
    }
    fn get_variant(str: &str) -> Self {
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in str.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        let mut set: Vec<usize> = map.iter().map(|(_, &co)| co).collect();
        set.sort();
        set.reverse();
        match set.len() {
            1 => Self::FiveOfAKind,
            5 => Self::HighCard,
            2 => match set[0] {
                4 => Self::FourOfAKind,
                3 => Self::FullHouse,
                _ => panic!("{:?}", set),
            },
            3 => match set[0] {
                3 => Self::ThreeOfAKind,
                2 => Self::TwoPair,
                _ => panic!("{:?}", set),
            },
            4 => Self::OnePair,
            _ => panic!("{:?}", set),
        }
    }
}
