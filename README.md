# A small OS written in Rust

Following the instruction from [Writing an OS in Rust](https://os.phil-opp.com/second-edition/) by Philipp Oppermann

## Prerequesites

* Install Rust (https://rustup.rs/)
* Install `xargo`
```
cargo install xargo
rustup component add rust-src
```
* Install `bootimage`
```
cargo install bootimage
```
* Run Linux <br>You can refer to the [blog post](https://os.phil-opp.com/freestanding-rust-binary/#overwriting-the-entry-point) for building on Windows and macOS
* (Optional) qemu or VM for testing the built kernel

## Building

1. Clone this Repo
```
git clone https://github.com/huyvn/blog-os.git
```
2. Build the bootable image 
```
cd blog-os/
bootimage build
```
You should see a `bootimage.bin` file inside your current directory. You can boot it through a VM or copy to a USB drive to boot on real x86_64 hardware. <br>
**NOTE** This is not a CD image, so burning to CD doesn't work. 

## Booting

* With [qemu](https://www.qemu.org/)
```
qemu-system-x86_64 -drive format=raw,file=bootimage.bin
```

* On real hardware with USB disk
**WARNING** this command will delete everything on disk. Proceed with caution.
```
dd if=bootimage.bin of=/dev/sdX && sync
```
where `/dev/sdX` is the your USB drive.

* On VirtualBox or VMWare virtual machine
Follow the instruction here to convert the raw image file to VDI or VMDK format <br>https://blog.sleeplessbeastie.eu/2012/04/29/virtualbox-convert-raw-image-to-vdi-and-otherwise/


## License

The source code is licensed under Apache License (Version 2.0).
