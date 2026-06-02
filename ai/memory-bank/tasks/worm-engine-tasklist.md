---
name: Continuous Collision Detection (CCD) Implementation
about: Implement CCD to prevent high-velocity tunneling.
title: 'Continuous Collision Detection (CCD) Implementation'
labels: 'enhancement, physics'
assignees: ''
---

## Description
Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.

## Acceptance Criteria
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

## Assigned Agency Role
**Physics Engineer** needs to resolve/issue/test this feature.

## Files to Create/Edit
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

## Reference
Tier 1 Projects - Continuous Collision Detection (CCD)---
name: Data-Oriented Design (DOD) & ECS Refactoring
about: Refactor core engine structures to support Data-Oriented Design.
title: 'Data-Oriented Design (DOD) & ECS Refactoring'
labels: 'architecture, refactoring'
assignees: ''
---

## Description
Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.

## Acceptance Criteria
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

## Assigned Agency Role
**Architecture Lead** needs to resolve/issue/test this feature.

## Files to Create/Edit
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

## Reference
Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility---
name: Multithreading and SIMD Vectorization
about: Integrate rayon and std::simd for performance scaling.
title: 'Multithreading and SIMD Vectorization'
labels: 'performance, optimization'
assignees: ''
---

## Description
Integrate `rayon` for task-based parallelism and `std::simd` for vectorizing math operations in the physics pipeline.

## Acceptance Criteria
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- Thread synchronization does not introduce unresolvable latency.

## Assigned Agency Role
**Systems Engineer** needs to resolve/issue/test this feature.

## Files to Create/Edit
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

## Reference
Tier 1 Projects - Multithreading and SIMD Vectorization---
name: Cross-Platform Determinism Setup
about: Implement deterministic execution across architectures.
title: 'Cross-Platform Determinism Setup'
labels: 'determinism, ci'
assignees: ''
---

## Description
Implement strict floating-point math control and deterministic solver execution across multiple architectures.

## Acceptance Criteria
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

## Assigned Agency Role
**Systems Engineer** needs to resolve/issue/test this feature.

## Files to Create/Edit
- src/physics/math.rs
- src/physics/constants.rs
- Tests related to cross-platform execution.

## Reference
Tier 2 Projects - Cross-Platform Determinism---
name: GPU Acceleration (Compute Shaders) Integration
about: Integrate WGPU for GPU-accelerated compute shaders.
title: 'GPU Acceleration (Compute Shaders) Integration'
labels: 'gpu, wgpu'
assignees: ''
---

## Description
Future-proof the engine by integrating WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids.

## Acceptance Criteria
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.

## Assigned Agency Role
**Graphics Engineer** needs to resolve/issue/test this feature.

## Files to Create/Edit
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

## Reference
Tier 2 Projects - GPU Acceleration (Compute Shaders)
## Technical Notes
- Ensure explicit `.to_array()` is used when converting `wide::f32x4` to arrays.
- Ensure `use wide::f32x4;` is explicitly imported when needed to prevent compilation errors.
- wgpu 16-byte alignment crashes must be avoided by using flat `array<f32>` instead of `vec3<f32>` in shaders.
- SIMD must be done via the `wide` crate for cross-platform support. Do not use `std::simd`.
- Rayon must correctly use `.par_iter_mut().zip(...)` tuple chaining to iterate multiple mutable SoA vectors safely.
