use seq_macro::seq;
use paste::paste;

macro_rules! import {
    ($start:expr, $end: expr) => {
        seq!{ N in $start..=$end {
            paste! {
                pub mod [< day N >];
            }
        }}
    };
}

import!(1, 7);

pub trait DayModule {
    fn run(&self) -> (usize, usize);
}
pub const DAYS: [Option<&dyn DayModule>; 25] = [
    Some(&day1::Day1),
    Some(&day2::Day2),
    Some(&day3::Day3),
    Some(&day4::Day4),
    Some(&day5::Day5),
    Some(&day6::Day6),
    Some(&day7::Day7),
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
