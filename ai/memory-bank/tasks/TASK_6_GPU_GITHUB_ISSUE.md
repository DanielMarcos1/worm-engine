# GPU Acceleration (Compute Shaders) Integration

## Labels
gpu, wgpu

## Body
Integrate `wgpu` (~v0.19) for GPU-accelerated compute shaders targeting massive scale simulations. `Vector3d` sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives. In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>`.

**Acceptance Criteria:**
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.

**Assigned Agency Role:**
**Graphics Engineer** needs to resolve/issue/test this feature.

**Files to Create/Edit:**
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Quality Requirements:**
- [ ] Must pass `cargo check` cleanly
- [ ] Must pass `cargo test` suite
- [ ] No background processes in any commands - NEVER append `&`
- [ ] WGSL shaders must avoid 16-byte alignment crashes by using flat `array<f32>` and Rust structs must use `#[repr(C)]`, `Pod`, and `Zeroable`.
- [ ] Include Playwright screenshot testing: `./qa-playwright-capture.sh http://localhost:8000 public/qa-screenshots`
- [ ] Images from approved sources (Unsplash, https://picsum.photos/) - NO Pexels (403 errors)
