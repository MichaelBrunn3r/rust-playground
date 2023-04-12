use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let from_temp = &args[1];

    let unit = from_temp.chars().last().unwrap();
    let from_temp: f32 = from_temp[0..from_temp.len() - 1].parse().unwrap();

    let (converted_temp, to_unit) = match unit {
        'C' => (celsius_to_fahrenheit(from_temp), 'F'),
        'F' => (fahrenheit_to_celsius(from_temp), 'C'),
        _ => panic!(),
    };

    println!("{from_temp}˚{unit} = {converted_temp}˚{to_unit}");
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    temp * 1.8 + 32.0
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.0) / 1.8
}
