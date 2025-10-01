#!/bin/bash

# One-Click Selendra RPC Deployment Script
# Usage: ./deploy-selendra-rpc [domain] [email]

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
DOMAIN=${1:-"rpc.selendra.org"}
EMAIL=${2:-"admin@selendra.org"}
NODE_NAME=${3:-$(hostname)}  # Use hostname as default, can be overridden
COMPOSE_FILE="docker-compose.yml"
NGINX_CONFIG="nginx.conf"
SKIP_SSL=false

# Banner
echo -e "${BLUE}"
echo "=================================================================="
echo "    🚀 Selendra RPC One-Click Deployment Script"
echo "=================================================================="
echo -e "Domain: ${GREEN}$DOMAIN${NC}"
echo -e "Email:  ${GREEN}$EMAIL${NC}"
echo -e "Node:   ${GREEN}$NODE_NAME${NC}"
echo "=================================================================="
echo -e "${NC}"

# Functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

install_docker() {
    log_info "Installing Docker and Docker Compose..."
    
    # Update package list
    apt-get update -qq
    
    # Install required packages
    apt-get install -y \
        apt-transport-https \
        ca-certificates \
        curl \
        gnupg \
        lsb-release \
        dig \
        netstat-nat
    
    # Add Docker's official GPG key
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
    
    # Add Docker repository
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
    
    # Update package list
    apt-get update -qq
    
    # Install Docker
    apt-get install -y docker-ce docker-ce-cli containerd.io
    
    # Install Docker Compose
    curl -L "https://github.com/docker/compose/releases/download/v2.24.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
    chmod +x /usr/local/bin/docker-compose
    
    # Start Docker service
    systemctl start docker
    systemctl enable docker
    
    log_success "Docker and Docker Compose installed"
}

check_requirements() {
    log_info "Checking system requirements..."
    
    # Check if running as root (required for fast deployment)
    if [ "$EUID" -ne 0 ]; then
        log_error "This script must be run as root for automatic installation"
        echo "Run with: sudo $0 $*"
        exit 1
    fi
    
    # Install missing tools
    if ! command -v dig &> /dev/null; then
        log_info "Installing dig..."
        apt-get update -qq && apt-get install -y dnsutils
    fi
    
    if ! command -v netstat &> /dev/null; then
        log_info "Installing netstat..."
        apt-get update -qq && apt-get install -y net-tools
    fi
    
    if ! command -v curl &> /dev/null; then
        log_info "Installing curl..."
        apt-get update -qq && apt-get install -y curl
    fi
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_warning "Docker not found - installing automatically..."
        install_docker
    else
        log_success "Docker found"
    fi
    
    # Check Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        log_warning "Docker Compose not found - installing automatically..."
        curl -L "https://github.com/docker/compose/releases/download/v2.24.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
        chmod +x /usr/local/bin/docker-compose
        log_success "Docker Compose installed"
    else
        log_success "Docker Compose found"
    fi
    
    # Test Docker
    if ! docker ps &> /dev/null; then
        log_info "Starting Docker service..."
        systemctl start docker
        systemctl enable docker
        sleep 3
    fi
    
    if ! docker ps &> /dev/null; then
        log_error "Docker is not working properly"
        exit 1
    fi
    
    log_success "System requirements check passed"
}

