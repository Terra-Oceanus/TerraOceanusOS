BOOT_TARGET := x86_64-unknown-uefi
KERNEL_TARGET := x86_64-unknown-none

DISK_IMG := disk.img
BOOT_PART := ESP
KERNEL_PART := MAIN

all: run

download:
	rustup run nightly rustc -Z unstable-options --target $(BOOT_TARGET) --print target-spec-json > bootloader/target.json
	rustup run nightly rustc -Z unstable-options --target $(KERNEL_TARGET) --print target-spec-json > kernel/target.json

build:
	cargo build --manifest-path=bootloader/Cargo.toml --release --target $(BOOT_TARGET)

	RUSTFLAGS=" \
		-C relocation-model=static \
		-C link-arg=-no-pie \
		-C link-args=-Tkernel/kernel.ld \
		" \
	cargo build --manifest-path=kernel/Cargo.toml --release --target $(KERNEL_TARGET)

show: build
	readelf -l target/$(KERNEL_TARGET)/release/kernel

disk: build
	rm -f $(DISK_IMG)
	dd if=/dev/zero of=$(DISK_IMG) bs=1M count=512

	parted $(DISK_IMG) --script \
		mklabel gpt \
		mkpart $(BOOT_PART) fat32 1MiB 64MiB \
		set 1 esp on \
		mkpart $(KERNEL_PART) fat32 64MiB 100%

	@LOOP=$$(sudo losetup -Pf --show $(DISK_IMG)); \
		sudo udevadm settle; \
		\
		sudo mkfs.vfat -F 32 $${LOOP}p1; \
		mkdir -p mnt/$(BOOT_PART); \
		sudo mount $${LOOP}p1 mnt/$(BOOT_PART); \
		sudo mkdir -p mnt/$(BOOT_PART)/EFI/BOOT; \
		sudo cp target/$(BOOT_TARGET)/release/bootloader.efi mnt/$(BOOT_PART)/EFI/BOOT/BOOTX64.EFI; \
		sudo umount mnt/$(BOOT_PART); \
		\
		sudo mkfs.vfat -F 32 $${LOOP}p2; \
		mkdir -p mnt/$(KERNEL_PART); \
		sudo mount $${LOOP}p2 mnt/$(KERNEL_PART); \
		sudo cp target/$(KERNEL_TARGET)/release/kernel mnt/$(KERNEL_PART)/kernel; \
		sudo umount mnt/$(KERNEL_PART); \
		\
		sudo losetup -d $${LOOP}

	rm -rf mnt

debug: disk
	qemu-system-x86_64 \
		-enable-kvm \
		-cpu host,-svm \
		-smp 2 \
		-machine q35 \
		-drive if=pflash,format=raw,readonly=on,file=OVMF/OVMF_CODE_4M.fd \
		-drive if=pflash,format=raw,readonly=on,file=OVMF/OVMF_VARS_4M.fd \
		-drive file=$(DISK_IMG),format=raw,if=none,id=nvmedrive \
		-device nvme,serial=deadbeef,drive=nvmedrive,bus=pcie.0,addr=0x4 \
		-S -s

gdb:
	rust-gdb target/$(KERNEL_TARGET)/release/kernel

run: disk
	qemu-system-x86_64 \
		-enable-kvm \
		-cpu host,-svm \
		-smp 2 \
		-machine q35 \
		-drive if=pflash,format=raw,readonly=on,file=OVMF/OVMF_CODE_4M.fd \
		-drive if=pflash,format=raw,readonly=on,file=OVMF/OVMF_VARS_4M.fd \
		-drive file=$(DISK_IMG),format=raw,if=none,id=nvmedrive \
		-device nvme,serial=deadbeef,drive=nvmedrive,bus=pcie.0,addr=0x4 \
		-trace file=pci.log,enable=pci_*

clean:
	rm -rf target/

.PHONY: all download build show disk debug gdb run clean