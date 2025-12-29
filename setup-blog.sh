#!/bin/bash
set -e

echo "=== Blog Server Setup ==="

# Update system
apt update && apt upgrade -y

# Install only what we need
apt install -y caddy openssh-server

# Create blog directory
mkdir -p /opt/blog

# Create systemd service
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

# Enable services
systemctl daemon-reload
systemctl enable --now blog
systemctl enable --now caddy

echo "=== Setup Complete! ==="
