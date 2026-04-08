# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**: Expand scope to include Tier 1 State-of-the-Art (SOTA) initiatives—Continuous Collision Detection (CCD), DOD/ECS compatibility, and Multithreading/SIMD—and Tier 2 Experimental growth initiatives—Cross-Platform Determinism and GPU Acceleration.
**Technical Stack**: Rust, rayon, wide, libm, wgpu, wgsl
**Target Timeline**: v1.0.0 Roadmap (Current features are on v0.4.0 to v0.7.0/1.0.0 tracks based on SOTA Scope Issue)

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
**Assignment**: Continuous Collision Detection (CCD) Implementation needs to be resolved/issued/tested by the Physics Engineer

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
**Assignment**: Data-Oriented Design (DOD) & ECS Refactoring needs to be resolved/issued/tested by the Architecture Lead

### [ ] Task 3: Multithreading and SIMD Vectorization
**Description**: Integrate `rayon` for task-based parallelism and `wide` (replacing std::simd) for vectorizing math operations in the physics pipeline.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- Thread synchronization does not introduce unresolvable latency.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization
**Assignment**: Multithreading and SIMD Vectorization needs to be resolved/issued/tested by the Systems Engineer

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
**Assignment**: Cross-Platform Determinism Setup needs to be resolved/issued/tested by the Systems Engineer

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
**Assignment**: GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer

## Quality Requirements
- [ ] No background processes in any commands - NEVER append `&`
- [ ] Cargo check passes successfully: `cargo check`
- [ ] Cargo test passes successfully without introducing regressions: `cargo test`

## Technical Notes
**Development Stack**: Rust, rayon, wide, libm, wgpu (~v0.19) with WGSL
**Special Instructions**:
- Use `wide` crate (e.g., `wide::f32x4`) for SIMD operations instead of `std::simd`.
- When iterating over multiple mutable SoA arrays in parallel with `rayon`, chain `.par_chunks_mut(4).zip(...)` or `.par_iter_mut().zip(...)` instead of passing a tuple to `.into_par_iter()`.
- Send `Vector3d` data to WGSL via `bytemuck` using `#[repr(C)]` with `Pod` and `Zeroable` derives, and use a flat `array<f32>` (indexing by 3) in WGSL.
**Timeline Expectations**: Modular integration required to keep the v1.0.0 roadmap moving on time.