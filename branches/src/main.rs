use std::io;

fn main() {
    println!("Pick a game: temps, fib, or carol");

    loop {
        let mut game = String::new();
        io::stdin()
            .read_line(&mut game)
            .expect("Failed to read line");

        if game.trim() == "temps" {
            println!("You picked temps");
            temps();
            break;
        } else if game.trim() == "fib" {
            println!("You picked fib");
            fib();
            break;
        } else if game.trim() == "carol" {
            println!("You picked carol");
            carol();
            break;
        } else {
            println!("You must pick temps, fib or carol");
        }
    }
}

fn temps() {
    println!("Enter a fahrenheit temperature to convert to celsius.");

    loop {
        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        println!("You entered: {fahrenheit}");

        let fahrenheit: f32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The temp in celsius is: {}", (fahrenheit - 32.0) / 1.8);
        break;
    }
}

fn fib() {
    println!("Enter a which fibonacci number you want to see.");

    loop {
        let mut fibonacci = String::new();

        io::stdin()
            .read_line(&mut fibonacci)
            .expect("Failed to read line");

        println!("You entered: {fibonacci}");

        let mut fibonacci: u32 = match fibonacci.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut fib_vector: Vec<u32> = [0, 1].to_vec();

        for number in 2..fibonacci {
            let n_us = usize::try_from(number).unwrap();
            let n_fib = fib_vector[n_us - 1] + fib_vector[n_us - 2];
            fib_vector.push(n_fib);
        }

        println!("{:?}", fib_vector);

        let fib_us = usize::try_from(fibonacci).unwrap();

        fibonacci = fib_vector[fib_us - 1];

        println!("The fibonacci number is: {}", fibonacci);
        break;
    }
}

fn carol() {
    // On the twelfth day of Christmas,
    // my true love gave to me
    // Twelve drummers drumming,
    // Eleven pipers piping,
    // Ten lords a-leaping,
    // Nine ladies dancing,
    // Eight maids a-milking,
    // Seven swans a-swimming,
    // Six geese a-laying,
    // Five golden rings,
    // Four calling birds,
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree!

    const NTHS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const GIFTS: [&str; 11] = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves",
        // "a partridge in a pear tree",
    ];

    for number in 0..12 {
        let n_us = usize::try_from(number).unwrap();
        println!("\n");
        println!("On the {} night of christmas", NTHS[n_us]);
        println!("my true love gave to me");
        for num in 11 - number..11 {
            let num_us = usize::try_from(num).unwrap();
            println!("{}", GIFTS[num_us]);
        }

        if number == 0 {
            println!("A partridge in a pear tree");
        } else {
            println!("And a partridge in a pear tree");
        }
        println!("\n");
    }
}
