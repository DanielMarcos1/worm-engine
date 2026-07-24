# Title
GPU Acceleration (Compute Shaders) Integration

## Labels
gpu, wgpu

## Body
Future-proof the engine by integrating `wgpu` (~v0.19) for GPU-accelerated compute shaders, targeting massive scale simulations like soft-bodies or fluids.

**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.
- WGSL shaders must avoid 16-byte alignment crashes by using flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>`.
- `Vector3d` sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives.

GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer.
