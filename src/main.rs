use std::io;

enum Choices {
    END,
    CONT,
}

impl Choices {
    fn from_string(s: String) -> Option<Choices> {
        match s.to_uppercase().as_str() {
            "E" => Some(Choices::END),
            "C" => Some(Choices::CONT),
            _ => None,
        }
    }
}

fn numbers(n: i8) {
    match n {
        2 | 4 | 6 | 8 | 10 => println!("{} is even", n),
        1 | 3 | 5 | 7 | 9 => println!("{} is odd", n),
        _ => println!("{} falls out of the bounds of 1-10", n),
    }
}

fn main() {
    println!("Welcome !!");
    loop {
        let mut input = String::new();
        println!(
            "Enter your choice:
            - e to exit
            - c to continue
            "
        );
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let trimmed = input.trim().to_string();
        match Choices::from_string(trimmed) {
            Some(Choices::END) => {
                println!("Ending...");
                break;
            }
            Some(Choices::CONT) => {
                let mut number = String::new();
                println!("Enter a number from 1-10:");
                io::stdin()
                    .read_line(&mut number)
                    .expect("Failed to read input");
                let formatted = number.trim().parse::<i8>();
                match formatted {
                    Ok(f) => numbers(f),
                    Err(e) => println!("Error parsing: {}", e),
                }
            }
            None => {
                println!("{} does not satisfy options 'e' or 'c'", input.trim());
            }
        }
    }
}
