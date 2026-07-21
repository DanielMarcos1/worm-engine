# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**: Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art (SOTA), high-performance solution. Align technical milestones to deliver hyper-performance, deterministic simulation, and memory efficiency for modern data-oriented architectures.
**Technical Stack**: Rust, rayon, wide (for SIMD), wgpu (for GPU compute).
**Target Timeline**: v1.0.0 roadmap milestones.

## Development Tasks

### [ ] Task 1: Continuous Collision Detection (CCD) Implementation
**Description**: Implement CCD to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.
**Assigned Agency Role**: Physics Engineer
**Files to Create/Edit**: src/physics/ccd.rs, src/physics/mod.rs, src/physics/world.rs
**Reference**: Tier 1 Projects - Continuous Collision Detection (CCD)

### [ ] Task 2: Data-Oriented Design (DOD) & ECS Refactoring
**Description**: Refactor core engine structures to support Data-Oriented Design. The `World` struct must utilize a Structure of Arrays (SoA) layout directly owning core memory arrays without intermediate component structs like `RigidBodyComponents`.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency using SoA.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays.
**Assigned Agency Role**: Architecture Lead
**Files to Create/Edit**: src/physics/rigid_body.rs, src/physics/world.rs, src/physics/components.rs
**Reference**: Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility

### [ ] Task 3: Multithreading and SIMD Vectorization
**Description**: Integrate `rayon` for task-based parallelism and `wide` crate for vectorizing math operations. Parallel iteration over multiple mutable arrays must be chained using `.par_iter_mut().zip(...)`.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations utilize SIMD instructions via `wide`.
- Thread synchronization does not introduce unresolvable latency.
- Parallel iterations correctly use `.par_iter_mut().zip(...)`.
**Assigned Agency Role**: Systems Engineer
**Files to Create/Edit**: Cargo.toml, src/geometry/vector.rs, src/physics/world.rs
**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization

### [ ] Task 4: Cross-Platform Determinism Setup
**Description**: Implement strict floating-point math control and deterministic solver execution across architectures.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.
**Assigned Agency Role**: Systems Engineer
**Files to Create/Edit**: src/physics/math.rs, src/physics/constants.rs, Tests related to cross-platform execution.
**Reference**: Tier 2 Projects - Cross-Platform Determinism

### [ ] Task 5: GPU Acceleration (Compute Shaders) Integration
**Description**: Integrate WGPU for compute shaders. Use flat `array<f32>` instead of `vec3<f32>` in WGSL. Data sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable`.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- WGSL shaders avoid alignment crashes by using `array<f32>`.
- Vector data uses `#[repr(C)]` with `Pod` and `Zeroable` for `bytemuck` transfer.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.
**Assigned Agency Role**: Graphics Engineer
**Files to Create/Edit**: Cargo.toml, src/physics/gpu.rs, shaders/compute.wgsl
**Reference**: Tier 2 Projects - GPU Acceleration (Compute Shaders)

## Quality Requirements
- [ ] Development strictly follows Rust-native memory management.
- [ ] Multithreading logic must handle synchronization safely without deadlocks.
- [ ] Tasks are scoped to be implementable by a developer in 30-60 minutes where possible, broken down further if needed.

## Technical Notes
**Development Stack**: Rust, rayon, wide, wgpu, bytemuck.
**Special Instructions**: Ensure all memory constraints (e.g., DOD SoA, `wide` instead of `std::simd`) are strictly followed.
**Timeline Expectations**: Maintain 95% on-time delivery for 1.0.0 core roadmap.
