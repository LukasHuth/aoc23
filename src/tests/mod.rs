use crate::days;
use paste;
use seq_macro::seq;
fn get_function(num: usize) -> &'static Option<&'static dyn days::DayModule>{
    days::DAYS.get(num-1).unwrap()
}
const RESULTS: [[usize; 2]; 25] = [
    [54940, 54208],
    [1734, 70387],
    [540131, 86879020],
    [21558, 10425665],
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
