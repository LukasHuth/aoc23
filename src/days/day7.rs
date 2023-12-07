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
enum HandVariants {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
impl HandVariants {
    fn get_value(&self) -> usize {
        match self {
            Self::HighCard => 1,
            Self::OnePair => 2,
            Self::TwoPair => 3,
            Self::ThreeOfAKind => 4,
            Self::FullHouse => 5,
            Self::FourOfAKind => 6,
            Self::FiveOfAKind => 7,
        }
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
            2 => {
                match set[0] {
                    4 => Self::FourOfAKind,
                    3 => Self::FullHouse,
                    _ => panic!("{:?}", set),
                }
            },
            3 => {
                match set[0] {
                    3 => Self::ThreeOfAKind,
                    2 => Self::TwoPair,
                    _ => panic!("{:?}", set),
                }
            }
            4 => Self::OnePair,
            _ => panic!("{:?}", set),
        }
    }
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
        let mut sorted_chars = card.cards.map(|c|get_card_value(c));
        sorted_chars.sort();
        // println!("{:?}", sorted_chars);
        let mut value = card.variant.get_value() << 20;
        // println!("{:#b}", value);
        value += sorted_chars[4] << 16;
        // println!("{:#32x}", value);
        ( value + (sorted_chars[3] << 12) + (sorted_chars[2] << 8) + (sorted_chars[1] << 4) + sorted_chars[0], *card)
    }).collect();
    card_values.sort_by(|a, b|a.0.cmp(&b.0));
    for cv in card_values.iter().skip(900) {
        println!("{:?}", cv.1);
    }
    let result: usize = card_values.iter().enumerate().map(|(i, (_, card))| {
        (i+1) * card.bid
    }).sum();
    println!("Part1: {result}");
    result
}
fn get_card_value(c: char) -> usize {
    match c {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Why? ({})", c)
    }
}
fn part2(file_content: &String) -> usize {
    0
}
struct Part1Parse;
impl utils::TransformFunction<Card> for Part1Parse {
    fn transform(&self, line: &str) -> Card {
        let splited: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
        let cards_str = splited[0].clone();
        let cards = cards_str.chars().collect::<Vec<_>>().try_into().unwrap();
        let bid: usize = splited[1].parse().unwrap();
        Card { cards , variant: HandVariants::get_variant(cards_str.as_str()), bid }
    }
}