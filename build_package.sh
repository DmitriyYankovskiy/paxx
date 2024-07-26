cargo build --release
if [ ! -d "package" ]; then
    mkdir package
fi
cp target/release/paxx package/
echo "sudo cp paxx /usr/local/bin/" > package/install.sh

zip -r paxx_installer.zip package

rm -r package