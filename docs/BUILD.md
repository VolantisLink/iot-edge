# iot-edge

## 准备

1. 下载安装[debian11](https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-11.3.0-amd64-netinst.iso)
2. 安装基础环境：

```sh
apt install -y podman crun curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
reboot
cargo install cross --git https://github.com/cross-rs/cross
```

## 编译

1. 编译iot-edge需要在cross的官方docker镜像基础上定制镜像。
```
cd docker
./build-docker-images.sh
cd ..
```
2. 执行`make`命令，完成编译。
