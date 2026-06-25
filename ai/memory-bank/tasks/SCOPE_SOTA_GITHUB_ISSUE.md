# Title: Worm Engine SOTA Integration (v1.0.0 Roadmap)

## Labels
enhancement, high-priority, strategic

## Body

**Original Requirements**: Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art (SOTA), high-performance solution. Align our technical milestones to deliver hyper-performance, deterministic simulation, and memory efficiency, positioning us for the anticipated market shift toward highly scalable, data-oriented multiplayer experiences.
**Technical Stack**: Rust, rayon, wide, libm, wgpu (~v0.19), WGSL
**Target Timeline**: Integrated into v0.4.0 (CCD), v0.6.0 (DOD, SIMD), v0.7.0/v1.0.0 (Determinism), and v0.8.0/v1.0.0 (GPU) while maintaining 95% on-time delivery benchmark for the current roadmap milestones.

### Task 1: Continuous Collision Detection (CCD)
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.
**Files to Create/Edit**:
- src/physics/ccd.rs
- src/physics/mod.rs
- src/physics/world.rs
**Assigned Agency Role**: Physics Engineer

### Task 2: Data-Oriented Design (DOD) & ECS Refactoring (30-60 minutes)
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.
**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs
**Assigned Agency Role**: Architecture Lead

### Task 3: Multithreading Implementation
**Description**: Integrate `rayon` for task-based parallelism. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Thread synchronization does not introduce unresolvable latency.
- SIMD vectorization utilizes a Structure of Arrays (SoA) approach rather than AoS on individual math primitives.
**Files to Create/Edit**:
- Cargo.toml
- src/physics/world.rs
**Assigned Agency Role**: Systems Engineer

### Task 4: SIMD Vectorization Implementation
**Description**: Integrate `wide` for vectorizing math operations in the physics pipeline. Defer until DOD refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.
**Acceptance Criteria**:
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.
**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs
**Assigned Agency Role**: Systems Engineer

### Task 5: Cross-Platform Determinism Setup
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm`.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.
**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs
**Assigned Agency Role**: Systems Engineer

### Task 6: GPU Acceleration (Compute Shaders) Integration
**Description**: Integrate `wgpu` (~v0.19) for GPU-accelerated compute shaders targeting massive scale simulations. `Vector3d` sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives. In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>`.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.
**Files to Create/Edit**:
- Cargo.toml
- src/physics/gpu.rs
- shaders/compute.wgsl
**Assigned Agency Role**: Graphics Engineer

## Quality Requirements
- Must pass `cargo check` cleanly.
- Must pass `cargo test` suite.
- No background processes in any commands - NEVER append `&`.
- Playwright screenshot testing (`./qa-playwright-capture.sh http://localhost:8000 public/qa-screenshots`) is required for front-end tasks (if applicable). Image sources must be Unsplash or `https://picsum.photos/` (No Pexels).
- Iterating multiple mutable SoA arrays in `rayon` must chain `.par_iter_mut().zip(...)`.
- WGSL shaders must avoid 16-byte alignment crashes by using flat `array<f32>` and Rust structs must use `#[repr(C)]`, `Pod`, and `Zeroable`.
