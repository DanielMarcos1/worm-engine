# GPU Acceleration (Compute Shaders) Integration

## Labels
gpu, wgpu

## Body
**Description**:
Future-proof the engine by integrating WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids.

**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.

**Assigned Agency Role**:
**Graphics Engineer** needs to resolve/issue/test this feature.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Reference**:
Tier 2 Projects - GPU Acceleration (Compute Shaders)

**Technical Notes**:
- Use `wgpu` (~v0.19).
- For WGSL shaders, avoid 16-byte alignment crashes by using flat `array<f32>` instead of `vec3<f32>` to match standard Rust `bytemuck` arrays.
