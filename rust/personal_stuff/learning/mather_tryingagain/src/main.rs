use std::io;

fn main() {
    
    let mut input = String::new();

        println!("what is added to 2?");

    io::stdin().read_line(&mut input);

    let mut input: u32 = input.trim().parse().unwrap();

    println!("the result is {}",input + 2)
}
