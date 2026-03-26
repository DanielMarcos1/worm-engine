# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**: "Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art, high-performance solution capable of capturing top-tier market share in the simulation and gaming sectors."
**Technical Stack**: Rust, rayon, wide, libm, wgpu (~v0.19) with WGSL, bytemuck
**Target Timeline**: v1.0.0 Roadmap

## Development Tasks

### [ ] Task 1: Continuous Collision Detection (CCD) Implementation
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies. Continuous Collision Detection (CCD) Implementation needs to be resolved/issued/tested by the Physics Engineer.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

**Files to Create/Edit**:
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Continuous Collision Detection (CCD)
**Milestone**: v0.4.0

### [ ] Task 2: Data-Oriented Design (DOD) & ECS Refactoring
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs. Data-Oriented Design (DOD) & ECS Refactoring needs to be resolved/issued/tested by the Architecture Lead.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

**Reference**: Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility
**Milestone**: v0.6.0

### [ ] Task 3: Multithreading and SIMD Vectorization
**Description**: Integrate `rayon` for task-based parallelism and `wide` for vectorizing math operations in the physics pipeline. Multithreading and SIMD Vectorization needs to be resolved/issued/tested by the Systems Engineer.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- Thread synchronization does not introduce unresolvable latency.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization
**Milestone**: v0.6.0

### [ ] Task 4: Cross-Platform Determinism Setup
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm` for deterministic math operations. Cross-Platform Determinism Setup needs to be resolved/issued/tested by the Systems Engineer.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs
- Tests related to cross-platform execution.

**Reference**: Tier 2 Projects - Cross-Platform Determinism
**Milestone**: v0.7.0/v1.0.0

### [ ] Task 5: GPU Acceleration (Compute Shaders) Integration
**Description**: Integrate WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids. GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Reference**: Tier 2 Projects - GPU Acceleration (Compute Shaders)
**Milestone**: Innovation Pipeline (v1.0.0+)

## Quality Requirements
- [ ] Code passes `cargo check` without warnings
- [ ] Code passes all tests via `cargo test`
- [ ] No background processes in any commands - NEVER append `&`
- [ ] Data-Oriented Design structures must use flat arrays/SOA layout
- [ ] Do not use AoS SIMD on individual math primitives like `Vector3d`
- [ ] Use `#[repr(C)]` with `Pod` and `Zeroable` for `Vector3d` when sending to WGSL. In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>` to avoid 16-byte memory alignment crashes.
- [ ] Avoid redundant fields directly on the `World` struct, delegate to `RigidBodyComponents`
- [ ] Use chaining `.par_iter_mut().zip(...)` instead of passing a tuple to `.into_par_iter()` for rayon large mutable tuples.
- [ ] Utilize `Vector3d::length_squared()` for performant distance comparisons without hallucinating standard library traits.

## Technical Notes
**Development Stack**: Rust, rayon, wide, libm, wgpu (~v0.19) with WGSL, bytemuck
**Special Instructions**: Defer SIMD vectorization until the DOD refactoring is complete. Modularize high-risk features like CCD or GPU acceleration as optional add-ons rather than hard blockers for 1.0.0 core functionality. The core repository has no external API consumers yet, allowing for safe, broad architectural breaking changes.
**Timeline Expectations**: 35% ROI contingent on successful integration of SOTA features without derailing the 1.0.0 roadmap.
