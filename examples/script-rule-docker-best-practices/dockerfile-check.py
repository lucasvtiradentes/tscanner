#!/usr/bin/env python3

import json
import sys
import re

def add_issue(issues, file, line, message):
    issues.append({"file": file, "line": line, "message": message})

def main():
    data = json.load(sys.stdin)
    issues = []

    for f in data["files"]:
        path = f["path"]
        lines = f["lines"]
        content = f["content"]

        has_user_instruction = any("USER" in line and not line.strip().startswith("#") for line in lines)

        for i, line in enumerate(lines, 1):
            stripped = line.strip()

            if stripped.startswith("#"):
                continue

            if re.search(r"FROM\s+\S+:latest\b", line, re.IGNORECASE):
                add_issue(issues, path, i, "Avoid using ':latest' tag - use specific version for reproducibility")

            if re.search(r"FROM\s+\S+\s*$", line) and ":latest" not in line.lower() and ":" not in line:
                add_issue(issues, path, i, "No tag specified (defaults to ':latest') - use explicit version tag")

            if stripped.upper().startswith("RUN") and "apt-get install" in line and "-y" not in line:
                add_issue(issues, path, i, "Add '-y' flag to apt-get install for non-interactive installs")

            if stripped.upper().startswith("RUN") and "apt-get update" in line:
                if "apt-get install" not in line and "&& apt-get install" not in content[content.find(line):content.find(line)+500]:
                    add_issue(issues, path, i, "Combine 'apt-get update' with 'apt-get install' in same RUN to avoid cache issues")

            if re.match(r"^ADD\s+https?://", stripped, re.IGNORECASE):
                add_issue(issues, path, i, "Use 'curl' or 'wget' instead of ADD for remote URLs")

            if stripped.upper().startswith("COPY") and ".." in line:
                add_issue(issues, path, i, "Avoid copying from parent directories - restructure your build context")

        from_count = sum(1 for line in lines if line.strip().upper().startswith("FROM"))
        if from_count > 0 and not has_user_instruction:
            add_issue(issues, path, 1, "No USER instruction found - container will run as root")

    print(json.dumps({"issues": issues}))

if __name__ == "__main__":
    main()
