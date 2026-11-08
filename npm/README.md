# @hqli/wasm-imagequant

Fork of [wasm-imagequant](https://github.com/reed-soul/wasm-imagequant) `0.1.4` by reed-soul, MIT OR Apache-2.0.
The API is unchanged; see the upstream README for usage.

## What changed

Upstream scales each pixel's diffused error down to as little as 15% when it is perceptually small
(`clamp(dist_sq / 0.0025, 0.15, 1.0)` in `src/dither.rs`). On smooth gradients almost every error is small, so most of
it is dropped and the output shows visible bands. This fork always diffuses the full error (`dither_strength`).
Files get slightly larger from the extra dither noise.

## Building

```bash
sh npm/build.sh
```

Needs Rust with the `wasm32-unknown-unknown` target and `wasm-bindgen-cli` 0.2.129. Output goes to `pkg/`.
