use std::io;
use std::io::Write;

fn main() {

    loop {

        println!("[F] for Fahrenheit-to-Celsius\n[C] for Celsius-to-Fahrenheit\n[Q] to exit");

        let mut option = String::new();

        io::stdin().read_line(&mut option).expect("failure reading option");

        let from_fahrenheit_to_celsius = match option.trim().to_uppercase().as_str() {
            "F" => true,
            "C" => false,
            "Q" => std::process::exit(0),
            _ => {
                println!("invalid option");
                continue;
            }
        };

        print!("Temperature? ");
        // without the "flush" the string is not printed out
        io::stdout().flush().unwrap();

        let mut num = String::new();

        io::stdin().read_line(&mut num).expect("failure reading option");

        let input_temperature: f64 = match num.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("invalid temperature");
                continue;
            }
        };

        if from_fahrenheit_to_celsius {
            // {:..1} formats the number with 1 decimal point
            println!("{:.1} °C", fahrenheit_to_celsius(input_temperature));
        } else {
            println!("{:.1} °F", celsius_to_fahrenheit(input_temperature));
        }
    }
}

fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * 5.0/9.0
}

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * 9.0/5.0 + 32.0
}
