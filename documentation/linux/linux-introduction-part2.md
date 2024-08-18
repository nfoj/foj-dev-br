# Introduction - Part 2

## Commands

- Read the file: cat

  ```
    cat <name>
  
  ```

- Search for data in files: grep

  ```
    grep <name/character/> <local>
    grep http service

    // Not case-sensitive
    grep -i http service

    grep http *

    // Returns the names of the files that match the specified criteria
    grep -l http *  

    // Returns the names of the files that do not match the specified criteria
    grep -L http *

    // Recursive
    grep -r http *

    // Recursive + List
    grep -rl http *

    // Recursive + case-insensitive
    grep -ri http *
    
  ```

- Provides a more efficient way to view large files than cat: less

  ```

    // arrow keys
    less <name>

  
    // Space: scrolls the screen
    // Enter: displays line by line
    // b: back scroll the screen
    // q: exit
    more <name>


    // Shows the first lines of the file
    head <name>

    // Show the first 5 lines
    head -n 5 <name>

    // Show the last lines of the file
    tail <name>

    // Show the last 5 lines
    tail -n 5 <name>
          
  ```

- Searh files: find: find

  ```
    find / -name *.conf

    // super user
    sudo find / -name *.conf

    // maxdeath(directory count) <number>
    find /etc -maxdepth 2 -name *.conf    

    // time - recent files (minutos)
    find . -amin -5

    // time - recent files (days)
    find . -atime -2

    // size (B, M, G)
    find / -size +100M
    find / -size +3G    
    
  ```

- Copy path/files/ search: > , >>

  ```

    // >
    grep ssh service > text.txt
    cat /etc/passwd > text.txt
    
    // subscribes >>
    grep ssh services >> text.txt 

    // redirection: |
    cat /etc/passwd | grep <name> > list.txt

  ```

- Sort: sort

  ```
    cat /etc/passwd
    cat text.txt | sort
    cat text.txt | sort > list.txt
  
  ```

- Quest: Copy the last 5 lines of the .log file from the var folder
  
  ```
    //arch
    tail -n 5 pacman.log | grep 2024-08-17T20:17:23-030 > ~/test/list.txt
  
  ```

- filter: cut -d 

  ```
    //cut -d " " -f<colunms number> <name>
    cut -d " " -f1 list.txt

    cat list.txt | cut -d " " -f1 
    cat list.txt | cut -d " " -f1- 
    cat list.txt | cut -d " " -f1-2 > text1.txt
    cat list.txt | cut -d " " -f1,4- > text2.txt
        
  ```

- Expression: -E   

  ```
    // ^ beginning of the file
    cat <name> | grep -E "^A"

    // $ end of the file
    cat <name> | grep -E "a$"

    // start and end
    cat <name> | grep -E "^a$"

    // case-insensitive
    cat <name> | grep -iE "a"

    // . : all files
    cat <name> | grep -iE "^.at"

    // [abc] [124]
    cat <name> | grep -iE "^[20].."
  
  ```
