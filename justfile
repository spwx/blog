run:
    RUST_LOG=debug cargo watch -c -x run

# Deploy and run setup script on server
# Usage: just setup <server-ip> [domain] [install-dir] [binary-name]
# Example: just setup 192.168.1.100 example.com /opt/blog blog-engine
setup SERVER_IP DOMAIN="wall.ninja" INSTALL_DIR="/opt/blog" BINARY_NAME="blog-engine":
    @echo "Copying server-setup.sh to server..."
    scp server-setup.sh root@{{SERVER_IP}}:/root/
    @echo "Running setup script on server..."
    ssh root@{{SERVER_IP}} "chmod +x /root/server-setup.sh && /root/server-setup.sh {{DOMAIN}} {{INSTALL_DIR}} {{BINARY_NAME}}"
    @echo "Server setup complete!"

# Cross-compile and deploy to server
# Usage: just deploy <server-ip> [install-dir] [binary-name]
deploy SERVER_IP INSTALL_DIR="/opt/blog" BINARY_NAME="blog-engine":
    @echo "Building for Linux..."
    cargo zigbuild --release --target x86_64-unknown-linux-gnu
    @echo "Stopping blog service..."
    ssh root@{{SERVER_IP}} "systemctl stop blog"
    @echo "Copying binary to server..."
    scp target/x86_64-unknown-linux-gnu/release/{{BINARY_NAME}} root@{{SERVER_IP}}:{{INSTALL_DIR}}/
    @echo "Starting blog service..."
    ssh root@{{SERVER_IP}} "systemctl start blog"
    @echo "Deployed successfully!"
    @echo "Check status: ssh root@{{SERVER_IP}} systemctl status blog"

# Purge Cloudflare cache
purge-cache:
    @echo "Purging Cloudflare cache..."
    @curl -X POST "https://api.cloudflare.com/client/v4/zones/${CLOUDFLARE_ZONE_ID}/purge_cache" \
      -H "Authorization: Bearer ${CLOUDFLARE_API_TOKEN}" \
      -H "Content-Type: application/json" \
      --data '{"purge_everything":true}' \
      --silent --show-error --fail | jq -r 'if .success then "✓ Cache purged successfully" else "✗ Error: " + .errors[0].message end'

# Deploy and purge cache
deploy-purge SERVER_IP INSTALL_DIR="/opt/blog" BINARY_NAME="blog-engine": (deploy SERVER_IP INSTALL_DIR BINARY_NAME) purge-cache
    @echo "Deployment and cache purge complete!"