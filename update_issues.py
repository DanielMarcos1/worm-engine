import re
import os

files_to_update = [
    ("ISSUE_TASK_1_CCD.md", "Continuous Collision Detection (CCD) Implementation", "Physics Engineer"),
    ("ISSUE_TASK_2_DOD.md", "Data-Oriented Design (DOD) & ECS Refactoring", "Architecture Lead"),
    ("ISSUE_TASK_3_SIMD.md", "Multithreading and SIMD Vectorization", "Systems Engineer"),
    ("ISSUE_TASK_4_DETERMINISM.md", "Cross-Platform Determinism Setup", "Systems Engineer"),
    ("ISSUE_TASK_5_GPU.md", "GPU Acceleration (Compute Shaders) Integration", "Graphics Engineer"),
]

for filename, task, role in files_to_update:
    if not os.path.exists(filename):
        continue
    with open(filename, 'r') as f:
        content = f.read()

    # Replace assignment
    # Currently it's like: **Role** needs to resolve/issue/test this feature.
    # We want: Task needs to be resolved/issued/tested by the Role

    # Let's just replace the whole section text.
    new_text = f"{task} needs to be resolved/issued/tested by the {role}"
    content = re.sub(r'## Assigned Agency Role\n.*', f'## Assigned Agency Role\n{new_text}', content)

    # For ISSUE_TASK_3_SIMD.md, fix std::simd -> wide
    if filename == "ISSUE_TASK_3_SIMD.md":
        content = content.replace("std::simd", "wide crate")

    with open(filename, 'w') as f:
        f.write(content)
