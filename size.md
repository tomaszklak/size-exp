---
title: Making rust _dynamic_ libraries **smaller**, without sacrificing features
author: Tomasz Kłak
---


The end result
---

Real world examples, before:
```
-rwxr-xr-x  1 tomaszklak  staff   4,7M 17 lip 12:15 target/release/libnordtls.dylib*
-rwxr-xr-x  1 tomaszklak  staff    11M 17 lip 12:26 target/release/libtelio.dylib*
```

After:
```
-rwxr-xr-x  1 tomaszklak  staff   3,9M 17 lip 12:14 target/release/libnordtls.dylib*
-rwxr-xr-x  1 tomaszklak  staff   9,4M 17 lip 12:30 target/release/libtelio.dylib*
```

<!-- pause -->

And in the synhetic example:

```
-rwxr-xr-x  1 tomaszklak  staff   957K 17 lip 12:11 target/release/libbig.dylib
-rwxr-xr-x  1 tomaszklak  staff   262K 17 lip 12:11 target/release/libsmall.dylib
```

<!-- end_slide -->

TL;DR
---

Using the same `src/lib.rs` it's enough to:

<!-- pause -->

```
$ diff big/Cargo.toml small/Cargo.toml
7,8c7,8
< crate-type = ["cdylib", "lib"]
< name = "big"
---
> crate-type = ["cdylib"]
> name = "small"
```

<!-- pause -->

It the real world you most likely want to:
1. move the top level crate to `some-name-core`
2. re-export what is actually part of the `C` api in the (now empty) top-level crate
3. use the `core` crate if/where you used the rust api from the top-level crate
3. ...
4. profit!

See here for an example: https://github.com/NordSecurity/libtelio/commit/baee4b48d913af564d8a21f2b9e356334f695ede

<!-- end_slide -->

But why?
---

If we run `cargo` in verbose mode:

<!-- pause -->

```
rustc --crate-name small ../src/lib.rs [...] --crate-type cdylib                  [...] -C lto [...]
rustc --crate-name big   ../src/lib.rs [...] --crate-type cdylib --crate-type lib [...]        [...]
```

When to enable LTO is described in https://rust-lang.github.io/rfcs/1510-cdylib.html

<!-- end_slide -->

Can `lto` explain the effect?
---

Let's create a small lib without `lto`:

```
$ diff small/Cargo.toml small_no_lto/Cargo.toml
8c8
< name = "small"
---
> name = "small_no_lto"
18c18
< lto = true
---
> lto = false
```

<!-- pause -->

```
-rwxr-xr-x  1 tomaszklak  staff   957K 17 lip 13:18 target/release/libbig.dylib
-rwxr-xr-x  1 tomaszklak  staff   295K 17 lip 13:18 target/release/libsmall_no_lto.dylib
-rwxr-xr-x  1 tomaszklak  staff   262K 17 lip 13:17 target/release/libsmall.dylib
```

Lto is responsible for minority of the gain.

<!-- end_slide -->

cdylib vs cdylib + lib
---

From the rustc source:

```rust
fn crate_export_threshold(crate_type: CrateType) -> SymbolExportLevel {
    match crate_type {
        CrateType::Executable | CrateType::StaticLib | CrateType::ProcMacro | CrateType::Cdylib => {
            SymbolExportLevel::C
        }
        CrateType::Rlib | CrateType::Dylib | CrateType::Sdylib => SymbolExportLevel::Rust,
    }
}

pub fn crates_export_threshold(crate_types: &[CrateType]) -> SymbolExportLevel {
    if crate_types
        .iter()
        .any(|&crate_type| crate_export_threshold(crate_type) == SymbolExportLevel::Rust)
    {
        SymbolExportLevel::Rust
    } else {
        SymbolExportLevel::C
    }
}
```

To have rust style visibility of symbols, it's enough for only one of the crate types to be of the rust kind.

<!-- end_slide -->

cdylib vs cdylib + lib
---

Again, quoting RFC 1510:

<!-- pause -->
> Symbol visibility - rdylibs will expose all symbols as rlibs do, cdylibs will expose symbols as executables do.

<!-- pause -->
> This means that pub fn foo() {} will not be an exported symbol, but #[no_mangle] pub extern fn foo() {} will be an exported symbol.

<!-- pause -->
> Note that the compiler will also be at liberty to pass extra flags to the linker to actively hide exported Rust symbols from linked libraries.
