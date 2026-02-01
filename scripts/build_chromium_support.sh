#!/bin/bash

set -e

ROOTFS_DIR="rootfs"
BUILD_DIR="build"

echo "Building Linux compatibility layer..."

mkdir -p $BUILD_DIR/compat

cd userspace
cargo build --release --target i686-unknown-linux-musl --bin chromium_launcher
cp target/i686-unknown-linux-musl/release/chromium_launcher $BUILD_DIR/compat/
cd ..

echo "Building Chromium shim library..."
gcc -m32 -shared -fPIC -o $BUILD_DIR/compat/libchromium_shim.so rootfs/usr/lib/chromium_shim.c

echo "Setting up rootfs..."
mkdir -p $ROOTFS_DIR/usr/bin
mkdir -p $ROOTFS_DIR/usr/lib
mkdir -p $ROOTFS_DIR/tmp
mkdir -p $ROOTFS_DIR/root
mkdir -p $ROOTFS_DIR/etc

cp $BUILD_DIR/compat/chromium_launcher $ROOTFS_DIR/usr/bin/
cp $BUILD_DIR/compat/libchromium_shim.so $ROOTFS_DIR/usr/lib/

echo "Downloading Chromium (this may take a while)..."
if [ ! -f $BUILD_DIR/chromium-*.tar.gz ]; then
    wget -P $BUILD_DIR https://commondatastorage.googleapis.com/chromium-browser-snapshots/Linux_x64/LATEST
    REVISION=$(cat $BUILD_DIR/LATEST)
    wget -P $BUILD_DIR https://commondatastorage.googleapis.com/chromium-browser-snapshots/Linux_x64/$REVISION/chrome-linux.zip
    unzip -q $BUILD_DIR/chrome-linux.zip -d $BUILD_DIR/
fi

echo "Installing Chromium to rootfs..."
cp -r $BUILD_DIR/chrome-linux/* $ROOTFS_DIR/usr/lib/chromium/

cat > $ROOTFS_DIR/usr/bin/chromium << 'EOF'
#!/bin/sh
export LD_PRELOAD=/usr/lib/libchromium_shim.so
export WAYLAND_DISPLAY=wayland-0
export XDG_RUNTIME_DIR=/tmp
exec /usr/lib/chromium/chrome --no-sandbox --disable-gpu-sandbox "$@"
EOF

chmod +x $ROOTFS_DIR/usr/bin/chromium

echo "Creating musl environment..."
if [ ! -d $BUILD_DIR/musl ]; then
    wget -P $BUILD_DIR https://musl.libc.org/releases/musl-1.2.4.tar.gz
    tar -xzf $BUILD_DIR/musl-1.2.4.tar.gz -C $BUILD_DIR/
    cd $BUILD_DIR/musl-1.2.4
    ./configure --prefix=/usr --target=i686-linux-musl
    make -j$(nproc)
    make DESTDIR=../../$ROOTFS_DIR install
    cd ../..
fi

echo "Chromium support build complete!"
echo "Rootfs ready in $ROOTFS_DIR/"
