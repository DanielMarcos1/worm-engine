# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**: "Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art (SOTA), high-performance solution capable of capturing top-tier market share in the simulation and gaming sectors."
**Technical Stack**: Rust, rayon, wide, libm, wgpu (~v0.19), WGSL
**Target Timeline**: v0.4.0 (CCD), v0.6.0 (DOD & Multithreading/SIMD), v0.7.0/v1.0.0 (Determinism), v1.0.0 (GPU Acceleration)

## Development Tasks

### [ ] Task 1: Continuous Collision Detection (CCD)
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

**Files to Create/Edit**:
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

**Reference**: Continuous Collision Detection (CCD) from `ISSUE_STATE_OF_THE_ART.md`
**Assignment**: Task 1 Continuous Collision Detection needs to be resolved/issued/tested by the Physics Engineer

### [ ] Task 2: Data-Oriented Design (DOD) & ECS Refactoring
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

**Reference**: Data-Oriented Design (DOD) & ECS Compatibility from `ISSUE_STATE_OF_THE_ART.md`
**Assignment**: Task 2 Data-Oriented Design needs to be resolved/issued/tested by the Architecture Lead

### [ ] Task 3: Multithreading via rayon
**Description**: Integrate `rayon` for task-based parallelism to scale linearly up to 16 threads.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Thread synchronization does not introduce unresolvable latency.
- Iterating over multiple mutable SoA arrays in `rayon` chains `.par_iter_mut().zip(...)` or `.par_chunks_mut(4).zip(...)` instead of passing a tuple.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/world.rs

**Reference**: Multithreading and SIMD Vectorization from `ISSUE_STATE_OF_THE_ART.md`
**Assignment**: Task 3 Multithreading needs to be resolved/issued/tested by the Systems Engineer

### [ ] Task 4: SIMD Vectorization via wide
**Description**: Integrate the `wide` crate for vectorizing math operations in the physics pipeline. Defer until DOD refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.
**Acceptance Criteria**:
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions via the `wide` crate.
- SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Multithreading and SIMD Vectorization from `ISSUE_STATE_OF_THE_ART.md`
**Assignment**: Task 4 SIMD Vectorization needs to be resolved/issued/tested by the Systems Engineer

### [ ] Task 5: Cross-Platform Determinism Setup
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm`.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs

**Reference**: Cross-Platform Determinism from `ISSUE_STATE_OF_THE_ART.md`
**Assignment**: Task 5 Cross-Platform Determinism needs to be resolved/issued/tested by the Systems Engineer

### [ ] Task 6: GPU Acceleration (Compute Shaders) Integration
**Description**: Integrate `wgpu` (~v0.19) for GPU-accelerated compute shaders targeting massive scale simulations. `Vector3d` sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives. In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>`.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Reference**: GPU Acceleration (Compute Shaders) from `ISSUE_STATE_OF_THE_ART.md`
**Assignment**: Task 6 GPU Acceleration needs to be resolved/issued/tested by the Graphics Engineer

## Quality Requirements
- [ ] Must pass `cargo check` cleanly
- [ ] Must pass `cargo test` suite
- [ ] No background processes in any commands - NEVER append `&`
- [ ] SIMD uses `wide` exclusively, no `std::simd`
- [ ] Multithreading uses `rayon` with proper iteration techniques

## Technical Notes
**Development Stack**: Rust, rayon, wide, libm, wgpu (~v0.19), WGSL
**Special Instructions**: Ensure DOD refactoring is complete before implementing SIMD vectorization to allow SoA optimization.
**Timeline Expectations**: Front-loading investment in architectural refactoring (ECS compatibility) to minimize technical debt. SOTA features modularized as optional add-ons rather than hard blockers for 1.0.0 core functionality.
