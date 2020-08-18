lsusb

```
Bus 001 Device 036: ID 0483:5740 STMicroelectronics STM32F407
Bus 001 Device 038: ID 0483:df11 STMicroelectronics STM Device in DFU Mode
```

当使用stlink写入新的程序后，会出现固件被破坏的情况，这时，可以使用如下命令重新刷入固件：

```
dfu-util -D /home/ros-master/snap/arduino/41/.arduino15/packages/OpenCR/hardware/OpenCR/1.4.15/bootloaders/opencr_boot.bin -d 0483:df11 -a 0 -s 0x08000000
```

使用dynamixel workbench扫描的结果

```
Succeed to init : 9600
Find 0 Dynamixels
Succeed to init : 57600
Find 0 Dynamixels
Succeed to init : 115200
Find 0 Dynamixels
Succeed to init : 1000000
Find 2 Dynamixels
id : 1 model name : XL430-W250
id : 2 model name : XL430-W250
Succeed to init : 2000000
Find 0 Dynamixels
Succeed to init : 3000000
Find 0 Dynamixels
Succeed to init : 4000000
Find 0 Dynamixels
```

则现有dynamixel的配置如下：

```
baud rate: 1000000
max goal velocity 265

```