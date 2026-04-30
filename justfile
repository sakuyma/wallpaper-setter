default:
    cargo build --release 
    sudo cp target/release/wallpapers-rs /usr/bin/
