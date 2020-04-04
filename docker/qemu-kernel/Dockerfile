FROM plok/archlinux-devel:latest

RUN git clone --depth=1 --branch v5.6 https://github.com/torvalds/linux.git ~/linux

RUN cd ~/linux; make defconfig
RUN cd ~/linux; make -j5
