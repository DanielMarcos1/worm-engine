# Cross-Platform Determinism Setup

## Labels
determinism, ci

## Body
**Description**: Implement strict floating-point math control and deterministic solver execution across multiple architectures using `libm`.

**Acceptance Criteria**:
- Simulation yields identical results across different CPU architectures.
- CI testing pipeline includes deterministic behavior checks.
- Fallback mechanisms for non-deterministic math functions are implemented.

**Assigned Agency Role**:
Systems Engineer needs to take care of resolving, issuing, and testing this feature.

**Files to Create/Edit**:
- src/physics/math.rs
- src/physics/constants.rs
- Tests related to cross-platform execution.

**Quality Requirements**:
- Must pass `cargo check` cleanly
- Must pass `cargo test` suite
- No background processes in any commands - NEVER append `&`
- No server startup commands - assume development server running
- Include Playwright screenshot testing: `./qa-playwright-capture.sh http://localhost:8000 public/qa-screenshots`
- Images from approved sources (Unsplash, https://picsum.photos/) - NO Pexels (403 errors)
- Iterating multiple mutable SoA arrays in `rayon` must chain `.par_iter_mut().zip(...)`
- WGSL shaders must avoid 16-byte alignment crashes by using flat `array<f32>` and Rust structs must use `#[repr(C)]`, `Pod`, and `Zeroable`.
