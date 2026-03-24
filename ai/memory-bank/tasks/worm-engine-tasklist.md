# Worm Engine Development Tasks

## Specification Summary
**Original Requirements**:
- Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art, high-performance solution.
- Maintain 95% on-time delivery benchmark for the 1.0.0 roadmap.
- Implement Continuous Collision Detection (CCD) to prevent "tunneling" at high velocities (0% tunneling at 1000m/s).
- Implement Data-Oriented Design (DOD) & ECS Compatibility with an API integration time under 2 hours.
- Implement Multithreading and SIMD Vectorization using `rayon` and `std::simd` with linear scaling up to 16 threads.
- Implement Cross-Platform Determinism for competitive multiplayer and rollback netcode.
- Implement GPU Acceleration (Compute Shaders) for massive scale simulations via WGPU integration.

**Technical Stack**: Rust, `rayon`, `std::simd`, WGPU
**Target Timeline**: To be integrated within the 1.0.0 roadmap without blocking core deliveries, mitigating risks with strict agile milestones.

## Development Tasks

### [ ] Task 1: Continuous Collision Detection (CCD) Implementation (30-60 minutes)
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

**Files to Create/Edit**:
- src/physics/ccd.rs (new file)
- src/physics/mod.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Continuous Collision Detection (CCD)
**Assigned Agency Role**: **Physics Engineer** needs to resolve/issue/test this feature.

### [ ] Task 2: Data-Oriented Design (DOD) & ECS Refactoring (30-60 minutes)
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs (new file)

**Reference**: Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility
**Assigned Agency Role**: **Architecture Lead** needs to resolve/issue/test this feature.

### [x] Task 3: Multithreading and SIMD Vectorization (30-60 minutes)
**Description**: Integrate `rayon` for task-based parallelism and `std::simd` for vectorizing math operations in the physics pipeline.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions. (Deferred to DOD refactoring to avoid AoS overhead)
- Thread synchronization does not introduce unresolvable latency.

**Files to Create/Edit**:
- Cargo.toml (add `rayon`, enable SIMD features if needed)
- src/geometry/vector.rs
- src/physics/world.rs

**Reference**: Tier 1 Projects - Multithreading and SIMD Vectorization
**Assigned Agency Role**: **Systems Engineer** needs to resolve/issue/test this feature.

### [ ] Task 4: Cross-Platform Determinism Setup (30-60 minutes)
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

**Files to Create/Edit**:
- src/physics/math.rs (new file for deterministic operations)
- src/physics/constants.rs
- Tests related to cross-platform execution.

**Reference**: Tier 2 Projects - Cross-Platform Determinism
**Assigned Agency Role**: **Systems Engineer** needs to resolve/issue/test this feature.

### [ ] Task 5: GPU Acceleration (Compute Shaders) Integration (30-60 minutes)
**Description**: Future-proof the engine by integrating WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.

**Files to Create/Edit**:
- Cargo.toml (add `wgpu`)
- src/physics/gpu.rs (new file)
- shaders/compute.wgsl (new file)

**Reference**: Tier 2 Projects - GPU Acceleration (Compute Shaders)
**Assigned Agency Role**: **Graphics Engineer** needs to resolve/issue/test this feature.

## Quality Requirements
- [ ] No background processes in any commands - NEVER append `&`
- [ ] No server startup commands - assume development server running
- [ ] Must run and pass all standard Rust checks (`cargo check`, `cargo test`)
- [ ] Performance benchmarks must be established for Tier 1 initiatives
- [ ] Fallback to single-threaded, discrete collision detection if multithreading synchronization fails
- [ ] Ensure any floating-point math added follows determinism guidelines where applicable

## Technical Notes
**Development Stack**: Rust, `rayon`, `std::simd`, WGPU
**Special Instructions**: If CCD or GPU acceleration threatens the core roadmap, they will be modularized as optional add-ons rather than blocking 1.0.0. Focus on front-loading investment in architectural refactoring (DOD) to minimize technical debt.
**Timeline Expectations**: Maintain 95% on-time delivery. Tasks should be structured as agile milestones to quickly identify and mitigate risks.

### [x] Task 0: Project Scope Analysis and SOTA Strategy Formulation
**Description**: Analyze the project scope and create an issue on GitHub detailing what needs to be done to maintain the project scope and achieve State-of-the-Art (SOTA) level for the Worm Engine.
**Acceptance Criteria**:
- A comprehensive strategic review document (`ISSUE_SCOPE_AND_SOTA.md`) is created adopting the "Studio Producer" persona.
- The review uses the `Strategic Portfolio Review` template to analyze and balance the core v1.0.0 roadmap with Tier 1 & 2 SOTA initiatives.
- Resource allocation, strategic impact, and risk management are detailed.

**Files Created/Edited**:
- `ISSUE_SCOPE_AND_SOTA.md`

**Assigned Agency Role**: Studio Producer
