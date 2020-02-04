use std::io;

fn main() {

    println!("Choose one of the options (1/2)");
    let option: u8 = loop {
        println!("1. Celcius to Fahrenheit");
        println!("2. Fahrenheit to Celcius");

        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("Failed to read line");

        match option.trim().parse() {
            Ok(num) => if num == 1 || num == 2 {
                break num;
            } else {
                println!("Enter 1 or 2");
                continue;
            },
            Err(_) => {
                println!("Enter 1 or 2");
                continue;
            },
        };
    };

    println!("Now enter a numeric value to convert");
    let value: f64 = loop {

        let mut value = String::new();

        io::stdin().read_line(&mut value).expect("Failed to read line");

        match value.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Enter a number plz");
                continue;
            }
        };
    };

    if option == 1 {
        println!("{} degree Celcius equals {} degree Fahrenheit", value, celcius_to_fahrenheit(value));
    } else {
        println!("{} degree Fahrenheit equals {} degree Celcius", value, fahrenheit_to_celcius(value));
    }
}

fn celcius_to_fahrenheit(x: f64) -> f64 {
    x * (9./5.) + 32.
}

fn fahrenheit_to_celcius(x: f64) -> f64 {
    (x - 32.) * 5./9.
}
