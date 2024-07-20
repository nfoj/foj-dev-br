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
        
    - Placeholders

        {}: inserts the value of the next argument without any formatting;
        {:.n}: formats numbers with "n" decimal places.


        ```
            let a = "Alice";

            // {}
            println!("My name is {}", name);
            // My name is Alice
    

            //{...}
            println!("My name is {name}");
            // My name is Alice


            
            let pi = 3.14159;

            //{}
            println!("Pi is {}", pi);
            // Pi is 3.14159

            
            //{:.n}
            pritnln!("Pi is {:.2}", pi);
            // Pi is 3.14
            

        ```
    
    > [!IMPORTANT]
    > For more information access: <https://doc.rust-lang.org/std/fmt/>
