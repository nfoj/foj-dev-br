# PostgreSQL


- Install (Arch Linux)
    
    //  Install PostgreSQL 
    sudo pacman -S postgresql

    // Check version 
    postgres --version

    // Initialize PostgreSQL’s data directory
    initdb --locale en_US.UTF-8 -D /var/lib/postgres/data

    // Start the PostgreSQL
    sudo systemctl start postgresql
    sudo systemctl status postgresql
    sudo systemctl enable postgresql

    // Log
    sudo -u postgres psql

    // Create a new user 
    postgres=# CREATE USER <username> WITH ENCRYPTED PASSWORD ‘<password>’;

    //Create a new database
    CREATE DATABASE <dbname>;

    // All permissions to the desired user on the newly created database
    postgres=# GRANT ALL PRIVILEGES ON DATABASE <dbname> TO username;

    // Unistallations
    sudo pacman -Rcns postgresql  





- Install - Page Admin4

    // login 
    sudo -u postgres -i


    // initi
    initdb --locale $LANG -E UTF8 -D '/var/lib/postgres/data/'


    // exit
    exit


    // enable
    sudo systemctl enable --now postgresql


    // check
    sudo systemctl status postgresql


    // password
    psql -U postgres
    postgres=# \password # to set password


    // directories
    sudo mkdir /var/lib/pgadmin
    sudo mkdir /var/log/pgadmin


    //Change the owner of the files
    sudo chown $USER /var/lib/pgadmin
    sudo chown $USER /var/log/pgadmin


    //Create the Python-based virtual
    python3 -m venv pgadmin4


    // active
    source pgadmin4/bin/activate


    // install
    pip install pgadmin4


    // open
    pgadmin4


    // Set the email id or password
    email:- admin@admin.com
    password:- admin@123


    Host: localhost
    Port: 5432
    Maintenance database: postgres
    Username: postgres
    Password: <your password>

    
    Port: 
    http://127.0.0.1:5050






