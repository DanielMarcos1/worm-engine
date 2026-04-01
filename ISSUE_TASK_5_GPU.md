---
name: GPU Acceleration (Compute Shaders) Integration
about: Integrate WGPU for GPU-accelerated compute shaders.
title: 'GPU Acceleration (Compute Shaders) Integration'
labels: 'gpu, wgpu'
assignees: ''
---

## Description
Future-proof the engine by integrating WGPU (~v0.19) for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids.

## Acceptance Criteria
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.
- `Vector3d` data sent to WGSL compute shaders via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives in Rust.
- In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>` to avoid 16-byte memory alignment crashes.

## Assigned Agency Role
GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer

## Files to Create/Edit
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

## Reference
Tier 2 Projects - GPU Acceleration (Compute Shaders)