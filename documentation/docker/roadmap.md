# Docker!


## Introduction

    - What are Containers?
    - Need for Containers
    - Bare Metal vs VM vs Containers
    - Docker and OCI


## Underlying Technologies

    - Namespaces
    - cgroups
    - Union Filesystems


## Installation Setup

    - Docker Desktop (Linux)
        
    ```
  
       Arch Linux
       
       // Install
       sudo pacman -S docker


       //Permission (No sudo)
       sudo usermod -aG docker $USER 


       // Start Docker
       sudo systemctl start docker


       // Start docker (boot)
       sudo systemctl enable docker
        
       
    ```

    - Docker Engine


## Docker Basics


    docker container run: create e run a new container from an image

    ```
        sudo docker run hello-world

    ```


    docker ps: list containers

    ```
        sudo docker ps

        CONTAINER ID   IMAGE     COMMAND   CREATED   STATUS    PORTS     NAMES

    ```


    docker container ls -a: list container details 

    ```
        sudo docker container ls -a

        CONTAINER ID   IMAGE         COMMAND    CREATED         STATUS                     PORTS     NAMES
        ef9bced584b5   hello-world   "/hello"   5 minutes ago   Exited (0) 5 minutes ago             cool_varahamihira

    ```

    
    docker stop + ID: terminates the execution of the designated container. 

    ```
        sudo docker stop ef9bced584b5

        CONTAINER ID   IMAGE         COMMAND    CREATED         STATUS                     PORTS     NAMES
        ef9bced584b5   hello-world   "/hello"   5 minutes ago   Exited (0) 5 minutes ago             cool_varahamihira

    ```

    
    docker start + ID: runs the designated container. 

    ```
        sudo docker start ef9bced584b5

        CONTAINER ID   IMAGE         COMMAND    CREATED         STATUS                     PORTS     NAMES
        ef9bced584b5   hello-world   "/hello"   5 minutes ago   Exited (0) 5 minutes ago             cool_varahamihira

    ```


> [!NOTE]  
> Example: Ubuntu
    

    Docker Ubuntu (Iterative)
    
    ```

        sudo docker pull ubuntu
        sudo docker run -it ubuntu bash
        sudo docker stop ef9bced584b5 
        sudo docker start ef9bced584b5
        sudo docker exec -iti ef9bced584b5
   
        or

        sudo docker run -it ubuntu bash
        sudo docker stop ef9bced584b5
        sudo docker start ef9bced584b5
        sudo docker exec -iti ef9bced584b5 

    ```

    - i: Ensures that you can provide input to the container;
    - t: Allocates a pseudo-TTY;
    - bash: terminal.








