# color-wasm

> **This project has been superseded by [color-rs](https://github.com/jparnzen/color-rs), a Cargo workspace that extracts the core library into a dedicated crate and adds a CLI tool alongside the WASM bindings. color-wasm is archived here as the origin of that work.**

A Rust/WASM library implementing CSS Color 4 color space conversions and gamut mapping, compiled to WebAssembly for browser use.

## Origin

color-wasm was built alongside [CSS Color 4 gamut mapping contributions to Firefox](https://bugzilla.mozilla.org/show_bug.cgi?id=1847503) — a standalone Rust implementation of the same color science for testing, tooling, and future browser integration. This was the first version of that library: a single crate combining the color science core with WASM bindings.

## What it implements

- **Color spaces**: sRGB, sRGB-Linear, XYZ-D65, OKLAB, OKLCH, OKLrAB, OKLrCH
- **Gamut mapping**: Both CSS Color 4 algorithms — binary search with local MINDE, and raytracing via AABB intersection in linear-light space
- **Phantom type design**: `Color<S>` encodes color space at the type level — invalid conversions are compile errors, not runtime errors
- **Spec fidelity**: Matrix constants expressed as rational fractions (e.g. `506752. / 1228815.`), matching the CSS Color 4 specification rather than truncated approximations
- **WASM bindings**: `srgb_to_oklch` and `oklch_to_srgb` exposed via wasm-bindgen

## Technical notes

Two errors were found in the CSS Color 4 draft specification during implementation and reported to the CSS Working Group ([issue #10579](https://github.com/w3c/csswg-drafts/issues/10579)):
- [`cast_ray` step 5.5](https://github.com/w3c/csswg-drafts/issues/10579#issuecomment-4122988782): condition inversion (`< 1e-12` should be `> 1e-12`)
- [Raytrace step 11](https://github.com/w3c/csswg-drafts/issues/10579#issuecomment-4122677476): incorrect variable reference (`clip(current)` should be `clip(origin_rgb)`)

All test values verified against the [Coloraide](https://facelessuser.github.io/coloraide/) Python reference library.

## Evolution

The single-crate architecture here — core library and WASM bindings sharing a crate — made it difficult to add a CLI tool without pulling in wasm-bindgen as a dependency. **[color-rs](https://github.com/jparnzen/color-rs)** resolves this with a proper Cargo workspace: `color-lib` (pure Rust core), `color-wasm` (thin bindings), and `color-cli` (CLI tool). The color science is identical; the structure is cleaner.

## A note on process

The code in this project was hand-written and developed iteratively, with Claude (Anthropic) and Claude Code used for code review, bug identification, and Rust mentorship throughout. The README was drafted with Claude's assistance — the technical knowledge, motivations, and decisions described are my own; the concision is Claude's.

## License

MIT © 2026 John P. ARNZEN — see license header in each source file.
