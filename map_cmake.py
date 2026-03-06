import os
import re
import json
import csv

# ==== CONFIG ====
PROJECT_ROOT = "."  # Change this to your project root path
OUTPUT_JSON = "cmake_dependency_map.json"
OUTPUT_CSV = "cmake_dependency_map.csv"
OUTPUT_DOT = "cmake_dependency_graph.dot"

# ==== UTILS ====
def find_cmake_files(root_path):
    cmake_files = []
    for dirpath, _, filenames in os.walk(root_path):
        for f in filenames:
            if f == "CMakeLists.txt":
                cmake_files.append(os.path.join(dirpath, f))
    return cmake_files

def parse_cmake_file(file_path):
    targets = []
    dependencies = {}
    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()

        # Match add_library or add_executable
        for match in re.findall(r'add_(library|executable)\s*\(\s*([^\s\)]+)', content):
            target_name = match[1]
            targets.append(target_name)
            dependencies[target_name] = []

        # Match target_link_libraries
        for match in re.findall(r'target_link_libraries\s*\(\s*([^\s\)]+)\s+([^\)]+)\)', content):
            target = match[0]
            deps = re.split(r'\s+', match[1].strip())
            if target not in dependencies:
                dependencies[target] = []
            dependencies[target].extend([d for d in deps if d != "PUBLIC" and d != "PRIVATE" and d != "INTERFACE"])
    
    return targets, dependencies

# ==== MAIN ====
cmake_files = find_cmake_files(PROJECT_ROOT)
project_map = {}

for file_path in cmake_files:
    targets, deps = parse_cmake_file(file_path)
    if targets:
        project_map[file_path] = {"targets": targets, "dependencies": deps}

# ==== EXPORT JSON ====
with open(OUTPUT_JSON, "w", encoding="utf-8") as f:
    json.dump(project_map, f, indent=2)

# ==== EXPORT CSV ====
with open(OUTPUT_CSV, "w", newline="", encoding="utf-8") as f:
    writer = csv.writer(f)
    writer.writerow(["CMake File", "Target", "Dependencies"])
    for file_path, data in project_map.items():
        for target in data["targets"]:
            deps = ", ".join(data["dependencies"].get(target, []))
            writer.writerow([file_path, target, deps])

# ==== EXPORT DOT for Graphviz ====
with open(OUTPUT_DOT, "w", encoding="utf-8") as f:
    f.write("digraph cmake_graph {\n")
    for data in project_map.values():
        for target, deps in data["dependencies"].items():
            for dep in deps:
                f.write(f'  "{target}" -> "{dep}";\n')
    f.write("}\n")

print(f"Found {len(cmake_files)} CMakeLists.txt files.")
print(f"Generated: {OUTPUT_JSON}, {OUTPUT_CSV}, {OUTPUT_DOT}")
