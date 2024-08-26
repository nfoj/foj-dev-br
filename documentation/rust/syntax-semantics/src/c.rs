pub fn fmsyntax() {
    // functions
    let sum: u8 = sum(3, 2);
    println!("1 + 2 = {}", sum);
    println!("----------------------------------");

    // dead = let one_ > No use!!
    //let sub: u8 = sub(4, 3);
    //println!("4 - 3 = {}", sub);
    //println!("----------------------------------");

    let myresult: String = get_full_name("Roberto", "Porco");
    println!("Hello {0}", myresult);
    println!("----------------------------------");
}

fn sum(a: u8, b: u8) -> u8 {
    a + b
}

// No use function
#[allow(dead_code)]
fn sub(c: u8, d: u8) -> u8 {
    c - d
}

// &str + &str = String
fn get_full_name(first: &str, last: &str) -> String {
    let full_name: String = format!("{0} {1}", first, last);
    return full_name;
}
