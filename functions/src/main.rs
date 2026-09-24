fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labelled_measurements(5, 'h');



// Parameters - they are like variables that we pass into a function to use inside the function.
fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labelled_measurements(value: u32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements and Expressions
    //Statements are instructions that perform some action and don not return a value.
    //Expressions evaluate to a resultant value.
//let y = 6; //This is a statement
    let x = {
        let z = 4;
        z + 3
    }; //this is an expression
    println!("The value of x is {x}");

//Return Values - functions return values by default, but we can also specify that they don't have a return value by using the -> keyword.
fn five() -> i32 {
    5
}

    let x = five();
    println!("The value of x is: {x}");

fn plus_one(x: i32) -> i32 {
    x+1
}
    let x = plus_one(5);
    println!("The value of x is: {x}");
//Function Parameters - functions can have parameters, which are specified after the function name in parentheses. 

}