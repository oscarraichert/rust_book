use std::io;

fn main() {
    println!("Temperature converter!\n");
    println!("Select the type of conversion: \n1- °C to °F \n2- °F to °C");

    let mut conversion = String::new();

    io::stdin()
        .read_line(&mut conversion)
        .expect("Invalid input.");

    println!("Type the temperature to be converted: ");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Invalid input.");

    let temperature: i32 = temperature.trim().parse().expect("Not a number!");

    match conversion.trim() {
        "1" => celsius_to_fahrenheit(temperature),
        "2" => fahrenheit_to_celsius(temperature),
        _ => ()
    }
}

fn fahrenheit_to_celsius(temperature: i32) {
    println!("{temperature}°F is {}°C", (temperature - 32) * 5/9);
}

fn celsius_to_fahrenheit(temperature: i32) {
    println!("{}°C is {}°F", temperature, (temperature * 9/5) + 32);
}
