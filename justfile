run:
    RUST_LOG=debug cargo watch -c -x run

# Deploy and run setup script on server
setup SERVER_IP:
    @echo "Copying server-setup.sh to server..."
    scp server-setup.sh root@{{SERVER_IP}}:/root/
    @echo "Running setup script on server..."
    ssh root@{{SERVER_IP}} "chmod +x /root/server-setup.sh && /root/server-setup.sh"
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

# Purge Cloudflare cache
purge-cache:
    @echo "Purging Cloudflare cache..."
    @curl -X POST "https://api.cloudflare.com/client/v4/zones/${CLOUDFLARE_ZONE_ID}/purge_cache" \
      -H "Authorization: Bearer ${CLOUDFLARE_API_TOKEN}" \
      -H "Content-Type: application/json" \
      --data '{"purge_everything":true}' \
      --silent --show-error --fail | jq -r 'if .success then "✓ Cache purged successfully" else "✗ Error: " + .errors[0].message end'

# Deploy and purge cache
deploy-purge SERVER_IP: (deploy SERVER_IP) purge-cache
    @echo "Deployment and cache purge complete!"