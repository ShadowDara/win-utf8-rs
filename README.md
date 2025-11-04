# win-utf8-rs

Simple Cargo Create with an function to enable UTF-8 in the Windows
Terminal


add
```toml
[target.'cfg(windows)'.dependencies]
win-utf8-rs = "0.2.1"
```

Use
```rust
#[cfg(windows)]
enable_utf8();
```
