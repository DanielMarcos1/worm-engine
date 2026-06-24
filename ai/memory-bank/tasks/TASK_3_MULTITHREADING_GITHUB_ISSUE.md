# Multithreading Implementation

## Labels
performance, optimization

## Body
Integrate `rayon` for task-based parallelism. Refactor parallel iteration over large mutable SoA arrays in `World::step` to chain `.par_iter_mut().zip(...)` instead of passing tuples.

**Acceptance Criteria:**
- Engine scales linearly up to 16 threads on supported hardware.
- Thread synchronization does not introduce unresolvable latency.

**Assigned Agency Role:**
**Systems Engineer** needs to resolve/issue/test this feature.

**Files to Create/Edit:**
- Cargo.toml
- src/physics/world.rs

**Quality Requirements:**
- [ ] Must pass `cargo check` cleanly
- [ ] Must pass `cargo test` suite
- [ ] No background processes in any commands - NEVER append `&`
- [ ] Iterating multiple mutable SoA arrays in `rayon` must chain `.par_iter_mut().zip(...)`
- [ ] Include Playwright screenshot testing: `./qa-playwright-capture.sh http://localhost:8000 public/qa-screenshots`
- [ ] Images from approved sources (Unsplash, https://picsum.photos/) - NO Pexels (403 errors)
