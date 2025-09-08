use std::io;

fn main(){
    println!("Enter the first no: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed To read Line");
    let number1:f64 = input.trim().parse().expect("Please enter a valid No");
    println!("The no is : {number1}");


    print!("Enter the second no: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line ");
    let number2:f64 = input.trim().parse().expect("Please enter a valid no ");
    println!("the second no is : {number2}");

    let sum = number1+number2;
    let diff = number1 - number2;
    let multi = number1*number2;
    
    println!("Sum of the no is : {}",sum);
    println!("Difference of the no is : {}",diff);
    println!("Multiplication of the no is : {}",multi);
    

}