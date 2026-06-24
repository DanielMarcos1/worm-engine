# Title
Worm Engine v1.0.0 & SOTA Expansion Initiative

## Labels
strategic, portfolio, sota, physics

## Body

The current 3D physics engine landscape demands hyper-performance, deterministic simulation, and memory efficiency. We are evolving Worm Engine to a state-of-the-art (SOTA) solution.
Expected ROI: 35%+ return.


### Task 1: Continuous Collision Detection (CCD)
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
Tier 1 Projects - Continuous Collision Detection (CCD)
## Technical Constraints
- Modular add-on to not block 1.0.0. 0% tunneling at 1000m/s.

### Task 2: Data-Oriented Design (DOD) & ECS Refactoring
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
Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility
## Technical Constraints
- World struct must directly own concrete types for core SoA memory arrays (Vec<Vector3d>, Vec<f32>, Vec<Polygon>), replacing RigidBodyComponents.

### Task 3: Multithreading and SIMD Vectorization
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
Tier 1 Projects - Multithreading and SIMD Vectorization
## Technical Constraints
- SIMD vectorization using wide crate (not std::simd) over SoA chunks.
- Use par_iter_mut().zip(...) or par_chunks_mut(chunk_size).zip(...) for rayon tuple chaining.

### Task 4: Cross-Platform Determinism Setup
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
Tier 2 Projects - Cross-Platform Determinism
### Task 5: GPU Acceleration (Compute Shaders) Integration
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
## Technical Constraints
- For WGSL shaders, use flat array<f32> instead of vec3<f32> to match standard Rust bytemuck arrays and avoid wgpu 16-byte alignment crashes.
