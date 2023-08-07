# Tunneling
This document use as tunnel for reverse proxy with VPN.


## Dependencies

> On KOOMPI OS(Arch Linux) run the following command to install the packages:
```bash
sudo pacman -S nginx certbot certbot-nginx wireguard-tools

```

> On Debian run the following command to install the packages:
```bash
sudo apt install nginx certbot python-certbot-nginx wireguard

```

## Connect to VPN

### In this configuration, we use wireguard to connect to VPN:

create a file called `wg0.conf` in `/etc/wireguard/` with the following content:

>Note: you should provide your own config file, the following content is just an example.

```
[Interface]
# specify private key for client generated on WireGuard server
PrivateKey = uIvK5DfG1ATMMIFj7VrCqibPrSr2M1axTdei8eCDA0I=
# IP address for VPN interface
Address = 10.0.1.9
DNS = 1.1.1.1

[Peer]
# specify public key for server generated on WireGuard server
PublicKey = sXd+XOPi1My2tsBsBxNRq1UP9FKpB4R20PgXpEI1mzg=
# IP addresses you allow to connect
# on the example below, set WireGuard server's VPN IP address and real local network
#AllowedIPs = 10.0.1.1, 192.168.1.0/24
AllowedIPs = 0.0.0.0/0, ::/0 # Forward all traffic to server
# specify server's global IP address:port
# (acutually, example of IP below is for private range, replace to your own global IP)
EndPoint = 129.222.35.77:51820
```

Then connect to the VPN using the following command:
```bash
sudo nmcli connection import type wireguard file /etc/wireguard/wg0.conf
```

### Configuration
#### Nginx reverse proxy setup
add the following content into /etc/nginx.conf
```
stream {
  include streams-enabled/*;
}
```
This will import and make use of the NGINX stream module.
this module allows for continuous streaming of data in or
out of the machine with all the benefits of having an 
optimized reverse proxy.

```
# Create the streams-enabled folder
sudo mkdir /etc/nginx/streams-enabled
```
Now, inside the newly created directory /etc/nginx/streams-enabled/
create the proxy service file called `selendra-proxy.conf` with the following content:

```
# Change the static IP according to your current VPN IP address
server {
        listen              2435 ssl proxy_protocol;
        proxy_pass          Internal_IP:30333;
        proxy_protocol      on;
}

```
Now, inside folder /etc/nginx/sites-available/
create nginx site file called `selendra-endpoint.conf` with the following content:
```
server {
        listen 80;


        # change server_name to your own domain
        server_name example.org;
        location / {
                proxy_set_header Host $host;
                try_files $uri $uri/ =404;
                proxy_buffering off;
                proxy_pass http://Internal_IP:9944;
                proxy_set_header X-Real-IP $remote_addr;
                proxy_set_header Host $host;
                proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                proxy_http_version 1.1;
                proxy_set_header Upgrade $http_upgrade;
                proxy_set_header Connection "upgrade";
        }
}

```

To enable the site, simply create a symlink: 
```bash
sudo ln -s /etc/nginx/sites-available/selendra-endpoint.conf /etc/nginx/sites-enabled/selendra-endpoint.conf

```

Finally, run certbot with root privilege:
```bash
sudo certbot -d example.org
```

Never forget to reload the nginx server:
```bash
sudo nginx -s reload
```



## Example of Running Archive Node:

```bash
# Change --public-addr IP address according to the VPN IP Address you got
selendra \
        --chain testnet \
        --pruning=archive \
        --base-path /selendra/endpoint \
        --name archive-node \
        --rpc-cors all \
        --ws-external \
        --rpc-external \
        --public-addr=/ip4/PUBLIC_IP/tcp/2435 \
        --listen-addr=/ip4/0.0.0.0/tcp/30333 \
        --ws-max-connections 10000

```



## Example of Running Full Node:

```bash
# Change --public-addr IP address according to the VPN IP Address you got
selendra \
        --chain testnet \
        --base-path /selendra/endpoint \
        --name full-node \
        --rpc-cors all \
        --ws-external \
        --rpc-external \
        --public-addr=/ip4/PUBLIC_IP/tcp/2435 \
        --listen-addr=/ip4/0.0.0.0/tcp/30333 \
        --ws-max-connections 10000

```
