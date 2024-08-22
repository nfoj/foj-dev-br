# Docker Ubuntu

- Docker Ubuntu (Iterative)

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

## Connect Internet


- Update all packages

    apt-get update
    apt-get install iputils-ping curl net-tools
    apt-get install iproute2


- Check if the commands

    which ping
    which curl
    which ifconfig
    which ip


- Ping

    curl -I https://www.google.com


- Install Neovim//Nano ..

    apt-get install neovim

 
