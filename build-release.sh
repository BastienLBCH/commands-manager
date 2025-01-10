cargo build --release
cargo build --target=x86_64-pc-windows-gnu --release

cp target/release/ssh-manager bin/ssh-manager_apple_silicon
cp target/x86_64-pc-windows-gnu/release/ssh-manager.exe bin/ssh-manager_windows_x86_64.exe
