fn main() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

fn celsius_to_fahrenheit(celsius_temp: f64) -> f64 {
    let fahrenheit_temp: f64 = (1.8 * celsius_temp) + 32 as f64;
    println!("The computed temp is {} degrees", fahrenheit_temp);
    return fahrenheit_temp;
}