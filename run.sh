qemu-system-x86_64 \
	-serial stdio \
	-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -drive if=pflash,format=raw,file=/usr/share/OVMF/OVMF_CODE.fd \
    -drive file=img/moonix-os.iso,format=raw -m 1G \
    -boot d \
