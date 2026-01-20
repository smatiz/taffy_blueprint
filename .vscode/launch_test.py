
import json5
import os
import sys
import task_lib 

root_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
print(root_path)
# Load the JSON file
with open(os.path.join(root_path, "taffy.code-workspace"), "r", encoding="utf-8") as f:
    data = json5.load(f)

# Iterate over folders
for entry in data.get("folders", []):
    folder = entry.get("path")
    if not folder:
        continue

    full_path = os.path.join(root_path, folder)

    print(f"\n=============================")
    print(f"{folder}")
    print(f"-------------------------------")


    task_lib.run_cargo("test", full_path, 
                  use_mod=True, no_warning = True, 
                  backtrace = False, target_dir ="target/_test", 
                  release=False, features=["use_json", "use_macroquad"] )

