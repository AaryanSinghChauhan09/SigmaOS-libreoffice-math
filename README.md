# SigmaOS-libreoffice-math

> **LibreOffice Math — Formula Editor** — Mathematical formula notation and rendering (LibreOffice Math parity).

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) ecosystem.  
Parity / inspiration: **LibreOffice**

## Quickstart

```bash
cargo test    # run deterministic AAA unit tests
cargo check   # verify compilation
```

## Component Role

This crate is a component of the SigmaOS LibreOffice Math suite:
- Fully real Rust implementation (no stubs, no placeholders)
- Fail-fast validation with typed domain errors
- Deterministic unit tests (Arrange-Act-Assert pattern)
- Single Responsibility, Modular, Loose Coupling

## API

```rust
use sigmaos_libreoffice_math::*;
// See src/lib.rs for the full public API surface
```

## Documentation

Full documentation: [SigmaOS GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

## License

LGPL-2.1
