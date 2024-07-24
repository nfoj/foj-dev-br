pub fn vcdtypes() {
    
    // Comments

    // 1 Line

    /*
     * 2 or more lines
     */


    // Print
    println!("-----------------------------------");
    print!("Hello, Maria! ... ");
    println!("Hello, Carlos!");
    println!("-----------------------------------");


    // \n = break line

    println!("Hello, Maria! \nGood morning!");
    print!("1 + 1 = \n2");
    println!("\n-----------------------------------");


    // Placeholders = {}, {#} and {:.4}

    let name = "Alice";
    let age = 21;

    println!("-----------------------------------");
    println!("My name is {name} and i'm {age} years old!");
    println!("------------------------------");

    let pi = 3.14159265359;
    println!("Pi is approximately: {}", pi);
    println!("Pi to 2 decimal places: {:.2}", pi);
    println!("------------------------------");

    
    // Variables
    
    let name = "Alice";
    println!("What is your name: {}", name);

    let num = 24;
    println!("What number did you choose? {}", num);

    let letter =  'a';
    println!("Whats is the firts letter that comes to your mind? {}", letter);
    
    let ok = true; 
    println!("One plus one equals two? {}", ok);
    println!("------------------------------");


    // Type Primitive
    // u = positive values

    //u8 = 0 - 255
    let number_u8: u8 = 255;
    println!("u8 (0 - 255) = {}", number_u8);
    println!("------------------------------");

    //u16 =  0 - 65.535
    let number_u16: u16 = 65_535;
    println!("u8 (0 - 65.535) = {}", number_u16);
    println!("------------------------------");

    //u32 = 0 - 4.294.967.295
    let number_u32: u32 = 4_294_967_295;
    println!("u32 (0 - 4.294.967.295) = {}", number_u32);
    println!("------------------------------");

    //u64 = 0 - 18.446.744.073.709.551.615
    let number_u64: u64 = 18_446_744_073_709_551_615;
    println!("u64 (0 - 18.446.744.073.709.551.615){}", number_u64);
    println!("------------------------------");

    // i = positive e negative values

    //i8 = -128 - 127
    println!("i8 ({} - {})", std::i8::MIN, std::i8::MAX);
    println!("------------------------------");

    //i16 =  -32.768 - 32.767
    println!("i16 ({} - {})", std::i16::MIN, std::i16::MAX);
    println!("------------------------------");

    //i32 = 0 - 4.294.967.295
    println!("i32 ({} - {})", std::i32::MIN, std::i32::MAX);
    println!("------------------------------");

    //i64 = 0 - 18.446.744.073.709.551.615
    println!("u64 ({} - {})", std::i64::MIN, std::i64::MAX);
    println!("------------------------------");


    // 







    let a: char = 'a';
    println!("{}", a);

    println!("case!");











}
