const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    shadowing();
}

//Shadowing

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    let x:u32 = 0b0110_0011;
}

//Data Types :i32, :f32, :bool, :char, :tup

struct integer_types {
    a:u128,
    b:u64,
    c:u32,
    d:u16,
    e:u8,
    u:i128,
    w:i64,
    x:i32,
    y:i16,
    z:i8,
}

pub fn types(){
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    let t = true;
    let f: bool = false; // with explicit type annotation
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    let tup: (i32, f64, u8) = (500, 6.4, 1);

}





