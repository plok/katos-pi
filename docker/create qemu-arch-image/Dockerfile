FROM plok/archlinux-devel:latest

RUN dd if=/dev/zero of=/root/image.img bs=512M count=8

# sfdisk /root/image.img << EOF \
#    label=dos \
#    start=1MiB type=83 bootable \
#    EOF

RUN losetup -fP /root/image.img

RUN mkfs.ext4 /dev/loop0p1

RUN mkdir -p /mnt/loop
RUN mount /dev/loop0p1 /mnt/loop

RUN pacstrap /mnt/loop base
RUN arch-chroot /mnt/loop pacman --noconfirm -S linux mkinitcpio linux-firmware

# TODO remove auto-detect from /etc/mkinitrdcpio.conf

RUN arch-chroot /mnt/loop mkinitcpio -p linux
RUN arch-chroot /mnt/loop echo "root:katos" | chpasswd

# To run the image; copy the initram and vmlinuz from the /boot of the image
# qemu-system-x86_64 -m 2048 -kernel vmlinuz-linux -initrd initramfs-linux.img -drive file=image.img,format=raw -append root=/dev/sda1