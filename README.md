# ESP32 QEMU Error Reproduction

## Build in Docker container

```
cargo espflash save-image --chip esp32 --release --merge merged.bin
```

## Run in QEMU

```
qemu-system-xtensa -nographic -machine esp32 -drive file=merged.bin,if=mtd,format=raw
```

## Expected output

Running in QEMU results in a crash right after the system boots:

```
Adding SPI flash device
ets Jul 29 2019 12:21:46

rst:0x1 (POWERON_RESET),boot:0x12 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0030,len:7104
load:0x40078000,len:15576
load:0x40080400,len:4
ho 8 tail 4 room 4
load:0x40080404,len:3876
entry 0x4008064c
I (692) boot: ESP-IDF v5.1-beta1-378-gea5e0ff298-dirt 2nd stage bootloader
I (693) boot: compile time Jun  7 2023 07:48:23
I (693) boot: Multicore bootloader
I (707) boot: chip revision: v0.0
I (709) boot.esp32: SPI Speed      : 40MHz
I (709) boot.esp32: SPI Mode       : DIO
I (710) boot.esp32: SPI Flash Size : 4MB
I (715) boot: Enabling RNG early entropy source...
I (723) boot: Partition Table:
I (723) boot: ## Label            Usage          Type ST Offset   Length
I (723) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (723) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (724) boot:  2 factory          factory app      00 00 00010000 003f0000
I (728) boot: End of partition table
I (733) esp_image: segment 0: paddr=00010020 vaddr=3f400020 size=010c8h (  4296) map
I (748) esp_image: segment 1: paddr=000110f0 vaddr=3ffb0000 size=005e4h (  1508) load
I (762) esp_image: segment 2: paddr=000116dc vaddr=40080000 size=01318h (  4888) load
I (778) esp_image: segment 3: paddr=000129fc vaddr=00000000 size=0d61ch ( 54812)
I (796) esp_image: segment 4: paddr=00020020 vaddr=400d0020 size=02f4ch ( 12108) map
I (812) boot: Loaded app from partition at offset 0x10000
I (812) boot: Disabling RNG early entropy source...
Fatal exception (15): LoadStorePIFAddrError
epc1=0x400d03a2, epc2=0x00000000, epc3=0x00000000, excvaddr=0x7ff61bc8, depc=0x00000000
```