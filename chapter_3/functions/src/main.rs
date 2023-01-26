fn main() {
    println!("Hello world");

    hello_function("hello, ".to_owned(), 5);
    variable_expresssion();

    let b = function_return_default();
    let c = function_return();

    println!("function_return_default {b}");
    println!("function_return {c}");

    let sum_of_numbers = sum_numbers(3, 4);

    println!("Sum of numbers is {sum_of_numbers}");
}

fn hello_function(label: String, number: i32) {
    println!("{label}{number}");
}

fn variable_expresssion() {
    let x = {
        let y = 5;
        y + 1
    };

    println!("x is {x}");
}

fn function_return_default() -> i32 {
    5
}

fn function_return() -> i32 {
    return 10;
}

fn sum_numbers(a: i32, b: i32) -> i32 {
    a + b
}
