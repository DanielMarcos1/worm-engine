# Strategic Portfolio Plan: Worm Engine SOTA Integration

## Labels
strategic, enhancement, performance, sota

## Body
The Worm Engine is currently tracking towards 1.0.0. Core mechanics (Rigid/Soft bodies) are in progress, but we need strategic adjustments to achieve SOTA status. Expected 35% ROI. We are positioning perfectly for the anticipated market shift toward highly scalable, data-oriented multiplayer experiences.

### Strategic Objectives
Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art (SOTA), high-performance solution capable of capturing top-tier market share in the simulation and gaming sectors. Align our technical milestones to deliver hyper-performance, deterministic simulation, and memory efficiency, positioning us for the anticipated market shift toward highly scalable, data-oriented multiplayer experiences.

Projected 35% ROI through competitive licensing and modular architecture. Maintains our 95% on-time delivery benchmark for the core v1.0.0 roadmap milestones.

### What Needs to be Done

**1. Continuous Collision Detection (CCD) (Target: v0.4.0)**
- Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
- 0% tunneling observed at velocities up to 1000m/s.
- CCD pipeline integrates with the existing collision detection system.
- Performance impact remains within acceptable bounds for high-speed simulations.

**2. Data-Oriented Design (DOD) & ECS Refactoring (Target: v0.6.0)**
- Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

**3. Multithreading Implementation**
- Integrate `rayon` for task-based parallelism. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples.
- Engine scales linearly up to 16 threads on supported hardware.
- Thread synchronization does not introduce unresolvable latency.
- SIMD vectorization utilizes a Structure of Arrays (SoA) approach rather than AoS on individual math primitives.

**4. SIMD Vectorization Implementation (Target: v0.6.0)**
- Integrate `wide` for vectorizing math operations in the physics pipeline. Defer until DOD refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.
- Core math operations (vector additions, dot products, cross products) utilize SIMD instructions.
- SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.

**5. Cross-Platform Determinism Setup (Target: v0.7.0/v1.0.0)**
- Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm`.
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

**6. GPU Acceleration (Compute Shaders) Integration (Target: v0.8.0/v1.0.0)**
- Integrate `wgpu` (~v0.19) for GPU-accelerated compute shaders targeting massive scale simulations. `Vector3d` sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives. In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>`.
- Basic WGPU context is established and integrated into the build.
- A prototype compute shader runs and passes data back to the CPU physics pipeline.
- CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.

### Assigned Agency Role
- **Physics Engineer** needs to take care of resolving, issuing, and testing Continuous Collision Detection (CCD).
- **Architecture Lead** needs to take care of resolving, issuing, and testing Data-Oriented Design (DOD) & ECS Refactoring.
- **Systems Engineer** needs to take care of resolving, issuing, and testing Multithreading Implementation, SIMD Vectorization Implementation, and Cross-Platform Determinism Setup.
- **Graphics Engineer** needs to take care of resolving, issuing, and testing GPU Acceleration (Compute Shaders) Integration.

### Files to Create/Edit
- `src/physics/ccd.rs`
- `src/physics/mod.rs`
- `src/physics/world.rs`
- `src/physics/rigid_body.rs`
- `src/physics/components.rs`
- `Cargo.toml`
- `src/geometry/vector.rs`
- `src/physics/math.rs`
- `src/physics/constants.rs`
- `src/physics/gpu.rs`
- `shaders/compute.wgsl`
