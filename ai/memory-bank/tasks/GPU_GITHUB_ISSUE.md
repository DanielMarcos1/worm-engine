# Title
GPU Acceleration (Compute Shaders) Integration

## Labels
gpu, wgpu

## Body
**Description**: Integrate `wgpu` (~v0.19) for GPU-accelerated compute shaders targeting massive scale simulations.

**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.

**Technical Constraints**:
- Use `wgpu` (~v0.19) and WGSL.
- Avoid 16-byte alignment crashes in WGSL shaders by using flat `array<f32>` instead of `vec3<f32>`.
- Data sent via `bytemuck` (e.g., `Vector3d`) must use `#[repr(C)]` with `Pod` and `Zeroable` derives.
- Modularize as an optional add-on to not block v1.0.0.

**Assignment**:
GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer.
