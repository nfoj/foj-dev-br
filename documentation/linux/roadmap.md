# Arch Linux - LPIC-1

101.1 Lesson 1

1. Introduction

1.1 Device Inspection in Linux

    - lspci: show all devices currently connected to the PCI bus;
    - lsusb: list USB devices currently connected to the machine.

    ```
    lspic

    00:00.0 Host bridge: Advanced Micro Devices, Inc. [AMD] Renoir/Cezanne Root Complex
    00:00.2 IOMMU: Advanced Micro Devices, Inc. [AMD] Renoir/Cezanne IOMMU

   
    lsusb

    Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
    Bus 001 Device 002: ID 8087:0029 Intel Corp. AX200 Bluetooth

    ```

    Obs: 

    00:00.0 (bus address)
    1d6b:0002 (id - identification address)

    
> [!IMPORTANT]
> Arch Linux, does not have the usb package;
> Install: sudo pacman -S usbutils
> or! Command (root): pacman -Fy lsusb

> [!WARNING]  
> -F: Forces the reinstallation of packages, overwriting existing files;
> y: Automatically answers 'yes' to all prompts.
 
    
    ```

    lspci -s 00:00.0 -v
    
    00:00.0 Host bridge: Advanced Micro Devices, Inc. [AMD] Renoir/Cezanne Root Complex
        Subsystem: Micro-Star International Co., Ltd. [MSI] Device 7a38
        Flags: fast devsel


    ```

    - s: selection;
    - v: verbose;
    - k: kernel (lspci -s 00:00.0 -k);


    ```    
    lsusb -v -d 1d6b:0002

    Bus 005 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
    Couldn't open device, some information will be missing
    Device Descriptor:

    ```

    - v: verbose;
    - d: description;
    - t: tree (lsusb -t);
    - s: selection (lsubs -s 001:001); 



    
