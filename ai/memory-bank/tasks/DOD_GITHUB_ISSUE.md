# Data-Oriented Design (DOD) & ECS Refactoring
## Labels
architecture, refactoring
## Body
**Description**:
Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.

**Acceptance Criteria**:
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures without intermediate component structs like `RigidBodyComponents`.
- `World` directly owns core memory arrays (e.g., `Vec<Vector3d>`, `Vec<f32>`).

**Assigned Agency Role**:
**Architecture Lead** needs to resolve/issue/test this feature.

**Files to Create/Edit**:
- src/physics/rigid_body.rs
- src/physics/world.rs
- explicitly mandate the removal of src/physics/components.rs

**Reference**:
Tier 1 Projects - Data-Oriented Design (DOD) & ECS Compatibility
