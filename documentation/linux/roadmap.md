# Arch Linux - LPIC-1

101.1 Lesson 1

1. Introduction


1.0 Bios

    - Bios: basic input/output system:
        - System boot process: Loads the operating system stored on the hard drive or other storage device;
        - POST (Power-On Self-Test): Checks RAM memory, processor, storage devices, and other components;
        - BIOS/UEFI setup: Allows you to set the time, date, boot order, and other options.

    - UEFI: Unified extensible firmware interface
        - Bios + Upgrades;
            > Ethernet network configuration;
            > Fast boot
            > UEFI Shell


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



    lsmod: shows all currently loaded modules.

        - Module: module name;
        - Size: amount of RAM occupied by the module, in byte;
        - used by: depending modules.

    ```
        Module                  Size  Used by
        uinput                 20480  1
        rfcomm                102400  7
        snd_seq_dummy          12288  0
        snd_hrtimer            12288  1
        snd_seq               131072  7 snd_seq_dummy
        snd_seq_device         16384  1 snd_seq

    ```
        



> [!TIP]
> Kmod package: collection of tools for executing standard operations on Linux kernel modules (insert, remove, list, check properties, resolve dependencies and aliases, and list currently loaded kernel modules> );



