validate_inputs() {
    log_info "Validating inputs..."
    
    # Validate domain format
    if [[ ! "$DOMAIN" =~ ^[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$ ]]; then
        log_error "Invalid domain format: $DOMAIN"
        echo "Domain must be a valid FQDN (e.g., rpc.example.com)"
        exit 1
    fi
    
    # Check for placeholder domains
    if [[ "$DOMAIN" == *"example.com"* ]] || [[ "$DOMAIN" == *"yourdomain.com"* ]] || [[ "$DOMAIN" == *"domain.com"* ]]; then
        log_error "Please specify a real domain, not a placeholder"
        echo "Example: ./deploy-selendra-rpc rpc.selendra.org admin@selendra.org"
        exit 1
    fi
    
    # Validate email format
    if [[ ! "$EMAIL" =~ ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$ ]]; then
        log_error "Invalid email format: $EMAIL"
        echo "Email must be a valid email address (e.g., admin@example.com)"
        exit 1
    fi
    
    # Check for placeholder emails
    if [[ "$EMAIL" == *"domain.com"* ]] || [[ "$EMAIL" == *"example.com"* ]] || [[ "$EMAIL" == *"yourdomain.com"* ]]; then
        log_error "Please specify a real email, not a placeholder"
        echo "Example: ./deploy-selendra-rpc rpc.selendra.org admin@selendra.org"
        exit 1
    fi
    
    log_success "Input validation passed"
}

validate_domain() {
    log_info "Validating domain setup..."
    
    # Check if domain exists (has any DNS records)
    if ! dig +short $DOMAIN NS > /dev/null 2>&1 && ! dig +short $DOMAIN A > /dev/null 2>&1; then
        log_error "Domain $DOMAIN does not exist or has no DNS records"
        echo "Please ensure the domain is registered and has DNS configured"
        exit 1
    fi
    
    # Get server IP
    SERVER_IP=$(curl -s --max-time 10 ip.me || curl -s --max-time 10 ifconfig.me || curl -s --max-time 10 ipinfo.io/ip)
    if [ -z "$SERVER_IP" ]; then
        log_error "Failed to get server public IP"
        exit 1
    fi
    
    log_info "Server IP: $SERVER_IP"
    
    # Check domain resolution
    DOMAIN_IP=$(dig +short $DOMAIN A | tail -n1)
    if [ -z "$DOMAIN_IP" ]; then
        log_warning "Domain $DOMAIN does not have an A record"
        echo ""
        echo "To fix this, add a DNS A record:"
        echo "  Type: A"
        echo "  Name: $(echo $DOMAIN | cut -d. -f1)"
        echo "  Value: $SERVER_IP"
        echo "  TTL: 300 (or your provider's default)"
        echo ""
        read -p "Continue anyway? The script will set up everything except SSL. (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Deployment cancelled. Please configure DNS and try again."
            exit 1
        fi
        SKIP_SSL=true
        log_warning "Continuing without SSL setup"
        return
    fi
    
    log_info "Domain IP: $DOMAIN_IP"
    
    # Check if it's a valid IP
    if [[ ! "$DOMAIN_IP" =~ ^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}$ ]]; then
        log_error "Domain resolves to invalid IP: $DOMAIN_IP"
        exit 1
    fi
    
    # Compare IPs
    if [ "$SERVER_IP" != "$DOMAIN_IP" ]; then
        log_warning "Domain points to different IP"
        echo "  Expected: $SERVER_IP (this server)"
        echo "  Current:  $DOMAIN_IP"
        echo ""
        echo "To fix this, update your DNS A record:"
        echo "  Type: A"
        echo "  Name: $(echo $DOMAIN | cut -d. -f1)"
        echo "  Value: $SERVER_IP"
        echo ""
        read -p "Continue anyway? SSL will likely fail. (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            log_info "Deployment cancelled. Please update DNS and try again."
            exit 1
        fi
        SKIP_SSL=true
        log_warning "Continuing without SSL setup"
        return
    fi
    
    # Test if domain is actually reachable
    log_info "Testing domain connectivity..."
    if timeout 10 nc -z $DOMAIN_IP 80 2>/dev/null; then
        log_info "Domain is reachable on port 80"
    else
        log_warning "Domain may not be reachable on port 80"
    fi
    
    log_success "Domain validation passed"
}

check_ports() {
    log_info "Checking required ports..."
    
    # Stop conflicting services
    if netstat -tlnp 2>/dev/null | grep ":80 " > /dev/null; then
        log_warning "Port 80 is in use, stopping conflicting services"
        systemctl stop apache2 nginx 2>/dev/null || true
        # Kill any processes using port 80
        fuser -k 80/tcp 2>/dev/null || true
    fi
    
    if netstat -tlnp 2>/dev/null | grep ":443 " > /dev/null; then
        log_warning "Port 443 is in use, stopping conflicting services"
        systemctl stop apache2 nginx 2>/dev/null || true
        # Kill any processes using port 443
        fuser -k 443/tcp 2>/dev/null || true
    fi
    
    # Wait a moment for ports to be released
    sleep 2
    
    log_success "Port check completed"
}

cleanup_existing() {
    log_info "Cleaning up existing deployments..."
    
    # Stop and remove existing containers
    docker-compose down 2>/dev/null || true
    docker stop selendra-rpc-main selendra-nginx selendra-certbot 2>/dev/null || true
    docker rm selendra-rpc-main selendra-nginx selendra-certbot 2>/dev/null || true
    
    log_success "Cleanup completed"
}

create_docker_compose() {
    log_info "Creating Docker Compose configuration..."
    
    cat > $COMPOSE_FILE << EOF
version: '3.8'

services:
  selendra-rpc:
    image: image.koompi.org/library/selendra-rpc:latest
    container_name: selendra-rpc-main
    restart: unless-stopped
    volumes:
      - selendra_data:/data
    command: [
      "--chain", "selendra",
      "--base-path", "/data/selendradb",
      "--name", "$NODE_NAME",
      "--rpc-port", "9933",
      "--port", "40333",
      "--no-mdns",
      "--pool-limit", "1024",
      "--db-cache", "4096",
      "--max-runtime-instances", "8",
      "--runtime-cache-size", "4",
      "--rpc-external",
      "--rpc-cors", "all",
      "--rpc-max-connections", "2000"
    ]
    networks:
      - selendra_net
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9933"]
      interval: 30s
      timeout: 10s
      retries: 3

  nginx:
    image: nginx:alpine
    container_name: selendra-nginx
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - /etc/letsencrypt:/etc/letsencrypt:ro
      - /var/www/certbot:/var/www/certbot
    networks:
      - selendra_net
    depends_on:
      - selendra-rpc

networks:
  selendra_net:
    driver: bridge

volumes:
  selendra_data:
    driver: local
EOF
    
    log_success "Docker Compose file created"
}

create_nginx_config() {
    log_info "Creating NGINX configuration..."
    
    cat > $NGINX_CONFIG << EOF
events {
    worker_connections 2048;
}

http {
    limit_req_zone \$binary_remote_addr zone=rpc:10m rate=10r/s;
    
    server {
        listen 80;
        server_name $DOMAIN;
        
        location /.well-known/acme-challenge/ {
            root /var/www/certbot;
        }
        
        location /health {
            return 200 "healthy\\n";
            add_header Content-Type text/plain;
        }
        
        location / {
            proxy_pass http://selendra-rpc:9933;
            proxy_set_header Host \$host;
            proxy_set_header X-Real-IP \$remote_addr;
            proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
            
            proxy_http_version 1.1;
            proxy_set_header Upgrade \$http_upgrade;
            proxy_set_header Connection "upgrade";
            
            proxy_connect_timeout 60s;
            proxy_send_timeout 120s;
            proxy_read_timeout 120s;
            proxy_buffering off;
            
            add_header Access-Control-Allow-Origin *;
            add_header Access-Control-Allow-Methods "GET, POST, OPTIONS";
            add_header Access-Control-Allow-Headers "Content-Type, Authorization";
            
            limit_req zone=rpc burst=20 nodelay;
        }
    }
}
EOF
    
    log_success "NGINX configuration created"
}

start_services() {
    log_info "Starting Selendra RPC services..."
    
    # Pull latest image
    docker-compose pull
    
    # Start services
    docker-compose up -d
    
    # Wait for services to be ready
    log_info "Waiting for services to start..."
    sleep 30
    
    # Check if RPC is responding
    for i in {1..12}; do
        if curl -s http://localhost/health > /dev/null 2>&1; then
            log_success "Selendra RPC is running"
            break
        fi
        log_info "Waiting for RPC to be ready... ($i/12)"
        sleep 10
    done
    
    # Final health check
    if ! curl -s http://localhost/health > /dev/null 2>&1; then
        log_error "RPC failed to start properly"
        echo "Check logs with: docker-compose logs"
        exit 1
    fi
}

setup_ssl() {
    if [ "$SKIP_SSL" = true ]; then
        log_warning "Skipping SSL setup due to DNS issues"
        return
    fi
    
    log_info "Setting up SSL certificate..."
    
    # Create directories
    mkdir -p /etc/letsencrypt /var/www/certbot
    chmod 755 /var/www/certbot
    
    # Get SSL certificate
    docker run --rm \
        -v /etc/letsencrypt:/etc/letsencrypt \
        -v /var/www/certbot:/var/www/certbot \
        certbot/certbot \
        certonly --webroot \
        --webroot-path=/var/www/certbot \
        --email $EMAIL \
        --agree-tos \
        --no-eff-email \
        --force-renewal \
        -d $DOMAIN
    
    if [ $? -eq 0 ]; then
        log_success "SSL certificate obtained"
        update_nginx_ssl
    else
        log_warning "SSL certificate failed, continuing with HTTP"
        log_info "You can get SSL later with: docker run --rm -v /etc/letsencrypt:/etc/letsencrypt -v /var/www/certbot:/var/www/certbot certbot/certbot certonly --webroot --webroot-path=/var/www/certbot --email $EMAIL --agree-tos --no-eff-email -d $DOMAIN"
    fi
}

update_nginx_ssl() {
    log_info "Updating NGINX for SSL..."
    
    cat > $NGINX_CONFIG << EOF
events {
    worker_connections 2048;
}

http {
    limit_req_zone \$binary_remote_addr zone=rpc:10m rate=10r/s;
    
    server {
        listen 80;
        server_name $DOMAIN;
        
        location /.well-known/acme-challenge/ {
            root /var/www/certbot;
        }
        
        location / {
            return 301 https://\$server_name\$request_uri;
        }
    }
    
    server {
        listen 443 ssl http2;
        server_name $DOMAIN;
        
        ssl_certificate /etc/letsencrypt/live/$DOMAIN/fullchain.pem;
        ssl_certificate_key /etc/letsencrypt/live/$DOMAIN/privkey.pem;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers HIGH:!aNULL:!MD5;
        
        add_header X-Frame-Options DENY;
        add_header X-Content-Type-Options nosniff;
        add_header X-XSS-Protection "1; mode=block";
        add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
        
        location /health {
            access_log off;
            return 200 "healthy\\n";
            add_header Content-Type text/plain;
        }
        
        location / {
            proxy_pass http://selendra-rpc:9933;
            proxy_set_header Host \$host;
            proxy_set_header X-Real-IP \$remote_addr;
            proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto \$scheme;
            
            proxy_http_version 1.1;
            proxy_set_header Upgrade \$http_upgrade;
            proxy_set_header Connection "upgrade";
            
            proxy_connect_timeout 60s;
            proxy_send_timeout 120s;
            proxy_read_timeout 120s;
            proxy_buffering off;
            
            add_header Access-Control-Allow-Origin *;
            add_header Access-Control-Allow-Methods "GET, POST, OPTIONS";
            add_header Access-Control-Allow-Headers "Content-Type, Authorization";
            
            limit_req zone=rpc burst=20 nodelay;
            
            if (\$request_method = 'OPTIONS') {
                add_header Access-Control-Allow-Origin *;
                add_header Access-Control-Allow-Methods "GET, POST, OPTIONS";
                add_header Access-Control-Allow-Headers "Content-Type, Authorization";
                add_header Access-Control-Max-Age 86400;
                add_header Content-Length 0;
                return 204;
            }
        }
    }
}
EOF
    
    # Reload nginx
    docker-compose restart nginx
    log_success "NGINX updated with SSL"
}

setup_auto_renewal() {
    log_info "Setting up SSL auto-renewal..."
    
    # Create renewal script
    cat > renew-ssl.sh << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
docker run --rm -v /etc/letsencrypt:/etc/letsencrypt -v /var/www/certbot:/var/www/certbot certbot/certbot renew --quiet
docker-compose restart nginx
EOF
    
    chmod +x renew-ssl.sh
    
    # Add to crontab
    (crontab -l 2>/dev/null; echo "0 12 * * * $(pwd)/renew-ssl.sh") | crontab -
    
    log_success "Auto-renewal configured"
}

