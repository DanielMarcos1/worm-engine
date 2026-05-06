# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**:
- Implement Continuous Collision Detection (CCD) to prevent tunneling at high velocities (0% tunneling at 1000m/s).
- Refactor core engine structures to support Data-Oriented Design (DOD) & ECS Compatibility (seamless Bevy/Flecs integration).
- Integrate Multithreading and SIMD Vectorization using `rayon` and `wide` crate (`wide::f32x4` only, NO `std::simd`) for linear scaling up to 16 threads.
- Implement Cross-Platform Determinism Setup (strict floating-point math control).
- Future-proof with GPU Acceleration (Compute Shaders) Integration using WGPU.
**Technical Stack**: Rust, `rayon`, `wide`, `libm`, `wgpu` (v29.0.0), WGSL
**Target Timeline**: v0.4.0 (CCD), v0.6.0 (DOD & SIMD), v0.7.0/v1.0.0 (Determinism), Experimental (GPU).

## Development Tasks

### [ ] Task 1: Continuous Collision Detection (CCD) Implementation
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies. Needs to be resolved/issued/tested by the Physics Engineer.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

**Files to Create/Edit**:
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Continuous Collision Detection (CCD)

### [ ] Task 2: Data-Oriented Design (DOD) & ECS Refactoring
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs. Needs to be resolved/issued/tested by the Architecture Lead.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

**Reference**: Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility

### [ ] Task 3: Multithreading and SIMD Vectorization
**Description**: Integrate `rayon` for task-based parallelism and `wide` for vectorizing math operations in the physics pipeline. Needs to be resolved/issued/tested by the Systems Engineer.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions (using `wide`).
- Thread synchronization does not introduce unresolvable latency.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization

### [ ] Task 4: Cross-Platform Determinism Setup
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm`. Needs to be resolved/issued/tested by the Systems Engineer.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs
- Tests related to cross-platform execution.

**Reference**: Tier 2 Projects - Cross-Platform Determinism

### [ ] Task 5: GPU Acceleration (Compute Shaders) Integration
**Description**: Future-proof the engine by integrating WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids. Needs to be resolved/issued/tested by the Graphics Engineer.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Reference**: Tier 2 Projects - GPU Acceleration (Compute Shaders)

## Quality Requirements
- [ ] Code passes standard Rust checks: `cargo check` and `cargo test`.
- [ ] No background processes in any commands - NEVER append `&`.
- [ ] Strict use of `wide` crate for SIMD operations. Do NOT use `std::simd` (`portable_simd`).
- [ ] Avoid Array of Structures (AoS) SIMD on individual math primitives like `Vector3d`.
- [ ] Arithmetic methods in `Vector3d` must use standard scalar math (f32) to comply with SoA architectural pattern.
- [ ] WGPU implementations must use flat `array<f32>` in WGSL instead of `array<vec3<f32>>` to avoid memory alignment crashes.
- [ ] Modularity maintained for GPU and CCD features as optional add-ons to prevent scope creep.

## Technical Notes
**Development Stack**: Rust, rayon, wide, libm, wgpu, WGSL
**Special Instructions**: Front-load investment in architectural refactoring (ECS compatibility) to minimize technical debt. SOTA features (CCD, GPU acceleration) will be modularized as optional add-ons to not block 1.0.0.
**Timeline Expectations**: Maintaining 95% on-time delivery. v0.4.0 for CCD, v0.6.0 for DOD/SIMD, v0.7.0/v1.0.0 for Determinism, post-v0.6.0 for experimental GPU features.