use std::ops::Range;

pub struct Day5;
impl super::DayModule for Day5 {
    fn run(&self) -> (usize, usize) {
        let file_content = read_file::read("./inputs/day5.txt");
        let part1_res = utils::time_function!(part1, &file_content);
        let part2_res = utils::time_function!(part2, &file_content);
        (part1_res, part2_res)
    }
}
#[derive(Debug)]
pub(crate) struct Map {
    pub(crate) target_start: usize,
    pub(crate) source: Range<usize>,
}
#[derive(Debug)]
pub(crate) struct MapCollection {
    pub(crate) maps: Vec<Map>,
}
impl MapCollection {
    pub(crate) fn new() -> Self {
        Self { maps: vec![] }
    }
    pub(crate) fn push(&mut self, map: Map) {
        self.maps.push(map);
    }
    pub(crate) fn get_new_value(&self, val: usize) -> usize {
        for map in self.maps.iter() {
            if map.source.contains(&val) {
                return map.target_start + (val  - map.source.start);
            }
        }
        val
    }
    pub(crate) fn get_new_ranges(&self, val: Range<usize>) -> Vec<Range<usize>> {
        for map in self.maps.iter() {
            if let Some(result) = Self::generate_range_arr_if_possible(map, &val) {
                return result;
            }
        }
        vec![val]
    }
    fn generate_range_arr_if_possible(map: &Map, val: &Range<usize>) -> Option<Vec<Range<usize>>> {
        if map.source.contains(&val.start) {
            if map.source.contains(&(val.end-1)) {
                let start_offset = val.start - map.source.start;
                let start = map.target_start+start_offset;
                return Some(vec![start..(start+val.len())]);
            }
            let length = map.source.end - val.start;
            let start_offset = val.start - map.source.start;
            let start = map.target_start + start_offset;
            return Some(vec![start..(start+length), (val.start+length)..(val.end)]); //map.target_start + (val  - map.source.start);
        }
        if map.source.contains(&(val.end-1)) {
            let length = val.end - map.source.start;
            let start = map.target_start;
            return Some(vec![start..(start+length), val.start..(val.end-length)]);
        }
        None
    }
}
fn part1(file_content: &String) -> usize {
    let mut map: Vec<MapCollection> = Vec::new();
    let mut file_parts = file_content.split("\n\n");
    let seeds: Vec<usize> = file_parts.next().unwrap()[7..].split(" ").map(|s|s.parse().unwrap()).collect();
    for part in file_parts {
        let mut map_1 = MapCollection::new();
        for line in part.lines().skip(1) {
            let numbers: Vec<usize> = line.split(" ").map(|s|s.parse().unwrap()).collect();
            map_1.push(Map { target_start: numbers[0], source: numbers[1]..(numbers[1]+numbers[2])});
        }
        map.push(map_1);
    }
    let mut current = seeds;
    for i in 0..=6 {
        current = current.iter().map(|&num| {
            map[i].get_new_value(num)
        }).collect()
    }
    let result = current.iter().min().unwrap();
    println!("Part1: {result}");
    *result
}
fn part2(file_content: &String) -> usize {
    let mut map: Vec<MapCollection> = Vec::new();
    let mut file_parts = file_content.split("\n\n");
    let seeds: Vec<usize> = file_parts.next().unwrap()[7..].split(" ").map(|s|s.parse().unwrap()).collect();
    let seed_range: Vec<Range<usize>> = seeds.chunks(2).map(|parts| parts[0]..(parts[0]+parts[1])).collect();
    // println!("{:?}", seed_range);
    for part in file_parts {
        let mut map_1 = MapCollection::new();
        for line in part.lines().skip(1) {
            let numbers: Vec<usize> = line.split(" ").map(|s|s.parse().unwrap()).collect();
            map_1.push(Map { target_start: numbers[0], source: numbers[1]..(numbers[1]+numbers[2])});
        }
        map.push(map_1);
    }
    let mut current = seed_range;
    println!("{:?} length: {}", current, get_length(current.clone()));
    for i in 0..=6 {
        let mut ranges: Vec<Range<usize>> = Vec::new();
        current.iter().for_each(|num| {
            ranges.append(&mut map[i].get_new_ranges(num.clone()));
        });
        println!("{:?} length: {}", ranges, get_length(ranges.clone()));
        current = ranges;
    }
    let mut smallest_range = current[0].clone();
    for i in 1..current.len() {
        if current[i].start < smallest_range.start { smallest_range = current[i].clone(); }
    }
    let result = smallest_range.start;
    println!("Part1: {result}");
    result
}
pub(crate) fn get_length(arr: Vec<Range<usize>>) -> usize {
    arr.iter().map(|range|range.len()).sum()
}
