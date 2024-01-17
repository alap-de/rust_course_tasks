fn main() {
    let a: f64 = match read_user_input("Enter first number: ".to_string()).parse() {
        Ok(a) => a,
        Err(_) => panic!("Invalid input"),
    };

    let operation_str: String = read_user_input("Enter operation: ".to_string());

    let b: f64 = match read_user_input("Enter second number: ".to_string()).parse() {
        Ok(b) => b,
        Err(_) => panic!("Invalid input"),
    };

    let operation = match operation_str.as_str() {
        "+" => Operaion::Add(a, b),
        "-" => Operaion::Substract(a, b),
        "*" => Operaion::Multiply(a, b),
        "/" => Operaion::Divide(a, b),
        _ => panic!("Invalid operation"),
    };

    println!("Result: {}", calculate(operation));
}

enum Operaion {
    Add(f64, f64),
    Substract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operaion) -> f64 {
    match operation {
        Operaion::Add(a, b) => a + b,
        Operaion::Substract(a, b) => a - b,
        Operaion::Multiply(a, b) => a * b,
        Operaion::Divide(a, b) => a / b,
    }
}

fn read_user_input(context: String) -> String {
    println!("{}", context);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");
    input.trim().to_string()
}
