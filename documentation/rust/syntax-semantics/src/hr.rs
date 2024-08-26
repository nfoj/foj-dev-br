use std::io;

pub fn hackerrank() {
    // Array
    let alice: [i16; 3] = [1, 2, 3];
    let bob: [i16; 3] = [4, 5, 6];

    // print - Alice
    for i in alice {
        println!("{}", i);
    }
    println!("-----------------------------------------");

    // Print - Bob
    for i in bob {
        println!("{}", i);
    }
    println!("-----------------------------------------");

    // Sum - Alice
    let mut salice = 0;
    for i in alice {
        salice += i;
    }
    println!("{}", salice);
    println!("-----------------------------------------");

    // sum - Bob
    let mut sbob = 0;
    for i in bob {
        sbob += i;
    }
    println!("{}", sbob);
    println!("-----------------------------------------");

    // Size Array: Alice and Bob
    if alice.len() != bob.len() {
        println!("{}", false);
    } else {
        println!("{}", true);
    }
    println!("-----------------------------------------");

    println!("-----------------------------------------");
    println!("-----------------------------------------");
    println!("-----------------------------------------");
    println!("-----------------------------------------");
    println!("-----------------------------------------");
}
