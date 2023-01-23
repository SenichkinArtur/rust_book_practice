use std::io;

fn main() {
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;

    let mut x: i32 = 5;

    println!("mut x is {x}");

    x = 6;

    println!("updated x is {x}");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("y in the inner scope is {y}");
    }

    println!("y in the outer scope is {y}");

    let spaces = "   ";

    let spaces = spaces.len();

    println!("spaces.len(): {spaces}");
}
