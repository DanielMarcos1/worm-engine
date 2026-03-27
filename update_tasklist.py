import re
import os

filename = "ai/memory-bank/tasks/worm-engine-tasklist.md"
if os.path.exists(filename):
    with open(filename, 'r') as f:
        content = f.read()

    # Task 1
    content = re.sub(r'\*\*Assignment\*\*: Physics Engineer needs to resolve/issue/test this feature\.',
                     '**Assignment**: Continuous Collision Detection (CCD) needs to be resolved/issued/tested by the Physics Engineer', content)
    # Task 2
    content = re.sub(r'\*\*Assignment\*\*: Architecture Lead needs to resolve/issue/test this feature\.',
                     '**Assignment**: Data-Oriented Design (DOD) & ECS Refactoring needs to be resolved/issued/tested by the Architecture Lead', content)
    # Task 3
    content = re.sub(r'\*\*Assignment\*\*: Systems Engineer needs to resolve/issue/test this feature\.',
                     '**Assignment**: Multithreading Implementation needs to be resolved/issued/tested by the Systems Engineer', content, count=1)
    # Task 4
    content = re.sub(r'\*\*Assignment\*\*: Systems Engineer needs to resolve/issue/test this feature\.',
                     '**Assignment**: SIMD Vectorization Implementation needs to be resolved/issued/tested by the Systems Engineer', content, count=1)
    # Task 5
    content = re.sub(r'\*\*Assignment\*\*: Systems Engineer needs to resolve/issue/test this feature\.',
                     '**Assignment**: Cross-Platform Determinism Setup needs to be resolved/issued/tested by the Systems Engineer', content)
    # Task 6
    content = re.sub(r'\*\*Assignment\*\*: Graphics Engineer needs to resolve/issue/test this feature\.',
                     '**Assignment**: GPU Acceleration (Compute Shaders) Integration needs to be resolved/issued/tested by the Graphics Engineer', content)

    with open(filename, 'w') as f:
        f.write(content)
