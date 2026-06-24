# Data-Oriented Design (DOD) & ECS Refactoring
## Labels
architecture, refactoring
## Body
### Description
Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs. Ensure task is scoped to be implementable by a developer in 30-60 minutes.

### Acceptance Criteria
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

### Assigned Agency Role
**Architecture Lead** needs to resolve/issue/test this feature.

### Files to Create/Edit
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs
