mod a;

fn main() {
    println!("Variables, constants and data types:");
    a::vcdtypes();

    let result = sum_two(1, 2);
    println!("{}", result);
}

fn sum_two(c: u8, b: u8) -> u8 {
    c + b
}
