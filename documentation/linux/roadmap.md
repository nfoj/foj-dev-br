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

> [!IMPORTANT]
> Arch Linux, does not have the ubs package. 
> Installation (root): pacman -Fy lsusb



