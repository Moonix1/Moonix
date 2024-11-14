qemu-system-x86_64 \
	-serial file:soutput.txt \
    -drive if=pflash,format=raw,file=/usr/share/OVMF/OVMF_CODE.fd \
    -drive file=img/moonix-os.iso,format=raw -m 1G \
    -boot d \
