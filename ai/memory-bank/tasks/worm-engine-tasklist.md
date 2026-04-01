# Worm Engine Task List

## Continuous Collision Detection (CCD) Implementation
- [ ] [Continuous Collision Detection (CCD) Implementation] needs to be resolved/issued/tested by the [Physics Engineer]
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.
**Files to Create/Edit**:
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs

## Data-Oriented Design (DOD) & ECS Refactoring
- [ ] [Data-Oriented Design (DOD) & ECS Refactoring] needs to be resolved/issued/tested by the [Architecture Lead]
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.
**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

## Multithreading and SIMD Vectorization
- [ ] [Multithreading and SIMD Vectorization] needs to be resolved/issued/tested by the [Systems Engineer]
**Description**: Integrate `rayon` for task-based parallelism and `std::simd` for vectorizing math operations in the physics pipeline.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- Thread synchronization does not introduce unresolvable latency.
**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

## Cross-Platform Determinism Setup
- [ ] [Cross-Platform Determinism Setup] needs to be resolved/issued/tested by the [Systems Engineer]
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.
**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs
- Tests related to cross-platform execution.

## GPU Acceleration (Compute Shaders) Integration
- [ ] [GPU Acceleration (Compute Shaders) Integration] needs to be resolved/issued/tested by the [Graphics Engineer]
**Description**: Future-proof the engine by integrating WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.
**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl

## Quality Requirements
- [ ] `cargo check`
- [ ] `cargo test`

