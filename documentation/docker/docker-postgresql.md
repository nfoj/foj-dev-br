# Postgresql and Padm4


## Network

- Connect

```
  //
  docker network create --driver bridge my-network
  docker network ls
    
```

- Postgres

```
  docker run --name my-postgres --network=my-network -p 5433:5432 -e POSTGRES_PASSWORD=postgres -d postgres
  docker inspect my-network

  docker run --name my-pgadmin --network=my-network -p 15432:80 -e PGADMIN_DEFAULT_EMAIL=pgadmin@hotmail.com -e PGADMIN_DEFAULT_PASSWORD=postgres -d dpage/pgadmin4
  docker ps

  
```

- Config

```
  http://localhost:15432/
  
  Geral: Docker

  Connection: 
    Host name: my-postgres
    Port: 5432
    Username: postgres
    Password: postgres

  ```
