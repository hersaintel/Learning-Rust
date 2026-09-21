fn main () {
    println!("Hello, World!");

//This is how to write comments in Rust

//Variables and mutability
let x = 6;
println!("The value of x is: {x}");
let x = 5;
println!("The value of x is: {x}");

//shadowing
let x = 5;
let x = x + 1;

{
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");

}
println!("The value of x is: {x}");

//Constants
const TWENTY_FOUR_HOURS_IN_SECONDS: u32 = 60 * 60 * 24;
println!("There is {TWENTY_FOUR_HOURS_IN_SECONDS} seconds in twenty four hours")

}
