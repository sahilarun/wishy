#!/bin/bash

DISK_IMG=$1
USER_BIN=$2
INITRD=$3

LOOP_DEV=$(sudo losetup -f)
MOUNT_POINT=/tmp/wishy_mount

echo "Creating ext2 filesystem on $DISK_IMG"

sudo losetup -o 1048576 $LOOP_DEV $DISK_IMG

sudo mkfs.ext2 $LOOP_DEV

mkdir -p $MOUNT_POINT
sudo mount $LOOP_DEV $MOUNT_POINT

sudo mkdir -p $MOUNT_POINT/sbin
sudo mkdir -p $MOUNT_POINT/etc
sudo mkdir -p $MOUNT_POINT/tmp
sudo mkdir -p $MOUNT_POINT/usr/bin

if [ -f "$USER_BIN" ]; then
    sudo cp $USER_BIN $MOUNT_POINT/sbin/init
fi

echo "wishy OS v0.1" | sudo tee $MOUNT_POINT/etc/motd

sudo umount $MOUNT_POINT
sudo losetup -d $LOOP_DEV
rmdir $MOUNT_POINT

echo "ext2 filesystem created successfully"
