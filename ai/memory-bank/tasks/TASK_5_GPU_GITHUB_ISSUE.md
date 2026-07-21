# Title
GPU Acceleration (Compute Shaders) Integration

## Labels
gpu, wgpu

## Body
**Description**:
Future-proof the engine by integrating WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids. To avoid 16-byte alignment crashes in WGSL shaders, use flat `array<f32>` instead of `vec3<f32>`. Data sent via `bytemuck` (e.g., `Vector3d`) must use `#[repr(C)]` with `Pod` and `Zeroable` derives.

**Assigned Agency Role**:
**Graphics Engineer** needs to resolve/issue/test this feature.

**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- WGSL shaders avoid alignment crashes by using `array<f32>`.
- Vector data uses `#[repr(C)]` with `Pod` and `Zeroable` for `bytemuck` transfer.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Reference**:
Tier 2 Projects - GPU Acceleration (Compute Shaders)
