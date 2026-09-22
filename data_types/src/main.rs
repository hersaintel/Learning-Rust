fn main() {
//Rust is a staticallt typed language
//Scalar types 1. Integers
    Length	                Signed	Unsigned
    8-bit	                i8	    u8
    16-bit	                i16	    u16
    32-bit	                i32	    u32
    64-bit	                i64	    u64
    128-bit	                i128	u128
    Architecture-dependent	isize	usize

signed variant can store numbers from −(2n − 1) to 2n − 1 − 1, where n is the number of bits used to store an integer.(128 to 127)
unsigned variants can store numbers from 0 to 2n − 1.(0 to 255)

//Floating-Point types
    f32	32-bit floating-point
    f64	64-bit floating-point

    let x = 2.0; //f64
    let y: f32 = 3.0; //f32

//Integer and float literals are in decimal by default, but you can use an "e" or " E" to indicate scientific notation:
    let x = 1u8;
    let y = 2i32;
    let z: f64 = 3.0;
    let i = -1; //signed -1
    let u = 4_u8;

//Numeric Operations
    //addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.3;
    let truncated = -5 / 3; //Results in -1

    // Remainder
    let remainder = 43 % 5;

//The Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation

// The Character type
    let c = 'z';
    let z: char = 'Z'; //with explicit type annotation
    let heart_eyed_cat = '😻';

//The Compound Types
    //Tuples Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //To get the individual value of a tuple:(destructuring)
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    prinln!("The value of y is {y}");

    // We can also access a tuple element directly by using a period(.)
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;


// Arrays
    //Arrays are a compound type that can hold multiple values of the same type.
    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    //You write an array's type using square brackets, a semicolon and the data types of each element separated by commas.
    let a: [i32; 5] = [1,2,3,4,5];

    //You can also initialize an array to contain the same value of each element by specifying the initial value, followed by a semi-colon, and then the length of the array in square brackets.
    let a = [3; 5];

    //Array Element Access
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];

}
