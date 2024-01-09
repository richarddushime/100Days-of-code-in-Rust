// // // Declare a variable
// // let a_number;
    
// // // Declare a second variable and bind the value
// // let a_word = "Ten";
    
// // // Bind a value to the first variable
// // a_number = 10;

// // println!("The number is {}.", a_number);
// // println!("The word is {}.", a_word);

// // The `mut` keyword lets the variable be changed
// let mut a_number = 10; 
// println!("The number is {}.", a_number);

// // Change the value of an immutable variable
// a_number = 15;
// println!("Now the number is {}.", a_number);

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }