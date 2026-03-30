# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**: Expand the Worm Engine from a functional 3D physics engine into a state-of-the-art (SOTA), high-performance solution. Tier 1 projects include Continuous Collision Detection (CCD), Data-Oriented Design (DOD) & ECS Compatibility, and Multithreading and SIMD Vectorization. Tier 2/Innovation projects include Cross-Platform Determinism Setup and GPU Acceleration (Compute Shaders) Integration.
**Technical Stack**: Rust, `rayon`, `wide`, `libm`, `wgpu` (~v0.19) with WGSL
**Target Timeline**: v0.4.0 (CCD), v0.6.0 (DOD & SIMD), v0.7.0/v1.0.0 (Determinism), Post-v0.6.0 (GPU)

## Development Tasks

### [ ] Task 1: Continuous Collision Detection (CCD) Implementation
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

**Role Assignment**: Continuous Collision Detection (CCD) Implementation needs to be resolved/issued/tested by the Physics Engineer

**Files to Create/Edit**:
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Continuous Collision Detection (CCD)

### [ ] Task 2: Data-Oriented Design (DOD) & ECS Refactoring
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs. Memory layout optimized for cache coherency.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

**Role Assignment**: Data-Oriented Design (DOD) & ECS Refactoring needs to be resolved/issued/tested by the Architecture Lead

**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

**Reference**: Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility

### [ ] Task 3: Multithreading and SIMD Vectorization
**Description**: Integrate `rayon` for task-based parallelism and `wide` (not `std::simd`) for vectorizing math operations in the physics pipeline. Must defer SIMD until DOD refactoring is complete (Structure of Arrays approach).
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize `wide` SIMD instructions.
- Thread synchronization does not introduce unresolvable latency.

**Role Assignment**: Multithreading and SIMD Vectorization needs to be resolved/issued/tested by the Systems Engineer

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization

### [ ] Task 4: Cross-Platform Determinism Setup
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm`.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

**Role Assignment**: Cross-Platform Determinism Setup needs to be resolved/issued/tested by the Systems Engineer

**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs
- Tests related to cross-platform execution.

**Reference**: Tier 2 Projects - Cross-Platform Determinism

### [ ] Task 5: GPU Acceleration (Compute Shaders) Integration
**Description**: Future-proof the engine by integrating `wgpu` for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids. Use `#[repr(C)]` and flat `array<f32>` (indexing by 3) in WGSL.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.

**Role Assignment**: GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer

**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Reference**: Tier 2 Projects - GPU Acceleration (Compute Shaders)

## Quality Requirements
- [ ] Code must compile without errors using `cargo check`.
- [ ] All tests must pass using `cargo test`.
- [ ] Do not use background processes in any commands - NEVER append `&`.
- [ ] Strictly use the `wide` crate for SIMD operations, do not use `std::simd`.
- [ ] Use `rayon` for task-based parallelism.
- [ ] No direct AoS SIMD on `Vector3d` primitives; strictly use SoA.

## Technical Notes
**Development Stack**: Rust, `rayon`, `wide`, `libm`, `wgpu` (~v0.19) with WGSL
**Special Instructions**:
- Defer SIMD vectorization until DOD refactoring is complete.
- When iterating over mutable SoA arrays in parallel with `rayon`, chain `.par_iter_mut().zip(...)`.
- `Vector3d` implementation includes `length_squared()` for performant distance comparisons.
**Timeline Expectations**: Focus on delivering Tier 1 projects first to align with v0.6.0 roadmap without blocking core functionality delivery.
