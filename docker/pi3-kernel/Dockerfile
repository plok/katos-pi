FROM plok/archlinux-devel:latest

RUN git clone --depth=1 https://github.com/raspberrypi/tools.git ~/tools

RUN echo PATH=\$PATH:~/tools/arm-bcm2708/arm-linux-gnueabihf/bin >> ~/.bashrc;\
	source ~/.bashrc

RUN git clone --depth=1 https://github.com/raspberrypi/linux.git ~/linux

RUN \
	cd ~/linux;\
	KERNEL=kernel7;\
	make ARCH=arm CROSS_COMPILE=/root/tools/arm-bcm2708/arm-linux-gnueabihf/bin/arm-linux-gnueabihf- bcm2709_defconfig
	
RUN \
	cd ~/linux; make -j5 ARCH=arm CROSS_COMPILE=/root/tools/arm-bcm2708/arm-linux-gnueabihf/bin/arm-linux-gnueabihf- zImage modules dtbs

