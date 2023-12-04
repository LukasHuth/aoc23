pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub trait DayModule {
    fn run(&self) -> (usize, usize);
}
pub const DAYS: [Option<&dyn DayModule>; 25] = [
    Some(&day1::Day1),
    Some(&day2::Day2),
    Some(&day3::Day3),
    Some(&day4::Day4),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];
