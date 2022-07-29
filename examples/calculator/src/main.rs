fn main() {
    let mut args = std::env::args().skip(1);

    let left: f64 = args.next().unwrap().parse().unwrap();
    let operator: char = args.next().unwrap().chars().next().unwrap();
    let right: f64 = args.next().unwrap().parse().unwrap();

    let result = match operator {
        '+' => left + right,
        '-' => left - right,
        '*' | 'x' => left * right,
        '/' => left / right,
        _ => panic!("Unknown operator: {}", operator),
    };

    println!("{} {} {} = {}", left, operator, right, result);
}
