# GPU Acceleration (Compute Shaders) Integration

## Labels
gpu, wgpu

## Body
**Description**:
Future-proof the engine by integrating `wgpu` (~v0.19) for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids. For GPU integration, avoid 16-byte alignment crashes in WGSL shaders by using flat `array<f32>` instead of `vec3<f32>`. Data sent via `bytemuck` (e.g., `Vector3d`) must use `#[repr(C)]` with `Pod` and `Zeroable` derives.

**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Reference**:
Tier 2 Projects - GPU Acceleration (Compute Shaders)

### Assigned Agency Role
Graphics Engineer needs to take care of resolving, issuing, and testing the feature.
