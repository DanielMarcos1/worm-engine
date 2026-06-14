# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**: Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art, high-performance solution capable of capturing top-tier market share in the simulation and gaming sectors.
**Technical Stack**: Rust, rayon, wide, wgpu, libm
**Target Timeline**: v1.0.0 Roadmap

## Development Tasks

### [ ] Task 1: Continuous Collision Detection (CCD) Implementation
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

**Assigned Agency Role**: **Physics Engineer** needs to resolve/issue/test this feature.

**Files to Create/Edit**:
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Continuous Collision Detection (CCD)

### [ ] Task 2: Data-Oriented Design (DOD) & ECS Refactoring
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs. In src/physics/world.rs, the World struct explicitly uses concrete types for its core SoA memory arrays (Vec<Vector3d> for positions, velocities, accelerations, forces; Vec<f32> for masses; Vec<Polygon> for shapes). To properly adhere to DOD, World must directly own these arrays, completely replacing and removing the usage of RigidBodyComponents.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

**Assigned Agency Role**: **Architecture Lead** needs to resolve/issue/test this feature.

**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

**Reference**: Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility

### [ ] Task 3: Multithreading and SIMD Vectorization
**Description**: Integrate rayon for task-based parallelism and the wide crate for cross-platform SIMD vectorization in the physics pipeline (do not use std::simd). Use `use wide::f32x4;` explicitly in src/geometry/vector.rs. When converting wide::f32x4 SIMD vectors to arrays, use the explicit .to_array() method rather than .into(). When iterating multiple mutable Struct of Arrays (SoA) vectors in rayon, chain the iterators using .par_iter_mut().zip(...) for standard iteration, or .par_chunks_mut(chunk_size).zip(...) when processing SIMD chunks. Note that par_iter_mut() takes exactly zero arguments.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions via the wide crate.
- Thread synchronization does not introduce unresolvable latency.

**Assigned Agency Role**: **Systems Engineer** needs to resolve/issue/test this feature.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization

### [ ] Task 4: Cross-Platform Determinism Setup
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using libm for deterministic math operations.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

**Assigned Agency Role**: **Systems Engineer** needs to resolve/issue/test this feature.

**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs
- Tests related to cross-platform execution.

**Reference**: Tier 2 Projects - Cross-Platform Determinism

### [ ] Task 5: GPU Acceleration (Compute Shaders) Integration
**Description**: Future-proof the engine by integrating WGPU (~v0.19) for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids. For WGSL shaders, avoid 16-byte alignment crashes by using flat array<f32> instead of vec3<f32> to match standard Rust bytemuck arrays.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.

**Assigned Agency Role**: **Graphics Engineer** needs to resolve/issue/test this feature.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Reference**: Tier 2 Projects - GPU Acceleration (Compute Shaders)

## Quality Requirements
- [ ] No global `#![allow(...)]` attributes (like dead_code, unused_variables, unused_mut, or unused_imports) in top-level files like src/main.rs.
- [ ] All code features are fully implemented before marking tasks as complete.
- [ ] SIMD vectorization using the wide crate must be applied over SoA chunks, explicitly avoiding Array of Structures (AoS) SIMD on individual math primitives.
- [ ] Maintain >25% portfolio ROI and achieve 95% on-time delivery of the 1.0.0 core features alongside revised SOTA features.

## Technical Notes
**Development Stack**: Rust, rayon, wide, wgpu, libm
**Special Instructions**: The project uses a Data-Oriented Design (DOD) with a Structure of Arrays (SoA) layout. Do not edit application entry points (e.g., src/main.rs) with ad-hoc test code; use dedicated test functions with `#[test]` and `cargo test`.
**Timeline Expectations**: Implementations mapped to v1.0.0 Roadmap milestones. Modularize high-risk features to protect the 95% on-time delivery benchmark.
