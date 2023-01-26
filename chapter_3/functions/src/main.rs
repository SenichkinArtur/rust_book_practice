fn main() {
    println!("Hello world");

    hello_function("hello, ".to_owned(), 5);
}

fn hello_function(label: String, number: i32) {
    println!("{label}{number}");
}
