# gba-rs

A crate for running Rust code on the Game Boy Advance. *Highly experimental*.

It consists of two parts: `gba-boot`, which initializes parts of memory and jumps to the entry point, and a hardware abstraction library `gba`.

## Requirements
- `devkitARM` or `arm-none-eabi-gcc`
- `cargo-xbuild`

## Usage
1. Install the requirements.
2. Extract the header from an existing GBA ROM and place it in `builder/header.bin` using this command.
```
dd bs=1 count=192 if=rom.gba of=header.bin
```
3. Create a new example in the `gba-examples` crate.
4. Create a function with no arguments that returns `!` (the bottom type) by entering an endless loop or similar, and tag it with using the `gba_boot::entry` attribute.
```rust
use gba_boot::entry;

#[entry]
fn main() -> ! {
    loop {
        // ...
    }
}
```

5. To build the ROM, `cd` to the `builder` crate and run `cargo run -- --example <example-name> --release`. It will be output in the root as `target/thumbv4t-gba-cart/release/examples/<example-name>.gba`.

## Limitations
- The generated code is almost certainly not as optimized as it could be.
- There is no ability to switch between ARM and Thumb modes on a per-function basis in Rust code. The `thumb` LLVM feature would have to be added to a [whitelist of features](https://github.com/rust-lang/rust/blob/a2ec156a5b5d58f2a73bf21b1fe037b6ac1cf5cc/src/librustc_codegen_llvm/llvm_util.rs#L96) usable with `#[target_feature]` in `rustc`.
- `lld` cannot be used for linking since it doesn't fully support the `armv4t` target, as seen [here](https://github.com/llvm/llvm-project/blob/317f9e7ae77bffecc2cd3cbf08da86e6563ee699/lld/test/ELF/arm-blx-v4t.s#L5). Also, it doesn't seem to fully support some of the options in the linker script used.
- More of the library functions should probably be marked `unsafe` than there are currently.
- Certain memory access patterns that would be easier in C or C++ are far more difficult in Rust. For example, [object attribute data](http://www.akkit.org/info/gbatek.htm#lcdobjoamattributes) in OAM memory is interleaved such that it can simultaneously be accessed as both 128 entries of 8-byte attribute data and 32 entries of 32-byte affine data. Modifying both forms of memory at once might not even be possible without using unsafe Rust. See [this object affine example](gba-examples/examples/tonc-11.rs) for an example that does this.

## Credits
- The linker script and boot sequence are adapted from `devkitARM`.
- Many of the examples are adapted from or inspired by the [Tonc](https://www.coranac.com/tonc/text/toc.htm) GBA development tutorial.
- The `builder` program was modified from the version in the [bootloader](https://github.com/rust-osdev/bootloader) crate.
- [GBATek](http://www.akkit.org/info/gbatek.htm) was used as a reference.
