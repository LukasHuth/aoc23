use crate::days;
use paste;
use seq_macro::seq;
#[allow(unused)]
fn get_function(num: usize) -> &'static Option<&'static dyn days::DayModule>{
    days::DAYS.get(num-1).unwrap()
}
#[allow(unused)]
const RESULTS: [[usize; 2]; 25] = [
    [54940, 54208],
    [1734, 70387],
    [540131, 86879020],
    [21558, 10425665],
    [389056265, 137516820],
    [293046, 35150181],
    [253638586, 253253225],
    [14257, 16187743689077],
    [1772145754, 867],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
    [0, 0],
];
macro_rules! generate_day_test {
    ($start: expr, $end: expr) => {
        seq! { N in $start..=$end {
        paste::item! {
        #[test]
        fn [< test_day_ N >] () {
        if let Some(func) = get_function(N) {
        let result = func.run();
        assert_eq!(RESULTS[N-1][0], result.0);
        assert_eq!(RESULTS[N-1][1], result.1);
        }
        assert_eq!(1, 1);
        }
        }
        }}
    };
}

generate_day_test!(1, 25);


#[test]
fn map_test_overrun() {
    use days::day5::{Map, MapCollection, get_length};
    let map_1 = Map { target_start: 1, source: 0..15 };
    let map_collection = MapCollection { maps: vec![map_1] };
    let res = map_collection.get_new_ranges(0..20);
    let expected_result = vec![1..16, 15..20];
    assert_eq!(res, expected_result);
    assert_eq!(get_length(res), get_length(expected_result));
}
#[test]
fn map_test_underrun() {
    use days::day5::{Map, MapCollection, get_length};
    let map_1 = Map { target_start: 10, source: 5..20 };
    let map_collection = MapCollection { maps: vec![map_1] };
    let res = map_collection.get_new_ranges(0..20);
    let expected_result = vec![10..25, 0..5];
    assert_eq!(res, expected_result);
    assert_eq!(get_length(res), get_length(expected_result));
}
#[test]
fn map_test_including() {
    use days::day5::{Map, MapCollection, get_length};
    let map_1 = Map { target_start: 0, source: 5..20 };
    let map_collection = MapCollection { maps: vec![map_1] };
    let res = map_collection.get_new_ranges(7..17);
    let expected_result = vec![2..12];
    assert_eq!(res, expected_result);
    assert_eq!(get_length(res), get_length(expected_result));
}
#[test]
fn map_test_equals() {
    use days::day5::{Map, MapCollection, get_length};
    let map_1 = Map { target_start: 0, source: 5..15 };
    let map_collection = MapCollection { maps: vec![map_1] };
    let res = map_collection.get_new_ranges(5..15);
    let expected_result = vec![0..10];
    assert_eq!(res, expected_result);
    assert_eq!(get_length(res), get_length(expected_result));
}
#[test]
fn map_test_length() {
    use days::day5::get_length;
    let result = get_length(vec![0..5, 10..15]);
    let expected_result = 10;
    assert_eq!(result, expected_result);
}
