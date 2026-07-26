# Title
GPU Acceleration (Compute Shaders) Integration

## Labels
gpu, wgpu

## Body
Future-proof the engine by integrating `wgpu` (~v0.19) for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids. `Vector3d` sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives. In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>` to avoid 16-byte alignment crashes.

### Acceptance Criteria
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.

### Assigned Agency Role
GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer

### Files to Create/Edit
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

### Reference
Tier 2 Projects - GPU Acceleration (Compute Shaders)
