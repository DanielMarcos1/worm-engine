# Worm Engine Task List

## Quality Requirements
- `cargo check` must pass without errors.
- `cargo test` must pass successfully.

## Tasks

### 1. Data-Oriented Design (DOD) & ECS Refactoring
- Refactor `RigidBodyComponents` to a pure Structure of Arrays (SoA) layout. Deconstruct 3D vectors into independent flat arrays (e.g. `velocities_x`, `velocities_y`, etc.).
- Update `World` struct to delegate SoA layout to `RigidBodyComponents` and remove redundant fields.
- Refactor `World::step()` to iterate over independent flat arrays using Rayon `.par_iter_mut().zip(...)`.
- [Data-Oriented Design (DOD) & ECS Refactoring] needs to be resolved/issued/tested by the [Architecture Lead]

### 2. Continuous Collision Detection (CCD) Implementation
- Implement TOI calculation pipeline in `src/physics/ccd.rs` to prevent tunneling.
- Integrate CCD into the main collision detection system.
- [Continuous Collision Detection (CCD) Implementation] needs to be resolved/issued/tested by the [Physics Engineer]

### 3. Multithreading and SIMD Vectorization
- Integrate `wide` crate for SIMD vectorization with the new SoA layout.
- Vectorize core math operations in `World::step()` using `.par_chunks_mut(4).zip(...)`.
- [Multithreading and SIMD Vectorization] needs to be resolved/issued/tested by the [Systems Engineer]

### 4. Cross-Platform Determinism Setup
- Implement deterministic math functions using `libm`.
- Set up fallback mechanisms for non-deterministic functions.
- [Cross-Platform Determinism Setup] needs to be resolved/issued/tested by the [Systems Engineer]

### 5. GPU Acceleration (Compute Shaders) Integration
- Integrate WGPU context into the build.
- Write compute shaders using flat `array<f32>` arrays instead of `array<vec3<f32>>`.
- [GPU Acceleration (Compute Shaders) Integration] needs to be resolved/issued/tested by the [Graphics Engineer]
