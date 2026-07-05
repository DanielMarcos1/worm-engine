# Title
Strategic Portfolio Plan: Elevating Worm Engine to State-of-the-Art (SOTA) Level

## Labels
enhancement, strategic, core-architecture, performance

## Body
### Executive Summary
**Strategic Objectives**: Evolve the Worm Engine from a functional 3D physics engine into a state-of-the-art, high-performance solution capable of capturing top-tier market share in the simulation and gaming sectors.
**Portfolio Value**: Securing high-speed simulation markets via CCD and modern ECS adoption via DOD will drive [Expected Integration Adoption Increase], significantly boosting long-term revenue impact. Front-loading investment in DOD minimizes future technical debt, positioning us for a 35% overall ROI with balanced risk management.
**Market Opportunity**: The current 3D physics engine landscape demands hyper-performance, deterministic simulation, and memory efficiency. Our current scope establishes a functional baseline, but without advanced features, we risk lagging behind modern ECS-native engines (e.g., Bevy, Flecs) and AAA physics standards.
**Resource Strategy**: Strategically reallocate resources to tackle advanced Data-Oriented Design (DOD) and SIMD vectorization without derailing our v1.0.0 delivery.

### What Needs to Be Done (Project Scope)

To maintain the project scope and achieve SOTA status, the following tasks must be completed with strict adherence to technical constraints:

**1. Continuous Collision Detection (CCD)**
- **Description**: Implement Continuous Collision Detection to prevent "tunneling" at high velocities. This involves calculating time of impact (TOI) between moving bodies.
- **Acceptance Criteria**: 0% tunneling observed at velocities up to 1000m/s. CCD pipeline integrates with the existing collision detection system. Performance impact remains within acceptable bounds for high-speed simulations.
- **Assigned Agency Role**: Needs to be resolved/issued/tested by the Physics Engineer

**2. Data-Oriented Design (DOD) & ECS Refactoring**
- **Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.
- **Acceptance Criteria**: Memory layout is optimized for cache coherency. API allows integration with a standard ECS in under 2 hours. Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.
- **Assigned Agency Role**: Needs to be resolved/issued/tested by the Architecture Lead

**3. Multithreading Implementation**
- **Description**: Integrate `rayon` for task-based parallelism. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples.
- **Acceptance Criteria**: Engine scales linearly up to 16 threads on supported hardware. Thread synchronization does not introduce unresolvable latency. SIMD vectorization utilizes a Structure of Arrays (SoA) approach rather than AoS on individual math primitives.
- **Assigned Agency Role**: Needs to be resolved/issued/tested by the Systems Engineer

**4. SIMD Vectorization Implementation**
- **Description**: Integrate `wide` for vectorizing math operations in the physics pipeline. Defer until DOD refactoring is complete to use a Structure of Arrays (SoA) approach. Avoid applying Array of Structures (AoS) SIMD to individual math primitives like `Vector3d`.
- **Acceptance Criteria**: Core math operations (vector additions, dot products, cross products) utilize SIMD instructions. SIMD implementation leverages SoA approach exclusively without overhead on individual primitives.
- **Assigned Agency Role**: Needs to be resolved/issued/tested by the Systems Engineer

**5. Cross-Platform Determinism Setup**
- **Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm`.
- **Acceptance Criteria**: Simulation yields identical results across different CPU architectures. CI testing pipeline includes deterministic behavior checks. Fallback mechanisms for non-deterministic math functions are implemented.
- **Assigned Agency Role**: Needs to be resolved/issued/tested by the Systems Engineer

**6. GPU Acceleration (Compute Shaders) Integration**
- **Description**: Integrate `wgpu` (~v0.19) for GPU-accelerated compute shaders targeting massive scale simulations. `Vector3d` sent via `bytemuck` must use `#[repr(C)]` with `Pod` and `Zeroable` derives. In WGSL, use a flat `array<f32>` (indexing by 3) instead of `array<vec3<f32>>`.
- **Acceptance Criteria**: Basic WGPU context is established and integrated into the build. A prototype compute shader runs and passes data back to the CPU physics pipeline. CPU pipeline remains stable during GPU execution with no 16-byte memory alignment crashes.
- **Assigned Agency Role**: Needs to be resolved/issued/tested by the Graphics Engineer
