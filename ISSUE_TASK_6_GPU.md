---
name: GPU Acceleration (Compute Shaders) Integration
about: Integrate wgpu (~v0.19) for GPU-accelerated compute shaders targeting massive scale simulations.
title: 'GPU Acceleration (Compute Shaders) Integration'
labels: 'gpu, wgpu'
assignees: ''
---

## Description
Integrate `wgpu` (~v0.19) for GPU-accelerated compute shaders targeting massive scale simulations. `Vector3d` sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives. In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>`.

## Acceptance Criteria
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.

## Assigned Agency Role
Task 6 GPU Acceleration needs to be resolved/issued/tested by the Graphics Engineer.

## Files to Create/Edit
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

## Reference
GPU Acceleration (Compute Shaders) from `ISSUE_STATE_OF_THE_ART.md`