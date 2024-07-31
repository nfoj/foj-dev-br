# Rust!

1. Introduction

    a. What is Rust?
        
        Rust is a modern systems programming language designed for performance, reliability, and concurrency. It emphasizes safety and control over memory management,
        making it suitable for writing efficient and secure software. Developed by Mozilla Research, Rust has gained popularity for its innovative features such as 
        ownership and borrowing system, which helps prevent common programming errors like null pointer dereferencing and data races. 

        - Memory Safety;
        - Concurrency;
        - Performance;
        - Reability;
        - Ecosystem.

    b. Why use Rust?
        
        Rust is a system programming language that aims to provide memory safety, concurrency, and performance with a focus on zero cost abstractions. It Rust is 
        appreciated for the solutions it provides to common programming language issues. Its emphasis on safety, speed, and support for concurrent programming, as 
        well as its robust type system, are just a few reasons why developers choose Rust.
    
    c. Memory Safety and Zero-Cost Abstractions;
    
       Rust is a high-performance system programming language known for its safety features, including memory safety without garbage collection. It supports concurrent
       programming with thread safety and offers zero-cost abstractions for efficient, high-level coding.

    d. Environnment Setup;

       - Installing Rust and Cargo: https://www.rust-lang.org/tools/install
       - IDEs and Rust Toolchains: Visual Studio, Neovim, Jetbrains, Zed;
       - Rust REPL (Rust Playground): https://play.rust-lang.org/?version=stable&mode=debug&edition=2021


2. Syntax and Semantics

    a. Variables, constants and data types;

        - print, comments and placeholders;
        - variables and mutability;
        - data types;
        - functions;

    b. Control flow constructs;
    
    c. Functions and method syntax;
    
    d. Pattern matching and destructuring.


    - Comments

        Line comments: start with // and extend to the end of the line.
        Block comments: enclosed within /* and */, spanning multiple lines.

        ```
            // line;

            /* block; */

        ```


    - Print

        print!: print text without line breacks; 
        println!: print text with line breaks;


        ```
            print!("Hello, Maria!");
            println!("Hello, Carlos!");
        
        ```


    - Break line

        \n: should be used to insert a line break.
        

        ```

            println!("What is you name? \nMy name is Maria!");
            println!("1 + 1 = \n2"); 

        ```


    - Placeholders

        {}: inserts the value of the next argument without any formatting;
        {:.n}: formats numbers with "n" decimal places.


        ```

            let a = "Alice";

            // {}
            println!("My name is {}", name);
            // My name is Alice
    

            // {...}
            println!("My name is {name}");
            // My name is Alice


            let pi = 3.14159;

            // {}
            println!("Pi is {}", pi);
            // Pi is 3.14159

            
            // {:.n}
            pritnln!("Pi is {:.2}", pi);
            // Pi is 3.14

        ```
    
    > [!IMPORTANT]
    > For more information access: <https://doc.rust-lang.org/std/fmt/>


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


    
        ```

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

        
        bool: true or false;

        
        ```

            let  


        ```






