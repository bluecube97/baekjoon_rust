use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("error");

    let mut split_input = input.trim().split_whitespace();

    let int_a: i32 = split_input.next().unwrap().parse().expect("error");
    let int_b: i32 = split_input.next().unwrap().parse().expect("error");

    let result: i32 = int_a + int_b;

    println!("{}", result);
}
