# Title: Epic: Elevate Worm Engine to State-of-the-Art (SOTA) Market Leadership

## Labels
epic, strategic, performance, architecture, sota

## Body

### Executive Summary
As the Studio Producer, this Epic outlines the strategic initiatives required to evolve Worm Engine into a State-of-the-Art (SOTA), high-performance physics engine. Our primary objective is to maintain a 95% on-time delivery for the 1.0.0 roadmap while ensuring a 25% portfolio ROI with balanced risk. To accomplish this without derailing our release cadence, we will integrate advanced architectural features as modular add-ons and strategic refactors.

### Strategic Priorities and Task Breakdown

#### 1. Continuous Collision Detection (CCD) Implementation
**Description:** Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
**Acceptance Criteria:**
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.
**Assigned Agency Role:** Physics Engineer
**Files to Create/Edit:**
- `src/physics/ccd.rs`
- `src/physics/mod.rs`
- `src/physics/world.rs`
**Technical Notes:**
- Needs to resolve/issue/test this feature as a modular add-on to not block 1.0.0.

#### 2. Data-Oriented Design (DOD) & ECS Refactoring
**Description:** Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.
**Acceptance Criteria:**
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.
**Assigned Agency Role:** Architecture Lead
**Files to Create/Edit:**
- `src/physics/rigid_body.rs`
- `src/physics/world.rs`
- `src/physics/components.rs`
**Technical Notes:**
- The `World` struct must explicitly use concrete types for its core SoA memory arrays (`Vec<Vector3d>` for positions, velocities, accelerations, forces; `Vec<f32>` for masses; `Vec<Polygon>` for shapes) and directly own these arrays, completely replacing and removing the usage of `RigidBodyComponents`.

#### 3. Multithreading and SIMD Vectorization
**Description:** Integrate `rayon` for task-based parallelism and SIMD for vectorizing math operations in the physics pipeline.
**Acceptance Criteria:**
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- Thread synchronization does not introduce unresolvable latency.
**Assigned Agency Role:** Systems Engineer
**Files to Create/Edit:**
- `Cargo.toml`
- `src/geometry/vector.rs`
- `src/physics/world.rs`
**Technical Notes:**
- Must use the `wide` crate for cross-platform SIMD vectorization (do NOT use `std::simd`).
- When iterating multiple mutable Struct of Arrays (SoA) vectors in `rayon`, chain the iterators using `.par_iter_mut().zip(...)` for standard iteration, or `.par_chunks_mut(chunk_size).zip(...)` when processing SIMD chunks. Note that `par_iter_mut()` takes exactly zero arguments.
- SIMD vectorization using the `wide` crate must be applied over SoA chunks, explicitly avoiding Array of Structures (AoS) SIMD on individual math primitives like `Vector3d`.
- Explicitly import `use wide::f32x4;` in `src/geometry/vector.rs`.
- When converting `wide::f32x4` SIMD vectors to arrays, use the explicit `.to_array()` method rather than `.into()`.

#### 4. Cross-Platform Determinism Setup
**Description:** Implement strict floating-point math control and deterministic solver execution across multiple architectures.
**Acceptance Criteria:**
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.
**Assigned Agency Role:** Systems Engineer
**Files to Create/Edit:**
- `src/physics/math.rs`
- `src/physics/constants.rs`
- Tests related to cross-platform execution.
**Technical Notes:**
- Utilize `libm` for deterministic math operations across platforms.

#### 5. GPU Acceleration (Compute Shaders) Integration
**Description:** Future-proof the engine by integrating WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids.
**Acceptance Criteria:**
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.
**Assigned Agency Role:** Graphics Engineer
**Files to Create/Edit:**
- `Cargo.toml`
- `src/physics/gpu.rs`
- `shaders/compute.wgsl`
**Technical Notes:**
- Utilize `wgpu` (~v0.19) with WGSL.
- For WGSL shaders, avoid 16-byte alignment crashes by using flat `array<f32>` instead of `vec3<f32>` to match standard Rust `bytemuck` arrays.
