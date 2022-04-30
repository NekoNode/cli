# Nekocli

NekoNode is a website providing useful services. `Nekocli` is the command-line interface for those services.

## Installation

With Rust toolchain:

```bash
cargo install nekocli
```

## Send

NekoSend is a service for sharing files with your friends easily. You could use `nekocli` or simply `curl`.

```bash
# Send example.txt to friends
nekocli send ./example.txt
# Or use curl
curl -F "file=@example.txt" https://jumao.cli.nekonode.com/api/send

# Receive file with a share code
nekocli receive SHARE_CODE
# Or use curl
curl "https://jumao.cli.nekonode.com/api/send?code=SHARE_CODE"
```

