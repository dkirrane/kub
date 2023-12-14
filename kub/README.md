https://github.com/fede1024/rust-rdkafka/tree/master/examples
https://github.com/confluentinc/examples/blob/master/clients/cloud/rust/README.md

# Troubleshooting

Cargo build fails on windows
`error: failed to run custom build command for openssl-sys v0.9.93`
or

```
  Could not find directory of OpenSSL installation, and this `-sys` crate cannot
  proceed without this knowledge. If OpenSSL is installed and this crate had
  trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
  compilation process.
```

## Windows Fix

1. https://stackoverflow.com/a/70949736/1086117

```
OPENSSL_DIR=C:\Program Files\OpenSSL-Win64
OPENSSL_NO_VENDOR=1
RUSTFLAGS=-Ctarget-feature\=+crt-static
SSL_CERT=C:\OpenSSL-Win64\certs\cacert.pem
```

2. Install VS Build Tools and add cmake.exe to the PATH
   choco install cmake
3. C:\ProgramData\chocolatey\lib\cmake

# Ubuntu WSL Fix

```
sudo apt-get install pkg-config libssl-dev cmake
cargo build
```