run_tests() {
    log_info "Running final tests..."
    
    # Test HTTP
    if curl -s http://$DOMAIN/health > /dev/null; then
        log_success "HTTP endpoint working"
    else
        log_warning "HTTP endpoint not responding"
    fi
    
    # Test HTTPS (if SSL was set up)
    if [ -f "/etc/letsencrypt/live/$DOMAIN/fullchain.pem" ]; then
        if curl -s https://$DOMAIN/health > /dev/null; then
            log_success "HTTPS endpoint working"
        else
            log_warning "HTTPS endpoint not responding"
        fi
    fi
    
    # Test RPC
    RPC_RESPONSE=$(curl -s -H "Content-Type: application/json" \
        -d '{"id":1,"jsonrpc":"2.0","method":"system_chain","params":[]}' \
        http://$DOMAIN 2>/dev/null || echo "")
    
    if echo "$RPC_RESPONSE" | grep -q "result"; then
        log_success "RPC endpoint working"
    else
        log_warning "RPC endpoint may not be ready yet"
    fi
}

show_summary() {
    echo -e "\n${GREEN}=================================================================="
    echo "🎉 Selendra RPC Deployment Complete!"
    echo "=================================================================="
    echo -e "Domain: ${BLUE}$DOMAIN${NC}"
    echo -e "Node Name: ${BLUE}$NODE_NAME${NC}"
    echo -e "Status: ${GREEN}Running${NC}"
    echo ""
    echo "📍 Endpoints:"
    echo "  HTTP:  http://$DOMAIN"
    if [ -f "/etc/letsencrypt/live/$DOMAIN/fullchain.pem" ]; then
        echo "  HTTPS: https://$DOMAIN"
    fi
    echo "  Health: http://$DOMAIN/health"
    echo ""
    echo "🧪 Test RPC:"
    echo "  curl -H \"Content-Type: application/json\" \\"
    echo "       -d '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"system_chain\",\"params\":[]}' \\"
    echo "       http://$DOMAIN"
    echo ""
    echo "📊 Management Commands:"
    echo "  View logs:    docker-compose logs -f"
    echo "  Restart:      docker-compose restart"
    echo "  Stop:         docker-compose down"
    echo "  Update:       docker-compose pull && docker-compose up -d"
    echo ""
    echo "🔧 Files created:"
    echo "  - docker-compose.yml"
    echo "  - nginx.conf"
    echo "  - renew-ssl.sh"
    echo -e "==================================================================${NC}"
}

# Main execution
main() {
    validate_inputs
    check_requirements
    validate_domain
    check_ports
    cleanup_existing
    create_docker_compose
    create_nginx_config
    start_services
    
    # Try SSL setup (continue even if it fails)
    if curl -s http://$DOMAIN/.well-known/acme-challenge/ &>/dev/null || curl -s http://$DOMAIN/health &>/dev/null; then
        setup_ssl
        if [ "$SKIP_SSL" != true ]; then
            setup_auto_renewal
        fi
    else
        log_warning "Skipping SSL setup - domain not accessible"
    fi
    
    run_tests
    show_summary
}

# Handle interruption
trap 'log_error "Deployment interrupted"; exit 1' INT

# Show usage if help requested
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    echo "Usage: $0 [domain] [email] [node_name]"
    echo ""
    echo "Examples:"
    echo "  $0                                           # Uses defaults"
    echo "  $0 rpc.mysite.com                           # Custom domain"
    echo "  $0 rpc.mysite.com admin@mysite.com          # Custom domain and email"
    echo "  $0 rpc.mysite.com admin@mysite.com my-node  # Custom everything"
    echo ""
    echo "Defaults:"
    echo "  Domain:    rpc.selendra.org"
    echo "  Email:     admin@selendra.org"
    echo "  Node Name: $(hostname)"
    echo ""
    echo "Prerequisites:"
    echo "  - Domain must be registered and have DNS configured"
    echo "  - DNS A record should point to this server's IP"
    echo "  - Script must be run as root"
    echo ""
    echo "The script will validate everything before starting deployment."
    exit 0
fi

# Run main function
main
