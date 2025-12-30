#!/bin/bash
set -e

echo "=== Blog Server Setup ==="

# Update system
echo "Updating system packages..."
apt-get update && apt-get upgrade -y

# Install only what we need
echo "Installing Caddy and OpenSSH..."
apt-get install -y caddy openssh-server

# Create blog directory
echo "Creating blog directory..."
mkdir -p /opt/blog

# Create systemd service
echo "Creating blog systemd service..."
cat > /etc/systemd/system/blog.service <<'EOF'
[Unit]
Description=Blog Engine
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/blog
ExecStart=/opt/blog/blog-engine
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

# Create Caddyfile
echo "Creating Caddyfile with logging configuration..."
cat > /etc/caddy/Caddyfile <<'EOF'
{
    log {
        output file /var/log/caddy/access.log {
            roll_size 100mb
            roll_keep 5
            roll_keep_for 720h
        }
    }
}

wall.ninja {
    reverse_proxy localhost:3000
    encode gzip

    log {
        output file /var/log/caddy/wall.ninja.log {
            roll_size 100mb
            roll_keep 5
            roll_keep_for 720h
        }
    }
}

www.wall.ninja {
    redir https://wall.ninja{uri} permanent
}
EOF

# Enable and restart services to pick up config changes
echo "Reloading systemd configuration..."
systemctl daemon-reload
echo "Enabling and restarting blog service..."
systemctl enable blog
systemctl restart blog
echo "Enabling and reloading Caddy service..."
systemctl enable caddy
systemctl reload-or-restart caddy

echo "=== Setup Complete! ==="
