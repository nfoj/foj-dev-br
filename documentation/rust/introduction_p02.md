# Rust

2. Syntax and Semantics

    a. print, comments and placeholders;

- Comments:
  - Line: //
  - Block: /* and */

  ```
    // line;
    /* block; */
  
  ```

Examples: 
[Acess](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c28ecc0e7a47f8768700c062d89d3575)


- Print:
  - print!: print text without line breacks;
  - println!: print text with line breaks;

  ```
    print!("Hello, Maria!");
    println!("Hello, Carlos!");

  ```

Examples:
[Acess](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=08b5a78910559890abd6826a4f08088a)


- Line break: \n

  ```
    println!("What is your name? \nMy name is Maria!");
    println!("1 + 1 = \n2");
 
  ```

Example:
[Acess](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8a7202e2416a5d7c345aee5bfe25ac9b)


- Placeholders: {}, {:.n}

  ```
  let a = "Alice";

    // {}
    
    println!("My name is {}", name);
    // My name is Alice


    // {...}
    
    println!("My name is {name}");
    // My name is Alice

    
    // {:.n}
    
    pritnln!("Pi is {:.2}", pi);
    // Pi is 3.14

  ```

Example:
[Acess](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=db4f5a66bea59f54fd60eb8ccf5e3365)


> [!IMPORTANT]
> For more information access: <https://doc.rust-lang.org/std/fmt/>
