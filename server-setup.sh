#!/bin/bash
set -e

# Extract domain and redirect_domains from site.toml if available
TOML_DOMAIN=""
REDIRECT_DOMAINS=()
if [ -f "site.toml" ]; then
    # Extract domain line, remove https:// and quotes
    TOML_DOMAIN=$(grep '^domain = ' site.toml | sed 's/domain = "//;s/"//;s|https://||;s|http://||')

    # Extract redirect_domains array
    if grep -q '^redirect_domains = ' site.toml; then
        # Extract the array contents, remove brackets and quotes, split by comma
        REDIRECT_LINE=$(grep '^redirect_domains = ' site.toml | sed 's/redirect_domains = \[//;s/\]//')
        while IFS=',' read -ra DOMAINS; do
            for domain in "${DOMAINS[@]}"; do
                # Trim whitespace and quotes
                domain=$(echo "$domain" | sed 's/^[[:space:]]*"//;s/"[[:space:]]*$//')
                if [ -n "$domain" ]; then
                    REDIRECT_DOMAINS+=("$domain")
                fi
            done
        done <<< "$REDIRECT_LINE"
    fi
fi

# Configuration - can be overridden via environment variables
DOMAIN="${DOMAIN:-${TOML_DOMAIN}}"
INSTALL_DIR="${INSTALL_DIR:-/opt/blog}"
BINARY_NAME="${BINARY_NAME:-blog-engine}"

# Validate domain parameter
if [ $# -ge 1 ]; then
    DOMAIN="$1"
fi

if [ -z "$DOMAIN" ]; then
    echo "Error: DOMAIN must be set via one of:"
    echo "  1. site.toml (domain field)"
    echo "  2. Command line: $0 <domain> [install_dir] [binary_name]"
    echo "  3. Environment: DOMAIN=example.com $0"
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
EOF

# Add redirect blocks for each redirect domain
for redirect_domain in "${REDIRECT_DOMAINS[@]}"; do
    cat >> /etc/caddy/Caddyfile <<EOF

$redirect_domain {
    redir https://$DOMAIN{uri} permanent
}
EOF
done

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
