pub fn vcdtypes () {

    
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
   
    // Placeholders = {}, {:#?} and {:.4}
    
    let name = "Alice";
    let age = 21;
           
    println!("My name is {} and i'm {} years old!", name, age);
    println!("-----------------------------------");
    println!("My name is {name} and i'm {age} years old!");
    println!("------------------------------");

  
    let pi = 3.14159265359;
    println!("Pi is approximately: {}", pi);
    println!("Pi to 2 decimal places: {:.2}", pi);
    
}
