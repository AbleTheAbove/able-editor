cargo build --release
sudo mv target/release/able-editor /bin
sudo rm /bin/ae
sudo ln -s /bin/able-editor /bin/ae
