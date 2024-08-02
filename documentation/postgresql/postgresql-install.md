# PostgreSQL


- Install (Arch Linux)

```

    sudo pacman -S postgresql

```

- Check version 

```

    postgres --version

```

- Confirm psql is not running

```

    sudo systemctl status postgresql

```

- Login as the postgresql user 

```
    
    // Arch Linux
    sudo su - postgres
    
```

- Exit

```
    
    // Arch
    exit

```


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






