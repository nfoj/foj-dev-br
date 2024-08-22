2. Syntax and Semantics

    a. Variables, constants and data types;

        - print, comments and placeholders;
        - variables and mutability;
        - data types;

    b. Control flow constructs;
    
    c. Functions and method syntax;
    
    d. Pattern matching and destructuring.

    - Variables

        Variable are reserved memory spaces for storing information;

    
        ```
            
            let name = "Alice";
            println!("What is your name: {}", name);
            println!("------------------------------");

            let num = 24;
            println!("What number did you choose? {}", num);
            println!("------------------------------");

            let letter = 'a';
            println!("Whats is the firts letter that comes to your mind? {}", letter);
            println!("------------------------------");
   
            let ok = true; 
            println!("One plus one equals two? {}", ok);
            println!("------------------------------");

        ```


    - Variables and Mutability

        Add 'mut' after let to change the value of a variable.

        - let mut name = "Jonas";
        - let mut age = 40;
        - let mut city = "Mexico";
        - let mut pet = "Cat"; 
        

> [!WARNING]  
> Erro!


        ```

            let name = "Alice";
            println!("What is your name: {}", name);
            
            // Erro
            name = "Jonas";
            pritnln!("What is your name: {}", name);

        ```
        
> [!WARNING]  
> Correct!


        ```
            // Add: mut
            let mut name = "Alice";
            println!("What is your name: {}", name);
            
            // Correct
            name = "Jonas";
            pritnln!("What is your name: {}", name);

        ```

    - Constant

        - A constant cannot be changed once it's been assigned a value;
        - Constants should be declared in uppercase and have an explicit type.


        ```

            // const
            const POINTS = 3;
            println!("{}", POINTS);


            // Erro!
            POINTS = 4;
            println!("{}", POINTS);

        ```
    

    - Shadowing

        Shadowing a variable by declaring a new one with the same name;

        ```

            // shadowing
            let food = "bread";
            println!("{}", food);
            
            let food = "milk";
            println!("{}", food);

            let food = "pizza";
            println!("{}", food);

        ```

    - Scope

        When a block of code is enclosed within curly braces {}, it enters a new scope, which is nested within the main program's scope.

        ```

        // scope
        let x = 1;
        println!("{}", x);

        {
            
            let x = 2;
            println!("{}", x);

        }
    
        println!("{}", x);

        ```


    - Types

        Primitives: u, i, f, char, bool;
    
        u: positive values.

        ```

            // u8 = 0 - 255
            println!("u8 ({} - {})", std::u8::MIN, std::u8::MAX);
                        
            let number_u8: u8 =  255;
            println!("{}", number_u8);


            // u16 = 0 - 65.535
            println!("u16 ({} - {})", std::u16::MIN, std::u16::MAX);    
 
            let number_u16: u16 =  255;
            println!("{}", number_u16);


            // u32 = 0 - 4.294.967.295
            println!("u32 ({} - {})", std::u32::MIN, std::u32::MAX);
 
            let number_u32: u32 =  255;
            println!("{}", number_u32);


            // u64 = 0 - 18.446.744.073.709.551.615 
            println!("u64 ({} - {})", std::u64::MIN, std::u64::MAX);
 
            let number_u64: u64 =  255;
            println!("{}", number_u64);


            // u128 = 0 -  
            println!("u128 ({} - {})", std::u128::MIN, std::u128::MAX);
 
            let number_u128: u128 =  255;
            println!("{}", number_u128);


        ```


> [!TIP]
> Use the command: 'std::type::MIN or MAX' to print the size of a variable:
> println!("u8 ({} - ())", std::u8::MIN, std::u8::MAX);
> println!("u16 ({} - {})", std::u16::MIN, std::u16::MAX);



        i: positve e negative values.

        ```

            // i8 = -128 - 127
            println!("i8 ({} - {})", std::i8::MIN, std::i8::MAX);
 
            let number_i8: i8 =  127;
            println!("{}", number_i8);


            // i16 = 0 - 65535
            println!("i16 ({} - {})", std::i16::MIN, std::i16::MAX);
 
            let number_i16: i16 =  127;
            println!("{}", number_i16);


            // i32 = -2147483648 - 2147483647
            println!("i32 ({} - {})", std::i32::MIN, std::i32::MAX);
 
            let number_i32: i32 =  127;
            println!("{}", number_i32);


            // i64 = -9223372036854775808 - 922337203685477580 
            println!("i64 ({} - {})", std::i64::MIN, std::i64::MAX);
 
            let number_i64: i64 =  127;
            println!("{}", number_i64);


            // i128 =  
            println!("i128 ({} - {})", std::i128::MIN, std::i128::MAX);
 
            let number_i128: i128 =  127;
            println!("{}", number_i128);

    
        ```

> [!IMPORTANT]  
> Rust offers additional data types: usize and isize. Refer to the documentation for details.
> These types automatically adjust to the system's architecture (32-bit or 64-bit).



        f: positive and negative decimal numbers.


        ```

            // f32 = -340282350000000000000000000000000000000 - 34028235000000000000000000000000000000
            println!("f32 ({} - {})", std::f32::MIN, std::f32::MAX);
 
            let number_f32: f32 =  179.76;
            println!("{}", number_f32);


            // f64 = -17976931348623157000 ... - 17976931348623157000 ...
            println!("f64 ({} - {})", std::f64::MIN, std::f64::MAX);
 
            let number_f64: f64 =  179.76;
            println!("{}", number_f64);


        ```

        char: holds only one Unicode character (use 'value').


        ```

            let character: char = 'a';
            println!("{}", character);
            
            let symbol: char = ' '
            println!("{}", symbol);

        ```

        str: text!

        ```

        let first_name: &str = "Roberto";
        println!("{}", first_name);
        

        ```
       
        bool: true or false;

        
        ```

            let checked: bool = true;
            println!("The data was checked? {}", checked);
            

        ```


        Compound: tuples and array


        tuples: allow you to group values of various data types together.


        ```

            let data_types: (u8, char, f32, i64) = (2, 'a', 5.4, 28);
            println!("{:?}", data_types);
    

            let person = ("Gregor", 64, 1.82);
            let (x, y, z) = person;
            println!("My name is {x}, i'm {y} years old and my height is {z}");


        ```

        array: a statically sized list of a single data type


        ```

            let list: [u8; 3] = [1, 5, 9];
            println!("{:?}", list);

            let list_icons: [char; 6] = [``, ``, ``, ``, ``, ``];
            println!("{:?}", list_icons);


        ```


    C. Functions and Modules


    - Functions

        Reusable code blocks that perform specific tasks, essential for organizing and structuring code, making it more readable, efficient, and maintainable.

        ```

            // main
            fn main () {

            let result = sum (1, 2);    
            println!("{}",result);
            
            
            }
                        

            fn sum (a: u8, b: u8) -> u8 {
                a + b
            }


            // No use function!
            #[allow(dead_code)]
            fn subtration (x: i8, y:i8) -> i8 {
                x - y
            }    
    
           
        ```

    
