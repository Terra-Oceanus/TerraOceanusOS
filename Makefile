all: run

download:
	rustup run nightly rustc -Z unstable-options --target x86_64-unknown-uefi --print target-spec-json > bootloader/target.json
	rustup run nightly rustc -Z unstable-options --target x86_64-unknown-none --print target-spec-json > kernel/target.json

build:
	cargo build --manifest-path=bootloader/Cargo.toml --release --target x86_64-unknown-uefi

	RUSTFLAGS=" \
		-C relocation-model=static \
		-C link-arg=-no-pie \
		-C link-args=-Tkernel/kernel.ld \
		" \
	cargo build --manifest-path=kernel/Cargo.toml --release --target x86_64-unknown-none

copy: build
	cp target/x86_64-unknown-uefi/release/bootloader.efi ESP/EFI/BOOT/BOOTX64.EFI
	cp target/x86_64-unknown-none/release/kernel ESP/kernel

show: copy
	readelf -l ESP/kernel

debug: copy
	qemu-system-x86_64 \
		-enable-kvm \
		-cpu host,-svm \
		-smp 2 \
		-drive if=pflash,format=raw,readonly=on,file=OVMF/OVMF_CODE_4M.fd \
		-drive if=pflash,format=raw,readonly=on,file=OVMF/OVMF_VARS_4M.fd \
		-drive format=raw,file=fat:rw:ESP \
		-S -s

gdb:
	rust-gdb target/x86_64-unknown-none/release/kernel

run: copy
	qemu-system-x86_64 \
		-enable-kvm \
		-cpu host,-svm \
		-smp 2 \
		-drive if=pflash,format=raw,readonly=on,file=OVMF/OVMF_CODE_4M.fd \
		-drive if=pflash,format=raw,readonly=on,file=OVMF/OVMF_VARS_4M.fd \
		-drive format=raw,file=fat:rw:ESP

clean:
	rm -rf target/

.PHONY: all download build copy show debug gdb run clean