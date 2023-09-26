#!/bin/bash
#
# Future Work: Implement friendly creating systemd service with user-input, features "--base-path, --name"

version=("0.2.1")

## Fetch Selendra Binary
wget https://github.com/selendra/selendra/releases/download/$version/selendra

## Make Selendra binary executable
chmod +x selendra

## copy selendra systemd service to systemd directory
sudo cp ./packaging/selendra.service /etc/systemd/system/selendra.service

## Enable selendra service
sudo systemctl enable selendra.service


