# Images

## dockfile

> [!NOTE]  
> You need to have the files on your machine to be able to generate the Docker image.

   - Create image 
  
    FROM = node:14 // Indicate <image> and <version>
    WORKDIR = Predefined directory
    COPY = Copy files // In case you use a workdir (. .)
    RUN npm install = install the required dependencies 
    ENTRYPOIN npm start =  start operation

    
  ```
    FROM node:14
    WORKDIR /app-node
    COPY . .
    RUN npm install
    ENTRYPOINT npm start
    
  ```

  - Build

  ```
  
    //sudo docker build -t <title>/<namedir>:<version><path>
    sudo docker build -t teste/page-node:1.0 .
     
  ```

  -t : title
  . : path


> [!IMPORTANT]  
> Acess [Docker Reference](https://docs.docker.com/reference/dockerfile/)
