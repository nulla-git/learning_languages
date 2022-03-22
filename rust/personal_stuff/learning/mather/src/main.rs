use std::io; 

fn main() {
    let mut number_to_add = String::new();
    
    println!("what do you wanna add to two");

    io::stdin().read_line(&mut number_to_add);

    let number_to_add: u32  = number_to_add.trim().parse().unwrap(); 

    let mut result = 2 + number_to_add;

        println!("result is {}", result);

}

