# Title
Data-Oriented Design (DOD) & ECS Refactoring

## Labels
architecture, refactoring

## Body
**Description**: Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.

**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

**Technical Constraints**:
- The `World` struct in `src/physics/world.rs` must utilize a Data-Oriented Design (DOD) with a Structure of Arrays (SoA) layout, directly owning core memory arrays (`Vec<Vector3d>` for positions, velocities, etc., `Vec<f32>` for masses, `Vec<Polygon>` for shapes) without intermediate component structs like `RigidBodyComponents`.

**Assignment**:
Data-Oriented Design (DOD) & ECS Refactoring needs to be resolved/issued/tested by the Architecture Lead.
