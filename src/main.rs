pub mod days;
mod tests;
fn main() {
    let args = std::env::args();
    if args.len() < 2 {
        println!("please specify at least one day to execute!");
    }
    let arguments: Vec<_> = args.collect();
    let arguments_without_program_name: Vec<_> = arguments[1..arguments.len()].iter().collect();
    for arg in arguments_without_program_name {
        if let Ok(day) = arg.parse() {
            execute(day);
        }
    }
}
fn execute(num: usize) {
    use days::DAYS;
    println!("Day {num}:");
    println!("-------------------");
    if let Some(day) = DAYS.get(num-1) {
        day.unwrap().run();
    }
    println!("-------------------");
}
