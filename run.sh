#!/bin/bash


selendra-node \
    --validator \
    --public-validator-addresses "<publice-ip>:30343" \
    --validator-port "30343" \
    --chain "selendra" \
    --base-path "<database>" \
    --name "<name>" \
    --rpc-port 9933 \
    --port 40333 \
    --no-mdns \
    --pool-limit 1024 \
    --db-cache 1024 \
    --runtime-cache-size 2 \
    --max-runtime-instances 8 
