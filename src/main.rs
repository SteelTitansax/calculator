use std::env::{args,Args};
fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number: f32 = first.parse::<f32>().unwrap();
    let second_number: f32 = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);
    println!("{:?}", result);
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        'x' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator"),
    }
}