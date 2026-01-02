#!/bin/bash
set -e

# Configuration - can be overridden via environment variables
DOMAIN="${DOMAIN:-wall.ninja}"
INSTALL_DIR="${INSTALL_DIR:-/opt/blog}"
BINARY_NAME="${BINARY_NAME:-blog-engine}"

# Validate domain parameter
if [ $# -ge 1 ]; then
    DOMAIN="$1"
fi

if [ -z "$DOMAIN" ]; then
    echo "Error: DOMAIN must be set or passed as first argument"
    echo "Usage: $0 <domain> [install_dir] [binary_name]"
    echo "   or: DOMAIN=example.com $0"
    exit 1
fi

# Optional overrides from command line
if [ $# -ge 2 ]; then
    INSTALL_DIR="$2"
fi

if [ $# -ge 3 ]; then
    BINARY_NAME="$3"
fi

echo "=== Blog Server Setup ==="
echo "Domain: $DOMAIN"
echo "Install directory: $INSTALL_DIR"
echo "Binary name: $BINARY_NAME"
echo ""

# Update system
echo "Updating system packages..."
apt-get update && apt-get upgrade -y

# Install only what we need
echo "Installing Caddy and OpenSSH..."
apt-get install -y caddy openssh-server

# Create blog directory
echo "Creating blog directory..."
mkdir -p "$INSTALL_DIR"

# Create systemd service
echo "Creating blog systemd service..."
cat > /etc/systemd/system/blog.service <<EOF
[Unit]
Description=Blog Engine
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/$BINARY_NAME
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

# Create Caddyfile
echo "Creating Caddyfile with logging configuration..."
cat > /etc/caddy/Caddyfile <<EOF
{
    log {
        output file /var/log/caddy/access.log {
            roll_size 100mb
            roll_keep 5
            roll_keep_for 720h
        }
    }
}

$DOMAIN {
    reverse_proxy localhost:3000
    encode gzip

    log {
        output file /var/log/caddy/$DOMAIN.log {
            roll_size 100mb
            roll_keep 5
            roll_keep_for 720h
        }
    }
}

www.$DOMAIN {
    redir https://$DOMAIN{uri} permanent
}
EOF

# Enable and restart services to pick up config changes
echo "Reloading systemd configuration..."
systemctl daemon-reload
echo "Enabling blog service..."
systemctl enable blog
# Note: The blog service will be started by the first deploy
echo "Enabling and reloading Caddy service..."
systemctl enable caddy
systemctl reload-or-restart caddy

echo "=== Setup Complete! ==="
