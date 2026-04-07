# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**:
- Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art, high-performance solution.
- Implement Continuous Collision Detection (CCD) to prevent "tunneling" at high velocities.
- Refactor core engine structures to support Data-Oriented Design (DOD) and modern ECS architectures.
- Integrate `rayon` for task-based parallelism and SIMD vectorization for performance scaling.
- Implement strict floating-point math control and deterministic solver execution across multiple architectures.
- Integrate WGPU for GPU-accelerated compute shaders.
**Technical Stack**: Rust, rayon, wide (for SIMD), libm, wgpu, WGSL. (Note: std::simd is unstable and not to be used, use wide crate instead).
**Target Timeline**: Integrate as modular add-ons to maintain 95% on-time delivery benchmark for v1.0.0 roadmap.

## Development Tasks

### [ ] Task 1: Continuous Collision Detection (CCD) Implementation
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

**Assigned Role**: Task 1 needs to be resolved/issued/tested by the Physics Engineer.

**Files to Create/Edit**:
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Continuous Collision Detection (CCD)

### [ ] Task 2: Data-Oriented Design (DOD) & ECS Refactoring
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs. Memory layout must use pure SoA layout.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency via SoA layout (e.g. velocities_x, velocities_y, velocities_z instead of Vec<Vector3d>).
- API allows integration with a standard ECS in under 2 hours.
- Core systems operate on flat arrays or similar DOD structures.

**Assigned Role**: Task 2 needs to be resolved/issued/tested by the Architecture Lead.

**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

**Reference**: Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility

### [ ] Task 3: Multithreading and SIMD Vectorization
**Description**: Integrate `rayon` for task-based parallelism and `wide` crate (NOT `std::simd`) for vectorizing math operations in the physics pipeline.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations utilize SIMD instructions (wide crate).
- Thread synchronization does not introduce unresolvable latency.
- Parallel iteration over multiple mutable SoA arrays uses chained `.par_chunks_mut(4).zip(...)` instead of passing a tuple to `.into_par_iter()`.

**Assigned Role**: Task 3 needs to be resolved/issued/tested by the Systems Engineer.

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

**Assigned Role**: Task 4 needs to be resolved/issued/tested by the Systems Engineer.

**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs
- Tests related to cross-platform execution.

**Reference**: Tier 2 Projects - Cross-Platform Determinism

### [ ] Task 5: GPU Acceleration (Compute Shaders) Integration
**Description**: Future-proof the engine by integrating WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline. Use flat array<f32> in WGSL instead of array<vec3<f32>> to avoid alignment crashes.
- CPU pipeline remains stable during GPU execution.

**Assigned Role**: Task 5 needs to be resolved/issued/tested by the Graphics Engineer.

**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

**Reference**: Tier 2 Projects - GPU Acceleration (Compute Shaders)

## Quality Requirements
- [ ] Code must compile and pass `cargo check` and `cargo test`.
- [ ] No background processes in any commands - NEVER append `&`.
- [ ] Adhere to pure SoA DOD principles (e.g. deconstruct 3D vectors).
- [ ] Use `wide` crate for SIMD, never use `std::simd` (`portable_simd`).
- [ ] Delay Array of Structures (AoS) SIMD vectorization until DOD refactoring is complete.
- [ ] Use `#[repr(C)]` with `Pod` and `Zeroable` for `bytemuck` integration with WGPU.

## Technical Notes
**Development Stack**: Rust, rayon, wide, libm, wgpu, WGSL.
**Special Instructions**: Ensure all tasks are realistically scoped. Do not include unprompted features. Note that main.rs is currently a placeholder.
**Timeline Expectations**: Focus on modular add-ons to protect 95% on-time delivery of v1.0.0 roadmap.
