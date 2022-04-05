# Config

My configuration to build the [Linux kernel](https://github.com/starfive-tech/linux)
on the StarFive VisionFive board.

## Compilation

Install the compiler:

```
$ sudo apt-get install libncurses-dev libssl-dev bc flex bison make gcc gcc-riscv64-linux-gnu
```

Default config:

```
$ make -j24 ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- visionfive_defconfig
```

Compile and install in the SSD:

```
make -j24 ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu-
mkdir mnt
sudo mount /dev/sda3 mnt
sudo cp arch/riscv/boot/Image mnt
sudo cp arch/riscv/boot/dts/starfive/jh7100-starfive-visionfive-v1.dtb mnt
sudo umount mnt
rm -fr mnt
```
