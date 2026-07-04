# Title
Worm Engine State-of-the-Art (SOTA) Integration Tasks

## Labels
enhancement, performance, strategic

## Body
### Executive Summary
Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art (SOTA), high-performance solution. Align our technical milestones to deliver hyper-performance, deterministic simulation, and memory efficiency, positioning us for the anticipated market shift toward highly scalable, data-oriented multiplayer experiences.

### Technical Deliverables

#### Task 1: Continuous Collision Detection (CCD)
- **Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
- **Acceptance Criteria**:
  - 0% tunneling observed at velocities up to 1000m/s.
  - CCD pipeline integrates with the existing collision detection system.
  - Performance impact remains within acceptable bounds for high-speed simulations.
- **Files to Create/Edit**:
  - src/physics/ccd.rs
  - src/physics/mod.rs
  - src/physics/world.rs
- **Assigned Agency Role**: Continuous Collision Detection (CCD) needs to be resolved/issued/tested by the Physics Engineer

#### Task 2: Data-Oriented Design (DOD) & ECS Refactoring
- **Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.
- **Acceptance Criteria**:
  - Memory layout is optimized for cache coherency.
  - API allows integration with a standard ECS in under 2 hours.
  - Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.
- **Files to Create/Edit**:
  - src/physics/rigid_body.rs
  - src/physics/world.rs
  - src/physics/components.rs
- **Assigned Agency Role**: Data-Oriented Design (DOD) & ECS Refactoring (30-60 minutes) needs to be resolved/issued/tested by the Architecture Lead

#### Task 3: Multithreading Implementation
- **Description**: Integrate `rayon` for task-based parallelism. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples.
- **Acceptance Criteria**:
  - Engine scales linearly up to 16 threads on supported hardware.
  - Thread synchronization does not introduce unresolvable latency.
  - SIMD vectorization utilizes a Structure of Arrays (SoA) approach rather than AoS on individual math primitives.
- **Files to Create/Edit**:
  - Cargo.toml
  - src/physics/world.rs
- **Assigned Agency Role**: Multithreading Implementation needs to be resolved/issued/tested by the Systems Engineer

#### Task 4: SIMD Vectorization Implementation
- **Description**: Integrate `wide` for vectorizing math operations in the physics pipeline. Defer until DOD refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.
- **Acceptance Criteria**:
  - Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
  - SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.
- **Files to Create/Edit**:
  - Cargo.toml
  - src/geometry/vector.rs
  - src/physics/world.rs
- **Assigned Agency Role**: SIMD Vectorization Implementation needs to be resolved/issued/tested by the Systems Engineer

#### Task 5: Cross-Platform Determinism Setup
- **Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm`.
- **Acceptance Criteria**:
  - Simulation yields identical results across different CPU architectures.
  - CI testing pipeline includes deterministic behavior checks.
  - Fallback mechanisms for non-deterministic math functions are implemented.
- **Files to Create/Edit**:
  - src/physics/math.rs
  - src/physics/constants.rs
- **Assigned Agency Role**: Cross-Platform Determinism Setup needs to be resolved/issued/tested by the Systems Engineer

#### Task 6: GPU Acceleration (Compute Shaders) Integration
- **Description**: Integrate `wgpu` (~v0.19) for GPU-accelerated compute shaders targeting massive scale simulations. `Vector3d` sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives. In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>`.
- **Acceptance Criteria**:
  - Basic WGPU context is established and integrated into the build.
  - A prototype compute shader runs and passes data back to the CPU physics pipeline.
  - CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.
- **Files to Create/Edit**:
  - Cargo.toml
  - src/physics/gpu.rs
  - shaders/compute.wgsl
- **Assigned Agency Role**: GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer
