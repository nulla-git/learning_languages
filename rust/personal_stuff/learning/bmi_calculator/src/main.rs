use std::io;


fn main() {

    println!("this program uses metric units");
    println!("please input your height in centimeters.");
    println!("also, if you input letters, I am not feeding your dog");

    let mut height = String::new();

    io::stdin().read_line(&mut height);

    let mut height: u32 = height.trim().parse().unwrap();

    println!("ok, height is {}", height);

    println!("-------------");

    println!("ok, now what is your weight in kilograms.");

    let mut weight = String::new();

    io::stdin().read_line(&mut weight);

    let mut weight: u32 = weight.trim().parse().unwrap();

    println!("ok, your weight is {} kilograms",weight); 

    let mut bmi_index: u32 = 0;

    let mut bmi_index = height / (weight * weight;

    println!("ok, your bmi is {}",bmi_index)

}
