```bash
rustc main.rs -o main \
    --target x86_64-unknown-none \
    -C opt-level=3 \
    -C debuginfo=none \
    -C strip=symbols \
    -C lto=fat \
    -C panic=abort \
    -C codegen-units=1
```

## References

<https://gitlab.com/x86-psABIs/x86-64-ABI>

<https://syscalls.mebeim.net/?table=x86/64/x64/latest>
