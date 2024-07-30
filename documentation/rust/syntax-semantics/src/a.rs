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
    println!("------------------------------");

    let num = 24;
    println!("What number did you choose? {}", num);
    println!("------------------------------");

    let letter =  'a';
    println!("Whats is the firts letter that comes to your mind? {}", letter);
    println!("------------------------------");
   
    let ok = true; 
    println!("One plus one equals two? {}", ok);
    println!("------------------------------");


    // Variables and Mutability
    
    // ERROR!
    //let firstname = "Alice";
    //firstname = "Jonas"; 
    //println!("{}", firstname);
    //println!("------------------------------");

    // CORRECT
    let mut firstname = "Alice"; // Add = mut
    firstname = "Jonas"; 
    println!("{}", firstname);
    println!("------------------------------");


    // Constant
    const TWO: i8 = 2;
    println!("{}", TWO);
    println!("------------------------------");
    
    
    // Shadowing
    let food = "bread";
    println!("{}", food);


    let food = "milk";
    println!("{}", food);


    let food = "pizza";
    println!("{}", food);
    println!("------------------------------");
    
    // Scope
    let pet = "cat";
    println!("{}", pet);
    {
        let pet = "dog";
        println!("{}", pet);
    }
    println!("{}", pet);
    println!("------------------------------");


    
    // Type Primitive
    
    // u = positive values

    //u8
    println!("u8 ({} - {})", std::u8::MIN, std::u8::MAX);
    println!("------------------------------");

    //u16
    println!("u16 ({} - {})", std::u16::MIN, std::u16::MAX);
    println!("------------------------------");

    //u32
    println!("u32 ({} - {})", std::u32::MIN, std::u32::MAX);
    println!("------------------------------");

    //u64
    println!("u64 ({} - {})", std::u64::MIN, std::u64::MAX);
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


    // f = floating number values
    
    //f32
    println!("f32 ({} - {})", std::f32::MIN, std::f32::MAX);
    println!("------------------------------");  
    
    //f64
    println!("f64 ({} - {})", std::f64::MIN, std::f64::MAX);
    println!("------------------------------");

  
    // Char = a character
 
    let a: char = 'a';
    println!("{}", a);
    println!("------------------------------");

   
    



}
