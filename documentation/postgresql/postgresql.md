# PostgreSQL


- Basic Commands 

```
  \h = help!

  \h create role = help create role

  \? = list commands

  \l = list data base

  \du = list users

  \c <data base> = acess data base especification

  \q = exit

  \d = list table
  \dS = list table system 

  \! = terminal
    exit > user sql

  \! date = acess terminal command
  \! pwd
  
```


- Create DBase

```
  // #
  CREATE DATABASE "name" 
  WITH
  OWNER = postgres
  ENCODING = 'UTF8'
  CONNECTION LIMIT = -1;
  
```

  -1: no limit connect


- Delete DBase

```
  DROP DATABASE "name";
  
```


- Types

  - Numerics:
    integer (-1 ... 1)
    real (-1.0 ... 1.0)
    serial (+1.0 - Autoincrement)
    numeric (decimal types explict - .00, .000)  

  - String:
    varchar (n) 
    char (n definition)
    text (no definition)

  - Boolean
    true
    false

  - time
    date 'YYYY-MM-DD'
    time 'HH24:MM:SS'
    timestamp 'YYYY-MM-DD HH24:MM:SS'


- Types - Use

```

  CREATE TABLE student (
    id SERIAL,
    name VARCHAR(255),
    cpf CHAR(11),
    observation TEXT,
    year INTEGER,
    money NUMERIC(10,2),
    height real,
    active BOOLEAN
    date_of_birth DATE
    hour TIME,
    registration TIMESTAMP        
  );

  
  // NUMERIC(10,2): 12345678,90

```


- Insert sigle row into table

```
  INSERT INTO "table name" (column_1, column2, column_3, ...)
    VALUES('value_column_1', 'value_column_2',value_column_3, ...);


  INSERT INTO student (name, cpf, observation, year, money, height, active, date_of_birth, hour, registration)
    VALUES ('Alice', '00000000000', 'text', 29, 100.19, 1.69, true, '1999-10-11', '17:48:00', '2023-06-02 08:40:00');

  
```


- CRUD (Create, Read, Update and Delete)

  SELECT: column or * (all columns)
    FROM: table name
    WHERE: number (id - serial);

  UPDATE: name table
    SET column = 'new value'
    WHERE: number (id - serial);

  DELETE: column or * (all columns)
    FROM: table name
    WHERE: name 'name'
    
```
  SELECT *
    FROM student
    WHERE id = 1;

  UPDATE student
    SET name = 'Roberto', 
    cpf = '11111111111',
    observation = 'Text 2',
    year = 50, 
    money = 10.19,
    height = 2.01,
    active = FALSE,
    date_of_birth = '1990-01-19'
    hour = '11:20:00',
    registration = '2022-11-14 11:13:00'
    WHERE id = 1;


  SELECT *
    FROM student
    WHERE name = 'Roberto';

  DELETE *
    FROM student
    WHERE name = 'Roberto';  
  
```

- SELECT and AS

  SELECT: column or * (all columns)
  AS: temporary names

```
  SELECT name, year, ...
    FROM student;

  SELECT name AS "FIRST and LAST",
    year
    observation AS obs
  FROM student;  
  
```

- Create DBase

```

  
```

- Create DBase

```

  
```

- Create DBase

```

  
```

- Create DBase

```

  
```

- Create DBase

```

  
```

- Create DBase

```

  
```

- Create DBase

```

  
```




- Basic Commands 

```
  \h = help!
  \h create role = help create role

  \? = list commands

  \l = list data base

  \du = list users

  \c <data base> = acess data base especification

  \q = exit

  \d = list table
  \dS = list table system 

  \! = terminal
    exit > user sql

  \! date = acess terminal command
  \! pwd
  
```

- Select

```

  SELECT * FROM <NAME TABLE>;


```

- 

```

```

- 

```

```

- 

```

```

- 

```

```

- 

```

```

