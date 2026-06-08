# Worm Engine Task List

## Technical Notes
- `wgpu` (~v0.19) has 16-byte alignment issues. Use flat `array<f32>` instead of `vec3<f32>` in WGSL shaders.
- `wide` crate must be used for SIMD. It must be applied over SoA chunks, explicitly avoiding AoS SIMD on individual math primitives.
- `rayon` tuple chaining requirements: chain iterators using `.par_chunks_mut(chunk_size).zip(...)` when processing SIMD chunks. Note that `par_iter_mut()` takes exactly zero arguments.

## Quality Requirements
- No server startup commands (assume development server running).
- Strict use of approved image sources (Unsplash, picsum.photos) with NO Pexels due to 403 errors.

## Tasks

### Continuous Collision Detection (CCD) Implementation
**Assigned Agency Role**: Physics Engineer
**Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
**Acceptance Criteria**:
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.
**Files to Create/Edit**: `src/physics/ccd.rs`, `src/physics/mod.rs`, `src/physics/world.rs`
- [ ] Implement Task

### Data-Oriented Design (DOD) & ECS Refactoring
**Assigned Agency Role**: Architecture Lead
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.
**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.
**Files to Create/Edit**: `src/physics/rigid_body.rs`, `src/physics/world.rs`, `src/physics/components.rs`
- [ ] Implement Task

### Multithreading and SIMD Vectorization
**Assigned Agency Role**: Systems Engineer
**Description**: Integrate `rayon` for task-based parallelism and `std::simd` (or `wide`) for vectorizing math operations in the physics pipeline.
**Acceptance Criteria**:
- Engine scales linearly up to 16 threads on supported hardware.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- Thread synchronization does not introduce unresolvable latency.
**Files to Create/Edit**: `Cargo.toml`, `src/geometry/vector.rs`, `src/physics/world.rs`
- [ ] Implement Task

### Cross-Platform Determinism Setup
**Assigned Agency Role**: Systems Engineer
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures.
**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.
**Files to Create/Edit**: `src/physics/math.rs`, `src/physics/constants.rs`, Tests related to cross-platform execution.
- [ ] Implement Task

### GPU Acceleration (Compute Shaders) Integration
**Assigned Agency Role**: Graphics Engineer
**Description**: Future-proof the engine by integrating WGPU for GPU-accelerated compute shaders, initially targeting massive scale simulations like soft-bodies or fluids.
**Acceptance Criteria**:
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution.
**Files to Create/Edit**: `Cargo.toml`, `src/physics/gpu.rs`, `shaders/compute.wgsl`
- [ ] Implement Task
