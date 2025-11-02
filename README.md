# win-utf8-rs

Simple Cargo Create with an function to enable UTF-8 in the Windows
Terminal


add
```toml
[target.'cfg(windows)'.dependencies]
windows = { version = "0.62.1", features = ["Win32_System_Console"] }
```

Use
```rust
#[cfg(windows)]
enable_utf8();
```
