# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**: "Evolve the Worm Engine into a state-of-the-art (SOTA), high-performance 3D physics engine capable of capturing top-tier market share in the simulation and gaming sectors without compromising the core v1.0.0 delivery timeline." "Expanding our scope to include critical features like Continuous Collision Detection (CCD) and ECS compatibility".
**Technical Stack**: Rust, `rayon` (task-based parallelism), `wide` (cross-platform SIMD), `libm` (deterministic math), `wgpu` & WGSL (GPU compute acceleration).
**Target Timeline**: v0.4.0 (CCD), v0.6.0 (DOD & SIMD), v0.7.0/v1.0.0 (Determinism), v0.8.0/v1.0.0 Experimental (GPU Acceleration).

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
**Assigned Role**: **[Physics Engineer]** needs to resolve/issue/test this feature.

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
**Assigned Role**: **[Architecture Lead]** needs to resolve/issue/test this feature.

### [ ] Task 3: Multithreading and SIMD Vectorization
**Description**: Integrate `rayon` for task-based parallelism and SIMD for vectorizing math operations in the physics pipeline (using Structure of Arrays approach, deferring SIMD until DOD refactoring is complete).
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions (via `wide` crate).
- Thread synchronization does not introduce unresolvable latency.
- SIMD vectorization utilizes a Structure of Arrays (SoA) approach rather than AoS on individual math primitives.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization
**Assigned Role**: **[Systems Engineer]** needs to resolve/issue/test this feature.

### [ ] Task 4: Cross-Platform Determinism Setup
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm`.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs
- Tests related to cross-platform execution

**Reference**: Tier 2 Projects - Cross-Platform Determinism
**Assigned Role**: **[Systems Engineer]** needs to resolve/issue/test this feature.

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
**Assigned Role**: **[Graphics Engineer]** needs to resolve/issue/test this feature.

## Quality Requirements
- [ ] No background processes in any commands - NEVER append `&`
- [ ] No server startup commands - assume development server running
- [ ] Must run and pass all standard Rust checks (`cargo check`, `cargo test`)
- [ ] Performance benchmarks must be established for Tier 1 initiatives
- [ ] Fallback to single-threaded, discrete collision detection if multithreading synchronization fails
- [ ] Ensure any floating-point math added follows determinism guidelines where applicable

## Technical Notes
**Development Stack**: Rust, rayon, wide, libm, wgpu, wgsl
**Special Instructions**: Defer SIMD vectorization until DOD refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying AoS SIMD to individual math primitives. Ensure CCD and GPU features are modular to not block 1.0.0.
**Timeline Expectations**: Integration within the v1.0.0 roadmap milestones (v0.4.0, v0.6.0, v0.7.0, v0.8.0), aiming for 95% on-time delivery.
