# Linux Onboarding: obtendo e tratando informações do sistema

# Scripts 

#!/usr/bin/env bash
#!/bin/bash


- Copy 

  - r: recursive copy
  - v: verbose (displayed the messages)
  
```  
  #!/usr/bin/env bash
  echo "Enter the path to the backup:"
  read dir_bkp
  cp -rv $dir_bkp ~/backup
  echo ""
  echo "Backup Complete!"  

```

- Config Path: to make a newly created script accessible from any location within the system, it must be placed in a directory included in the system's search path. This can be achieved by modifying the .profile file or by adding the script to the bin directory.

- IP: ip addr, hostname -I, ifconfig(install)  

- IP route: ip route

- DNS: cat /etc/resolv.conf

- Host: cat /etc/hosts


- System Info: lshw (install) or /proc

```
  apt-get install lshw

  or 

  apt-get install lshw-gtk (Graphic)

  
  lshw | more
  
   description: Computer
    width: 64 bits
    capabilities: smp vsyscall32
  *-core
       description: Motherboard
       physical id: 0
     *-memory
          description: System memory
          physical id: 0
          size: 14GiB
     *-cpu
  
  ...

  lshw | grep sata

  *-sata
     capabilities: sata ahci_1.0 bus_master cap_list rom
  *-sata:0
    capabilities: sata ahci_1.0 bus_master cap_list
    
```

- /proc

```
  cat meminfo | more
    emTotal:       14202696 kB
    MemFree:         9557448 kB
    MemAvailable:   12167004 kB
    Buffers:            3716 kB
    ...

  cat cpuinfo | more
    processor	: 0
    vendor_id	: AuthenticAMD
    cpu family	: 25
    model		: 80
    model name	: AMD Ryzen 5 5600G with Radeon Graphics
  ...

  cat cpuinfo | grep processor
    processor	: 0
    processor	: 1
    processor	: 2
  ...
    
```

- /dev: devices  

```

  df -h
  filesystem      Size  Used Avail Use% Mounted on
  dev             6.8G     0  6.8G   0% /dev
  run             6.8G  1.4M  6.8G   1% /run
  efivarfs        128K   46K   78K  37% /sys/firmware/efi/efivars
  /dev/sdc2       223G  4.3G  217G   2% /
  tmpfs           6.8G  464K  6.8G   1% /dev/shm
  tmpfs           6.8G  101M  6.7G   2% /tmp

  // part

  /dev/sdc2       223G  4.3G  217G   2% /.snapshots
  /dev/sdc2       223G  4.3G  217G   2% /var/cache/pacman/pkg
  /dev/sdc2       223G  4.3G  217G   2% /home
  /dev/sdc2       223G  4.3G  217G   2% /var/log
  /dev/sdc1      1022M  158M  865M  16% /boot

```

- /var/log

```

  //show only the line with ssh
  cat auth.log | grep -i ssh

 
  // -f: follows the changes in real time
  tail -f auth.log
  
  
```

- ps: process  

```

  // process hierarchy
  ps -ef 


  // list all running processes
  ps aux 
  ps aux | grep -i ssh 
  
```

- top == ps 

```
  h = help
  P = cpu
  m = memory
  L = locale
  pstree = process tree

```

- Kill: stop process

```

  // list
  kill -l

  ps -ef
  kill -15 <id process>
  
```

- User

```
  // /ect/sudoers
  groups: list groups

  // list permissions user
  cat /etc/group | grep -i <name>
  cat /etc/group | grep ubuntu

  // root
  sudo <command>
  sudo su 
  sudo -i
  
```

- passwd: command used to change the user's password

```
  passwd
  enter password
  retry password

  // deitals user config
  cat /etc/passwd | grep root
  
```
