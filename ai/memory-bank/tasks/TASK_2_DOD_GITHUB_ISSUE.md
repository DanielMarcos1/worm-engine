# Task 2: Data-Oriented Design (DOD) & ECS Refactoring (30-60 minutes)
## Labels
architecture, refactoring
## Body
### Description
Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs.

### Acceptance Criteria
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

### Files to Create/Edit
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

### Reference
Issue Task 2 DOD

### Assigned Agency Role
Data-Oriented Design (DOD) & ECS Refactoring (30-60 minutes) needs to be resolved/issued/tested by the **Architecture Lead**.
