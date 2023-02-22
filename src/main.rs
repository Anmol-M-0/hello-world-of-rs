use std::io;

fn main() {
    println!("Hello, world!");
    // let mut input = String::new();
    // let buf = io::stdin().read_line(&mut input).unwrap();
    let (mut input, buf) = read_line();
    input = input.trim().to_string();
    println!("You entered: {}, {}", input, buf);
}


fn read_line() -> (String, usize){
    let mut input: String = String::new();
    let buf: usize = io::stdin().read_line(&mut input).unwrap();
    return (input, buf)
}