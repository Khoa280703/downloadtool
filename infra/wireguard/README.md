# WireGuard VPN Setup

This directory contains WireGuard configuration templates for secure communication between VPS and Home Server.

## Network Topology

```
┌─────────────┐         WireGuard VPN          ┌─────────────────┐
│     VPS     │◄──────────────────────────────►│   Home Server   │
│   10.0.0.1  │         UDP 51820              │    10.0.0.2     │
│  (Public)   │                                │  (Behind NAT)   │
└─────────────┘                                └─────────────────┘
```

## Configuration Steps

### 1. Generate Keys

On both VPS and Home Server, generate WireGuard keys:

```bash
# Generate private key
wg genkey > private.key

# Generate public key from private key
wg pubkey < private.key > public.key

# Set proper permissions
chmod 600 private.key
```

### 2. Configure Home Server

1. Copy `homeserver.conf` to `/etc/wireguard/wg0.conf`
2. Replace `HOMESERVER_PRIVATE_KEY_HERE` with actual private key
3. Replace `VPS_PUBLIC_KEY_HERE` with VPS public key
4. Start WireGuard:
   ```bash
   sudo wg-quick up wg0
   sudo systemctl enable wg-quick@wg0
   ```

### 3. Configure VPS

1. Copy `vps.conf` to `/etc/wireguard/wg0.conf`
2. Replace `VPS_PRIVATE_KEY_HERE` with actual private key
3. Replace `HOMESERVER_PUBLIC_KEY_HERE` with Home Server public key
4. Replace `HOMESERVER_IP` with Home Server public IP or DDNS hostname
5. Start WireGuard:
   ```bash
   sudo wg-quick up wg0
   sudo systemctl enable wg-quick@wg0
   ```

### 4. Verify Connection

From VPS:
```bash
ping 10.0.0.2
```

From Home Server:
```bash
ping 10.0.0.1
```

Check WireGuard status:
```bash
sudo wg show
```

## Firewall Rules

### Home Server (if using UFW)

```bash
# Allow WireGuard port
sudo ufw allow 51820/udp

# Allow traffic from VPN subnet
sudo ufw allow from 10.0.0.0/24
```

### VPS (if using UFW)

```bash
# Allow WireGuard
sudo ufw allow 51820/udp

# Allow traffic from VPN subnet
sudo ufw allow from 10.0.0.0/24
```

## Troubleshooting

### Check if WireGuard module is loaded
```bash
lsmod | grep wireguard
```

If not loaded, try:
```bash
sudo modprobe wireguard
```

Or use userspace implementation:
```bash
sudo apt install wireguard-go
```

### Debug connection issues
```bash
# View WireGuard logs
sudo dmesg | grep wireguard

# Check interface status
ip addr show wg0

# Monitor handshake
sudo wg show wg0 latest-handshakes
```

## Service Addresses

After VPN is established:
- VPS API: `10.0.0.1:3000`
- GPU Worker gRPC: `10.0.0.2:50051`
