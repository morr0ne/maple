# 🍁 Maple

_An experimental Linux environment in Rust_

## What is this?

Maple is a from-scratch reimplementation of a Linux environment with the goal of creating an OS-like targetable system. The goal is for this to not be a distro but something you can actually target directly, kinda like Android but smaller and more personal. The target in fact is `x86_64-linux-maple`.

The goal rn is to create a crt0, libc and dynamic linker. The project will also include its own bespoke RAM init. For obvious reasons everything is highly opinionated since it's meant to be a battery-included system; everything you need bundled together nicely.

One of the design goals is to provide interfaces for modern languages. Direct syscalls are forbidden for non-privileged programs, but libsystem provides interfaces that more closely resemble what a modern programming language expects. A libc implementation is also provided for other languages, although they are encouraged to use the libsystem since it's designed to be friendlier.

A basic allocator is currently in progress as well as a dynamic linker. While still being in a POC phase, everything is linked statically. Fully static executables will be completely forbidden in the future as everyone has to at the very least link to the crt0 and either libc or libsystem if they wanna interact with the system. The dynamic linker must be included in the interp section of the file.

I don't claim that all the goals have been achieved yet, but those are what they should be. Feel free to provide feedback; an architecture file will be provided soon-ish.

### What's working so far

Libc currently implements:

- Printf family functions (`printf`, `fprintf`, etc.)
- Basic I/O operations
- Standard streams (`stdin`, `stdout`, `stderr`)
- Program startup (`_start`)

This is very basic but it's enough to write a full-standing C binary that will run on Linux without any additional tools required. The goal is to create our own target that will eventually support std to write programs.

## Building

Building requires nightly Rust for various unstable features. You should use rustup to use the version pinned by the project - any other version is not supported and might cause issues.

```bash
cargo build --release
```

Building in debug mode should be avoided at least for the time being.

## License

This project is licensed under the [EUPL](https://eupl.eu/). For more
information, please see the [LICENSE](LICENSE) file.
