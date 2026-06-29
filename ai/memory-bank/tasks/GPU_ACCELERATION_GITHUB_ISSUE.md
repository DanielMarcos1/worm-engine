# GPU Acceleration (Compute Shaders) Integration

## Labels
gpu, wgpu

## Body
**Description**: Integrate `wgpu` (~v0.19) for GPU-accelerated compute shaders targeting massive scale simulations.

**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.

**Technical Constraints & Notes**:
- Uses `wgpu` (~v0.19) and WGSL.
- `Vector3d` sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives.
- In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>` to avoid 16-byte alignment crashes.
- Files to Create/Edit: `Cargo.toml`, `src/physics/gpu.rs`, `shaders/compute.wgsl`.

**Assigned Agency Role**:
**Graphics Engineer** needs to take care of resolving, issuing, and testing the feature.
