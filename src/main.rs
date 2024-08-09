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

fn main() {
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
                println!("In the loop \n");
            }
            None => {
                println!("{} does not satisfy options 'e' or 'c'", input.trim());
            }
        }
    }
}
