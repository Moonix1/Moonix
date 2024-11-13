set -ex

cargo build --release

rm -rf bin/
mkdir -p bin/EFI/BOOT bin/boot/grub img/
cp grub.cfg bin/boot/grub/grub.cfg
cp target/x86_64-unknown-uefi/release/moonix-kernel.efi bin/EFI/BOOT/moonixos.efi

grub-mkrescue --output=img/moonix-os.iso bin/