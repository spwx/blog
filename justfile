run:
    RUST_LOG=debug cargo watch -c -x run

# Deploy and run setup script on server
setup SERVER_IP:
    @echo "Copying setup-blog.sh to server..."
    scp setup-blog.sh root@{{SERVER_IP}}:/root/
    @echo "Running setup script on server..."
    ssh root@{{SERVER_IP}} "chmod +x /root/setup-blog.sh && /root/setup-blog.sh"
    @echo "Server setup complete!"

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