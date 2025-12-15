# 山海OS | TerraOceanusOS

山海OS，基于 Intel x86-64 架构，用 [Rust](https://www.rust-lang.org/) 实现。

## 准备工作

开发工作在 WSL: Ubuntu-24.04 LTS 中进行。

### Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

需要重启终端。

添加所需 target：

```bash
rustup target add x86_64-unknown-uefi x86_64-unknown-none
```

### Make

```bash
sudo apt install build-essential
```

### Qemu

```bash
sudo apt install qemu-system-x86
```

开启KVM权限：

```bash
sudo usermod -aG kvm $USER
```

需要重新登陆。

### 磁盘相关

```bash
sudo apt install parted dosfstools mkfs.ntfs
```

### UEFI 固件

```bash
sudo apt install -y ovmf
mkdir -p OVMF
cp /usr/share/OVMF/OVMF_CODE_4M.fd /usr/share/OVMF/OVMF_VARS_4M.fd OVMF/
```
