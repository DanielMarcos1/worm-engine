# GPU Acceleration (Compute Shaders) Integration

## Labels
gpu, wgpu, experimental

## Body
### Description
Integrate `wgpu` (~v0.19) for GPU-accelerated compute shaders targeting massive scale simulations. `Vector3d` sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives. In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>`.

### Acceptance Criteria
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.

### Technical Constraints
- Integrate `wgpu` (~v0.19) and WGSL.
- In WGSL, avoid 16-byte alignment crashes by using a flat `array<f32>` instead of `vec3<f32>` or `array<vec3<f32>>`.
- Data sent via `bytemuck` (e.g., `Vector3d`) must use `#[repr(C)]` with `Pod` and `Zeroable` derives.
- Kept modular to avoid scope creep and not block v1.0.0.

### Assigned Agency Role
**Graphics Engineer** needs to take care of resolving, issuing, and testing the feature.

### Files to Create/Edit
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

### Project Specs
- **Budget**: [To be determined]
- **Timeline**: Future-proofing
- **Strategic Impact**: Future-proofing for massive scale simulations (soft-bodies, fluids)
