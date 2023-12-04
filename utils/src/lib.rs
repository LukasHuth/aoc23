pub trait TransformFunction<T> {
    fn transform(&self, line: &str) -> T;
}
pub fn transform_lines<T>(str: String, func: &dyn TransformFunction<T>) -> Vec<T> {
    str.lines().map(|line| {func.transform(line)}).collect()
}
#[macro_export]
macro_rules! time_function{
    ($function_name:ident,$arg:expr) => {
        {
        {
        let now = std::time::Instant::now();
        let res = $function_name($arg);
        println!("Elapsed: {:?}", now.elapsed());
        res
        }
        }
    };
}
