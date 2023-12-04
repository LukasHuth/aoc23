pub struct Day2;
impl super::DayModule for Day2 {
    fn run(&self) -> (usize, usize) {
        let file_input = read_file::read("./inputs/day2.txt");
        let part1_result = utils::time_function!(part1, &file_input);
        let part2_result = utils::time_function!(part2, &file_input);
        (part1_result, part2_result)
    }
}
#[derive(Debug, Clone, Copy)]
struct Draw {
    red: u32, blue: u32, green: u32,
}
#[derive(Debug, Clone)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}
fn part1(file_input: &String) -> usize {
    let games: Vec<Game> = utils::transform_lines(file_input.clone(), &Part1TransformStruct);
    let games = filter_not_possible_games(games, 14, 12, 13);
    let result: u32 = games.iter().map(|games| games.id).sum();
    // println!("{:?}", games);
    println!("Part1: {result}");
    result as usize
}
fn part2(file_input: &String) -> usize {
    let games: Vec<Game> = utils::transform_lines(file_input.clone(), &Part1TransformStruct);
    let games: Vec<Game> = games.iter().map(|game| {
        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;
        for draw in game.draws.clone() {
            if max_green < draw.green { max_green = draw.green; }
            if max_red < draw.red { max_red = draw.red; }
            if max_blue < draw.blue { max_blue = draw.blue; }
        }
        Game { id: game.id, draws: vec![ Draw { red: max_red, blue: max_blue, green: max_green }]}
    }).collect();
    let game_power: Vec<u32> = games.iter().map(|game| game.draws[0].red * game.draws[0].blue * game.draws[0].green ).collect();
    let result: u32 = game_power.iter().sum();
    // println!("{:?}", games);
    println!("Part2: {result}");
    result as usize
}
fn filter_not_possible_games(games: Vec<Game>, max_blue: u32, max_red: u32, max_green: u32) -> Vec<Game> {
    games.iter().filter(|game|
        game.draws.iter().all(|draw|
            draw.blue <= max_blue && draw.red <= max_red && draw.green <= max_green
        )
    ).map(|g|g.clone()).collect()
}
struct Part1TransformStruct;
impl utils::TransformFunction<Game> for Part1TransformStruct {
    fn transform(&self, line: &str) -> Game {
        let line = line[5..].to_string();
        let id: u32 = line.split(":").next().unwrap().parse().unwrap();
        let draws = line.split(": ").last().unwrap();
        let draws: Vec<Draw> = draws.split("; ").map(|draw| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for color in draw.split(", ") {
                let value: u32 = color.trim().split(" ").next().unwrap().parse().unwrap();
                let color = color.trim().split(" ").last().unwrap();
                match color {
                    "blue" => blue = value,
                    "red" => red = value,
                    "green" => green = value,
                    _ => panic!("unknown color: '{color}'"),
                }
            }
            Draw { red, blue, green }
        }).collect();
        // println!("{}", line);
        Game { id, draws }
    }
}
