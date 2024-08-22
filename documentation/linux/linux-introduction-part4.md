# Linux Onboarding: trabalhe com usuários, permissões e dispositivos

- Create Users

```
  // Create - No configure
  useradd <name>

  // Create user and group
  adduser <name>

  // verification users
  cat /etc/group | grep <name>

  // verification password
  cat /etc/shadow | grep <name>

  //delete
  userdel <name>
  
```

- Groups

```
    // create group
    groupadd projects
  
    // verification
    cat /etc/group

    // add user 
    usermod -aG <name-group> <nome-user>

    // verification
    groups <name-user>

    // delete 
    groupdel <name-group>      

```

  a: add
  G: group supplementary
  g: primary


- Permission

  sudo chmod -R 755
  
  r: read
  w: write
  x: execute
  d: directory
  -: file

  u-: user
  g-: group
  o-: owner

  755:rwx r-x r-x 
  7: rwx(User) 
  5: r-x(Group)
  5: r-x(other)

- Numerical Presentation
  7: rwx
  6: rw-
  5: r-x
  4: r--
  3: -wx
  2: -w-
  1: --x
  0: ---

OBS: A directory or file can only have one owner and one group

```

  // permission
  chmod 700 <name-file> or <name-dir>
  chmod o-r <name-file> or <name-dir>

  // permission user
  chown <name-user> <name-file> or <name-dir>

  // permission group 
  chgrp <name-group> <name-file> or <name-dir>

  // permission user and group
  chwon <name-user>:<name-group> <name-file> or <name-dir>

  // special permission (Tree)
  chmod g+s <name-file> or <nome-dir>
      
```

- Link

```
  ls -s <path>
  
```

- Packages (Update - Install - Remove - Upgrade (Attention)

```

  // ubuntu
  apt update
  apt install <name-package>
  
  apt remove <name-package>
  apt autoremove <name-package> // remove dependencies
  
  apt upgrade
  

  // Debian or Docker  
  apt-get update
  apt-get install <name-package>

  apt-get remove <name-package>
  apt-get autoremove <name-package>

  apt-get upgrade
  
```

- Packages - Search - List - Show (Details)

```
  apt-get search <name-packager>
  apt-get search <name-packager> | more

  apt list
  apt list | grep installed > list-package.txt

  apt show <name-package>
  
```

- Add Disk - List

```
  // list disk
  lshw -c disk
  fdisk -l 
  
```

- Disk partition: fdisk

```
  fdisk /dev/sdb
  m: help
  p: print partition table
  l: list partition
  n: new partition
  ... 
  
```

- Format: mkfs

``` 
  // list types
  mkfs. 
  mkfs -t ext4 /dev/sdb1

  //verification
  mount | grep sd
  
```

  mkdfs: make file system
  -t: type
  ext4 // btrfs // xfs // f2fs
  path partition
  mount: list disk mount  


- Mount

```
  cd /media/
  sudo mkdir disk2
  mount /dev/sdb1
  mount /dev/sdb1 /media/disk2/
  sudo mount /dev/sdb1 /media/disk2/
  mount | grep sd 

  cat /etc/fstab
  sudo blkid

  cd /etc
  cp fstab fstab.pkg
  sudo vi /etc/fstab
  past UUID= "id"+ path + ext4 (ext) defautls 0 2 (Order!)
  sudo umount /media/disk2  // verification last line
  sudo mount -a
  cd /media/ 
  ls -l
  
```

- Service

```
  // Status - Start - Stop
  systemctl status <name-package>
  systemctl start <name-package>
  systemctl stop <name-package>


  // disable - enable
  systemctl disable <name-package>
  systemctl enable <name-package>
  

  // Service
  service <name-package>
  service --status-all
  
```
