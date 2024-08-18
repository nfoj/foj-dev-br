# Linux

## Commands 

- Show directory: pwd

  ```
    pwd
    /home/user
  
  ```

- Tab: autocomplete
  
  ```
    tab + <name>
  
  ```

  List content: ls, ls -a, ls -al 

  ```
    ls 
    downloads  git

    ls *
    arq  arq1  arq2

    dir:
    dir 1
    
    ls -a
    .  ..  .bash_history  .bash_logout

    la -al
    drwx------ 1 user user  280 Aug 16 06:31 .
    drwxr-xr-x 1 root root    8 Jul 28 00:00 ..
  
  ```

- Clear screen: clear or ctrl + l
  
  ```
    clear
    ctrl + l
  
  ```

- Help: <command> --help or man <command>

  ```
    ls --help

    Usage: ls [OPTION]... [FILE]...
    List information about the FILEs (the current directory by default).
    Sort entries alphabetically if none of -cftuvSUX nor --sort is specified.

    Mandatory arguments to long options are mandatory for short options too.
    -a, --all                  do not ignore entries starting with .
    -A, --almost-all           do not list implied . and ..
    --author               with -l, print the author of each file
  
   ```

- FHS (Filesystem): cd 
  
  ```
    cd 
    home/user

    cd .config
    alacritty  dconf  gtk-3.0

    cd /etc/alsa
    conf.d

    // return
    cd -
    cd /etc

    // next 
    cd ../.config/ 
  
  ```

- Create directories: dir
  
  ```
    mdkir <name>

    // name + space
    mkdir <name>\ <name>
  
    // multiples directories
    mkdir -p dir/dir/dir1/dir2  

    // ocult directories
    mdkir .<name>
    
  ```

- Create files: touch or >

  
  ```
    touch <name>
    touch arq.txt
    
    > <name>
    > arq.txt

    // multiples files
    touch <name> <name2> <name3>

    // ocult files
    touch .<name>
    
  ```

- Remove: rm <name> or rm -r <name> or rm -rf <name>
  
  ```
    // files
    rm <name>

    // dir 
    rm -r 

    // delete all
    rm -rf
    
  ```

> [!WARNING]  
> Be careful when deleting a file or folder. You can only recover it if you have backup software installed or if you've made a backup..


- Copy
  
  ```
    cp <nome> path

    // cp all
    cp -r * path
    cp -r * ../dir2/dir3
    
  ```

- History 
  
  ```
      history
  
  ```

- Globbing: *, ? and []

  ```
    // files all
    ls arq* 
    ls *1*
  
    // ? = substitutes any character
    ls arq1?
    ls arq??

    // [] = ranger of value
    ls arq[1-5]
    ls *[1-5]
    ls arq[1,2,5]
    ls ???[1-5]
    ls ???[1-5]*
    ls [A,a]rq[1-5]
    
  ```
