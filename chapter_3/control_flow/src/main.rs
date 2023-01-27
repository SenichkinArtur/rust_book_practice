fn main() {
    let var: i32 = 5;

    if var == 5 {
        println!("var is 5");
    } else if var == 6 {
        println!("var is 6")
    } else {
        println!("var is not 5 or 6");
    }

    let var_condition = if var == 5 { 5 } else { 10 };

    println!("var_condition is {var_condition}");
}
