run:
    RUST_LOG=debug cargo watch -c -x run

# Cross-compile and deploy to server
deploy SERVER_IP:
    @echo "Building for Linux..."
    cargo zigbuild --release --target x86_64-unknown-linux-gnu
    @echo "Stopping blog service..."
    ssh root@{{SERVER_IP}} "systemctl stop blog"
    @echo "Copying binary to server..."
    scp target/x86_64-unknown-linux-gnu/release/blog-engine root@{{SERVER_IP}}:/opt/blog/
    @echo "Starting blog service..."
    ssh root@{{SERVER_IP}} "systemctl start blog"
    @echo "Deployed successfully!"
    @echo "Check status: ssh root@{{SERVER_IP}} systemctl status blog"