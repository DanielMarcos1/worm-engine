# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**: "Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art, high-performance solution capable of capturing top-tier market share in the simulation and gaming sectors."
**Technical Stack**: Rust, rayon, wide, libm, wgpu (~v0.19), WGSL
**Target Timeline**: Integrated into v0.4.0 (CCD), v0.6.0 (DOD, SIMD), v0.7.0/v1.0.0 (Determinism), and v0.8.0/v1.0.0 (GPU) while maintaining 95% on-time delivery benchmark for the current roadmap milestones.

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

**Reference**: Issue Task 1 CCD
**Assignment**: Task 1 Continuous Collision Detection (CCD) needs to be resolved/issued/tested by the Physics Engineer

### [ ] Task 2: Data-Oriented Design (DOD) & ECS Refactoring (30-60 minutes)
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

**Reference**: Issue Task 2 DOD
**Assignment**: Task 2 Data-Oriented Design (DOD) & ECS Refactoring needs to be resolved/issued/tested by the Architecture Lead

### [ ] Task 3: Multithreading Implementation
**Description**: Integrate `rayon` for task-based parallelism. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Thread synchronization does not introduce unresolvable latency.
- SIMD vectorization utilizes a Structure of Arrays (SoA) approach rather than AoS on individual math primitives.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/world.rs

**Reference**: Issue Task 3 SIMD (Part 1 - Rayon)
**Assignment**: Task 3 Multithreading Implementation needs to be resolved/issued/tested by the Systems Engineer

### [ ] Task 4: SIMD Vectorization Implementation
**Description**: Integrate `wide` for vectorizing math operations in the physics pipeline. Defer until DOD refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`. Do not use `std::simd`.
**Acceptance Criteria**:
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions using `wide`.
- SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Issue Task 3 SIMD (Part 2 - SIMD)
**Assignment**: Task 4 SIMD Vectorization Implementation needs to be resolved/issued/tested by the Systems Engineer

### [ ] Task 5: Cross-Platform Determinism Setup
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm`.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs

**Reference**: Issue Task 4 Determinism
**Assignment**: Task 5 Cross-Platform Determinism Setup needs to be resolved/issued/tested by the Systems Engineer

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

**Reference**: Issue Task 5 GPU
**Assignment**: Task 6 GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer

## Quality Requirements
- [ ] Must pass `cargo check` cleanly
- [ ] Must pass `cargo test` suite
- [ ] No background processes in any commands - NEVER append `&`
- [ ] Iterating multiple mutable SoA arrays in `rayon` must chain `.par_iter_mut().zip(...)`
- [ ] WGSL shaders must avoid 16-byte alignment crashes by using flat `array<f32>` and Rust structs must use `#[repr(C)]`, `Pod`, and `Zeroable`.

## Technical Notes
**Development Stack**: Rust, rayon, wide, libm, wgpu (~v0.19), WGSL
**Special Instructions**: Ensure DOD refactoring is complete before implementing SIMD vectorization to allow SoA optimization. Risk of scope creep with GPU/CCD features; modularize as optional add-ons to not block v1.0.0.
**Timeline Expectations**: Milestones to be met for 0.4.0, 0.6.0, 0.7.0, and 0.8.0 as per strategic portfolio plan. Target 30-60 minutes maximum per actionable development task.
