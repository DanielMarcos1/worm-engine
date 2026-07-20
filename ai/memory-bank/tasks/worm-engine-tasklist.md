# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**: Implement Tier 1 State-of-the-Art (SOTA) initiatives (Continuous Collision Detection, DOD/ECS compatibility, Multithreading/SIMD) and Tier 2 (Determinism, GPU) to capture top-tier market share.
**Technical Stack**: Rust, rayon, wide, wgpu, wgsl
**Target Timeline**: v1.0.0 Roadmap and beyond

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
- src/physics/mod.rs

**Reference**: Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility

### [ ] Task 3: Multithreading and SIMD Vectorization
**Description**: Integrate `rayon` for task-based parallelism and `wide` crate for vectorizing math operations in the physics pipeline.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations utilize SIMD instructions via the `wide` crate.
- Thread synchronization does not introduce unresolvable latency.

**Files to Create/Edit**:
- Cargo.toml
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization

### [ ] Task 4: Cross-Platform Determinism Setup
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures.
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

## Quality Requirements
- [ ] No compilation warnings (run `cargo clippy`)
- [ ] Rust code follows established formatting rules
- [ ] Implementations must preserve DOD and SoA invariants
- [ ] Multi-threaded operations using `rayon` must chain iterations properly

## Technical Notes
**Development Stack**: Rust, rayon, wide, wgpu, wgsl
**Special Instructions**:
- Must use `wide` instead of `std::simd`
- Must use flat `array<f32>` in WGSL shaders instead of `vec3<f32>`
- Data sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives
**Timeline Expectations**: 1.0.0 milestones as per specifications.
