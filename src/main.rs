use std::env::{args, Args};

fn main() {

    println!("*************************************");
    println!("*************************************");
    println!("INFO TO RUN THIS APPLICATION!!!\n");
    println!("Run the following command:");
    println!("$ cargo run arg1 arg2 arg3");
    println!("(EXAMPLE: $ cargo run 1 + 1)");
    println!("*************************************");
    println!("*************************************");
    println!("ALTERNATIVELY:");
    println!("To run the binary and test your application run the following command:");
    println!("$ target/release/<name_project> arg1 arg2 arg3");
    println!("(EXAMPLE: $ target/release/calcolatrice-rust 1 + 1)");
    println!("*************************************");
    println!("*************************************");

    let mut args: Args = args();
    // output della posizione del binario compilato e gli argomenti da riga di comando passati al programma
    println!("{:?}", args);
    println!("*************************************");

    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    // println!("{} {} {}", first, operator, second);

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);
  
    println!("{}", output(first_number, operator, second_number, result));

    fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
        format!("{} {} {} = {}", first_number, operator, second_number, result)
      }

    fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
        match operator {
          '+' => first_number + second_number,
          '-' => first_number - second_number,
          '/' => first_number / second_number,
          '*' | 'X' | 'x' => first_number * second_number,
          _ => panic!("Invalid operator used."),
        }
      }

}
