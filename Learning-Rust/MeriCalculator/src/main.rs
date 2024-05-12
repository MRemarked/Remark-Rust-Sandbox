use std::io::{Write, stdout, stdin};

fn main() {
    let mut input1 = String::new();
    loop {
        print!("Whoa^5, type in the numbers! ");
        stdout().flush().unwrap();

        stdin().read_line(&mut input1).unwrap();
        
        let args: Vec<&str> = input1.split_whitespace().collect();

        if args.len() != 3 {
            println!("Not enough arguments");
            continue;
        }

        let result = match calculate(args) {
            Ok(r) => r,
            Err(e) => {
                println!("{e}");
                continue;
            },
        };

        println!("{}", result);
        input1.clear();
    }
}

// Box<dyn std::error::Error> is BAD
fn calculate(args: Vec<&str>) -> Result<f64, Box<dyn std::error::Error>> {
    let num1 = args[0].parse::<f64>()?;
    let num2 = args[2].parse::<f64>()?;

    Ok(match args[1] {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => return Err("Invalid Operator".into()),
    })
}