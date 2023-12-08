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

import!(1, 25);

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
    Some(&day8::Day8),
    Some(&day9::Day9),
    Some(&day10::Day10),
    Some(&day11::Day11),
    Some(&day12::Day12),
    Some(&day13::Day13),
    Some(&day14::Day14),
    Some(&day15::Day15),
    Some(&day16::Day16),
    Some(&day17::Day17),
    Some(&day18::Day18),
    Some(&day19::Day19),
    Some(&day20::Day20),
    Some(&day21::Day21),
    Some(&day22::Day22),
    Some(&day23::Day23),
    Some(&day24::Day24),
    Some(&day25::Day25),
];
