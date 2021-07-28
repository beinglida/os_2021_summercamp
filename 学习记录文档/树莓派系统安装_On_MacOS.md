确认设备“

```cpp
diskutil list
```

确认设备路径为：

/dev/disk2

将分区卸载：

```undefined
diskutil unmountDisk /dev/disk2
```

/Users/dali/Downloads/Compressed/2021-05-07-raspios-buster-armhf-full.img

sudo dd if=/Users/dali/Downloads/Compressed/2021-05-07-raspios-buster-armhf-full.img of=/dev/disk2 bs=4m;sync

![image-20210623225932667](/Users/dali/Library/Application Support/typora-user-images/image-20210623225932667.png)



network={
  ssid=“oslab_5G”
  psk=“gcf10l-jjmqq”
}

```
country=CN
ctrl_interface=DIR=/var/run/wpa_supplicant Group=netdev
update_config=1

network={
  ssid=“oslab_5G”
  psk=“gcf10l-jjmqq”
}
```