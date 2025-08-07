# 1. Install `dnsmasq`

```bash
brew install dnsmasq
```

> [!WARNING]
> This command will overwrite an existing configuration file.

```bash
cat >$(brew --prefix)/etc/dnsmasq.conf <<EOL
# Route all *.internal addresses to localhost
address=/internal/127.0.0.1
address=/internal/::1

# Don't read /etc/resolv.conf or any other configuration files.
no-resolv

# Never forward plain names (without a dot or domain part)
domain-needed

# Never forward addresses in the non-routed address spaces.
bogus-priv
EOL
```

```bash
sudo brew services start dnsmasq
```

# 2. Configure macOS For \*.internal Domains

```bash
sudo mkdir -p /etc/resolver
sudo bash -c 'echo "nameserver 127.0.0.1" > /etc/resolver/internal'
```

# 3. Test the Configuration

```bash
dig +short something.internal A @127.0.0.1
# 127.0.0.1

dig +short something.internal AAAA @127.0.0.1
# ::1
```

# 4. Install Proxy Certificate

```bash
security add-trusted-cert -k ~/Library/Keychains/login.keychain-db proxy/data/caddy/pki/authorities/local/root.crt
```
