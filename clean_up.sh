sed -i '/pub positions_x: Vec<f32>,/,/pub forces_z: Vec<f32>,/d' src/physics/world.rs
sed -i '/positions_x: Vec::new(),/,/forces_z: Vec::new(),/d' src/physics/world.rs
sed -i 's/\- \[x\] No background processes in any commands - NEVER append `\- \[ \] No background processes in any commands - NEVER append `\&`/\- \[x\] No background processes in any commands - NEVER append `\&`/g' ai/memory-bank/tasks/worm-engine-tasklist.md
