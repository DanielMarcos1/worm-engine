# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**: Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art (SOTA), high-performance solution. Align technical milestones to deliver hyper-performance, deterministic simulation, and memory efficiency, positioning for the anticipated market shift toward highly scalable, data-oriented multiplayer experiences.
**Technical Stack**: Rust, rayon, wide, libm, wgpu (~v0.19), WGSL
**Target Timeline**: v0.4.0 (CCD), v0.6.0 (DOD & SIMD), v0.7.0/v1.0.0 (Determinism), Experimental (GPU Acceleration).

## Development Tasks

### [ ] Task 1: Continuous Collision Detection (CCD) Implementation
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

**Files to Create/Edit**:
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Continuous Collision Detection (CCD)
**Assignment**: Continuous Collision Detection (CCD) needs to be resolved/issued/tested by the Physics Engineer.

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

**Reference**: Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility
**Assignment**: Data-Oriented Design (DOD) & ECS Refactoring needs to be resolved/issued/tested by the Architecture Lead.

### [ ] Task 3: Multithreading and SIMD Vectorization
**Description**: Integrate `rayon` for task-based parallelism and `wide` for vectorizing math operations in the physics pipeline.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions via the `wide` crate.
- Thread synchronization does not introduce unresolvable latency.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization
**Assignment**: Multithreading and SIMD Vectorization needs to be resolved/issued/tested by the Systems Engineer.

### [ ] Task 4: Cross-Platform Determinism Setup
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm`.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs
- Tests related to cross-platform execution.

**Reference**: Tier 2 Projects - Cross-Platform Determinism
**Assignment**: Cross-Platform Determinism Setup needs to be resolved/issued/tested by the Systems Engineer.

### [ ] Task 5: GPU Acceleration (Compute Shaders) Integration
**Description**: Future-proof the engine by integrating WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Reference**: Tier 2 Projects - GPU Acceleration (Compute Shaders)
**Assignment**: GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer.

## Quality Requirements
- [ ] No `std::simd` (`portable_simd`) in the codebase as it is an unstable Rust library feature that causes compilation errors. Strictly use the `wide` crate (e.g., `wide::f32x4`) for SIMD operations to ensure stable builds.
- [ ] Code must pass `cargo check` cleanly with no warnings or errors.
- [ ] Code must pass `cargo test` successfully for all implemented features and unit tests.
- [ ] No background processes in any commands - NEVER append `&`.
- [ ] Code should follow standard Rust formatting guidelines (`cargo fmt`).

## Technical Notes
**Development Stack**: Rust, rayon, wide, libm, wgpu (~v0.19), WGSL
**Special Instructions**:
- Maintain >25% portfolio ROI, achieve 95% on-time delivery.
- Defer SIMD vectorization until the Data-Oriented Design (DOD) refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives.
- When iterating over multiple mutable SoA arrays in parallel with `rayon`, chain `.par_iter_mut().zip(...)` instead of passing a tuple to `.into_par_iter()`.
- The `World` struct delegates its Data-Oriented Design (SoA) layout entirely to `RigidBodyComponents`.
**Timeline Expectations**: Integration of Tier 1 requirements should proceed without blocking the core 1.0.0 deliverables. Feature integration times: DOD under 2 hours, multithreading scales linearly.