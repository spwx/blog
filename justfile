run:
    RUST_LOG=debug cargo watch -c -x run

# Cross-compile and deploy to server
deploy SERVER_IP:
    @echo "Building for Linux..."
    cargo build --release --target x86_64-unknown-linux-gnu
    @echo "Copying binary to server..."
    scp target/x86_64-unknown-linux-gnu/release/blog-engine root@{{SERVER_IP}}:/opt/blog/
    @echo "Restarting blog service..."
    ssh root@{{SERVER_IP}} "systemctl restart blog"
    @echo "Deployed successfully!"
    @echo "Check status: ssh root@{{SERVER_IP}} systemctl status blog"