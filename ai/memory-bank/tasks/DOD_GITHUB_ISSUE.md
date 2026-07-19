# Title
Data-Oriented Design (DOD) & ECS Refactoring

## Labels
architecture, refactoring

## Body
## Description
Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.
The `World` struct in `src/physics/world.rs` utilizes a Data-Oriented Design (DOD) with a Structure of Arrays (SoA) layout, directly owning core memory arrays (`Vec<Vector3d>` for positions, velocities, etc., `Vec<f32>` for masses, `Vec<Polygon>` for shapes) without intermediate component structs like `RigidBodyComponents`.

## Acceptance Criteria
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.
- Ensure you fully clean up the codebase by deleting the obsolete source file (e.g., `src/physics/components.rs`), removing its module declaration from the parent module (`src/physics/mod.rs`), and updating all dependent imports.

## Assigned Agency Role
**Architecture Lead** needs to resolve/issue/test this feature.

## Files to Create/Edit
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs
- src/physics/mod.rs

## Reference
Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility
