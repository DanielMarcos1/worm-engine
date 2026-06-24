# Data-Oriented Design (DOD) & ECS Refactoring

## Labels
architecture, refactoring

## Body
Refactor core engine structures to support Data-Oriented Design, making it compatible with modern ECS architectures like Bevy and Flecs. Must use a Structure of Arrays (SoA) approach.

**Acceptance Criteria:**
- Memory layout is optimized for cache coherency.
- API allows integration with a standard ECS in under 2 hours.
- Core systems (e.g., rigid body updates) operate on flat arrays or similar DOD structures.

**Assigned Agency Role:**
**Architecture Lead** needs to resolve/issue/test this feature.

**Files to Create/Edit:**
- src/physics/rigid_body.rs
- src/physics/world.rs
- src/physics/components.rs

**Quality Requirements:**
- [ ] Must pass `cargo check` cleanly
- [ ] Must pass `cargo test` suite
- [ ] No background processes in any commands - NEVER append `&`
- [ ] Include Playwright screenshot testing: `./qa-playwright-capture.sh http://localhost:8000 public/qa-screenshots`
- [ ] Images from approved sources (Unsplash, https://picsum.photos/) - NO Pexels (403 errors)
