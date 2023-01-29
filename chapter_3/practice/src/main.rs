fn main() {
    let fahrenheit = 172.0;
    let celcius = convert_fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit} fahrenheit is {celcius} celcius");

    let c_to_f = 32.2;
    let f = convert_celcius_to_fahrenheit(c_to_f);
    println!("{c_to_f} celcius is {f} fahrenheit");
}

fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn convert_celcius_to_fahrenheit(celcius: f32) -> f32 {
    (celcius * (9.0 / 5.0)) + 32.0
}
