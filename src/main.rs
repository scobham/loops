use std::{env::temp_dir, io::stdin};

fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!!"); 

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (0..5).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!!!");

    // Convert Temperatures between Fahrenheit and Celsius
    println!("Lets convert a temperature:");
    println!("{}", convert_temp());

    fibonnaci();

}

// Convert Temperatures between Fahrenheit and Celsius
fn convert_temp () -> f32 {
    println!("Please enter your value:");
    let mut temperature = String::new();
    loop {
        stdin()
            .read_line(&mut temperature)
            .expect("Failed to get Temperature");
    
        // let temperature: f32 = temperature.trim().parse().expect("Please type a number!");
        let temperature: f32 = match temperature.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Please enter a proper value.");
    }

    let temperature = temperature.trim().parse().expect("Oops");

    println!("Enter your scale F: Fahrenheit, or C: Celcius.");
    
    let mut scale = String::new();

    stdin()
        .read_line(&mut scale)
        .expect("Failed to get scale!");

    let scale = scale.trim().to_string();

    println!("You entered: {} degrees {}", temperature, scale);

    if (scale == "F") || (scale == "f") {
        println!("Converting to Celcius:");
        return (temperature - 32.0)*5.0/9.0;
    } else if (scale == "C") | (scale == "c") {
        println!("Converting to Fahrenheit:");
        return (temperature * 9.0/5.0) + 32.0;
    } else {
        return 0.0;
    }   
}

// Fibonacci Sequence
fn fibonnaci () {
    println!("Here's the Fibonnaci Sequence.  How far do you want to take it?");

    let mut number = String::new();

    stdin()
        .read_line(&mut number)
        .expect("Couldn't read the number.");

    let number = number.trim().parse().expect("This isn't a number.");

    println!("The Fibonacci Sequesce to the {} is", number);

    let mut first = 0;
    let mut second = 1;
    // let mut counter = 0;
    let mut value = 0;

    print!("{}", first);
    
    for  i in 1..number {
        value = first + second;
        print!(", {}", value);
        first = second;
        second = value;

    }
}