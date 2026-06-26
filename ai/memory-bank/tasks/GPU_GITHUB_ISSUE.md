# GPU Acceleration (Compute Shaders) Integration
## Labels
gpu, wgpu
## Body
### Description
Integrate `wgpu` for GPU-accelerated compute shaders targeting massive scale simulations.

### Acceptance Criteria
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.

### Files to Create/Edit
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

### Technical Constraints
- Integrate `wgpu` (~v0.19) for GPU acceleration.
- Avoid 16-byte alignment crashes in WGSL shaders by using flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>` or `vec3<f32>`.
- Data sent via `bytemuck` (e.g., `Vector3d`) must use `#[repr(C)]` with `Pod` and `Zeroable` derives.

### Assigned Agency Role
**Graphics Engineer** needs to take care of resolving, issuing, and testing this feature.
