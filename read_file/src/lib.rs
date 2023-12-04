use std::{fs::File, io::Read};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn test() {
    println!("Hallo, Welt!");
}

pub fn read(name: &str) -> String {
    let mut file = File::open(name).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
