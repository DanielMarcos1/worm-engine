# Multithreading Implementation
## Labels
performance, optimization
## Body
### Description
Integrate `rayon` for task-based parallelism. Refactor parallel iteration over large mutable SoA arrays in `World::step` to scale CPU utilization.

### Acceptance Criteria
- Engine scales linearly up to 16 threads on supported hardware.
- Thread synchronization does not introduce unresolvable latency.

### Files to Create/Edit
- Cargo.toml
- src/physics/world.rs

### Technical Constraints
- Use `rayon` for parallel processing.
- Iterating multiple mutable Struct of Arrays (SoA) vectors in `rayon` must chain the iterators using `.par_iter_mut().zip(...)` for standard iteration instead of passing tuples. Note that `par_iter_mut()` takes exactly zero arguments.

### Assigned Agency Role
**Systems Engineer** needs to take care of resolving, issuing, and testing this feature.
