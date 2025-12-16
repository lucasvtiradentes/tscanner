#!/usr/bin/env python3

import json
import sys

MIN_LINES = 3

def main():
    data = sys.stdin.read()
    input_data = json.loads(data)
    issues = []

    for file in input_data["files"]:
        line_count = len(file["lines"])

        if line_count < MIN_LINES:
            issues.append({
                "file": file["path"],
                "line": 1,
                "message": f"File has only {line_count} lines, minimum is {MIN_LINES} lines"
            })

    print(json.dumps({"issues": issues}))

if __name__ == "__main__":
    main()
