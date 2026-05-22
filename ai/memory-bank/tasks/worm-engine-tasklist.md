# Worm Engine SOTA Integration Development Tasks

## Specification Summary
**Original Requirements**:
- Implement Continuous Collision Detection (CCD) to prevent tunneling at high velocities (0% tunneling at 1000m/s).
- Refactor core engine structures to support Data-Oriented Design (DOD) & ECS Compatibility (API integration under 2 hours).
- Integrate multithreading and SIMD Vectorization (`rayon`, `wide`) for performance scaling (linear scaling up to 16 threads).
- Establish cross-platform determinism setup.
- Integrate GPU acceleration (Compute Shaders) with WGPU.
**Technical Stack**: Rust, `rayon`, `wide`, `libm`, `wgpu` (~v0.19), WGSL.
**Target Timeline**: v0.4.0 for CCD, v0.6.0 for DOD and SIMD, v0.7.0/v1.0.0 for Determinism, post-v0.6.0 for GPU Acceleration.

## Development Tasks

### [ ] Task 1: Continuous Collision Detection (CCD) Implementation
**Description**: Implement CCD to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies. Modularize as an optional add-on.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

**Files to Create/Edit**:
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

**Reference**: Continuous Collision Detection (CCD) Implementation needs to be resolved/issued/tested by the Physics Engineer.

### [ ] Task 2: Data-Oriented Design (DOD) & ECS Refactoring
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs. Use explicit type allocations in flat arrays.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

**Reference**: Data-Oriented Design (DOD) & ECS Refactoring needs to be resolved/issued/tested by the Architecture Lead.

### [ ] Task 3: Multithreading and SIMD Vectorization
**Description**: Integrate `rayon` for task-based parallelism and the `wide` crate for vectorizing math operations in the physics pipeline (avoiding `std::simd`). Use SoA layout.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions via the `wide` crate.
- Thread synchronization does not introduce unresolvable latency.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Multithreading and SIMD Vectorization needs to be resolved/issued/tested by the Systems Engineer.

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

**Reference**: Cross-Platform Determinism Setup needs to be resolved/issued/tested by the Systems Engineer.

### [ ] Task 5: GPU Acceleration (Compute Shaders) Integration
**Description**: Future-proof the engine by integrating WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids. Keep modular as an optional add-on.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Reference**: GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer.

## Quality Requirements
- [ ] Must pass `cargo check` cleanly
- [ ] Must pass `cargo test` suite
- [ ] No background processes in any commands - NEVER append `&`.
- [ ] Iterating multiple mutable SoA arrays in `rayon` must chain `.par_iter_mut().zip(...)`
- [ ] WGSL shaders must avoid 16-byte alignment crashes by using flat `array<f32>` and Rust structs must use `#[repr(C)]`, `Pod`, and `Zeroable`.
- [ ] All components must use supported dependencies (`wide`, `rayon`, `libm`, `wgpu`). Strict avoidance of unstable Rust features like `std::simd`.

## Technical Notes
**Development Stack**: Rust, rayon, wide, libm, wgpu, WGSL.
**Special Instructions**: Ensure all SIMD operations use the `wide` crate with `.to_array()` for explicit conversion to avoid inference compilation errors. GPU and CCD features should be modularized as optional add-ons to mitigate risk of scope creep.
**Timeline Expectations**: Core 1.0.0 milestones must be maintained with 95% on-time delivery; SOTA features layered efficiently or scheduled for post-1.0.0 where risky.
