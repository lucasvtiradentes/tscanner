#!/bin/bash

INPUT=$(cat)

ISSUES="[]"

add_issue() {
  local file="$1"
  local line="$2"
  local message="$3"
  ISSUES=$(echo "$ISSUES" | jq --arg f "$file" --argjson l "$line" --arg m "$message" \
    '. + [{"file": $f, "line": $l, "message": $m}]')
}

while read -r file; do
  path=$(echo "$file" | jq -r '.path')
  content=$(echo "$file" | jq -r '.content')

  if ! echo "$content" | jq -e '.name' > /dev/null 2>&1; then
    add_issue "$path" 1 "Missing 'name' field in package.json"
  fi

  if ! echo "$content" | jq -e '.version' > /dev/null 2>&1; then
    add_issue "$path" 1 "Missing 'version' field in package.json"
  fi

  if ! echo "$content" | jq -e '.license' > /dev/null 2>&1; then
    add_issue "$path" 1 "Missing 'license' field in package.json"
  fi

  if ! echo "$content" | jq -e '.description' > /dev/null 2>&1; then
    add_issue "$path" 1 "Missing 'description' field - helps users understand the package"
  fi

  if echo "$content" | jq -e '.dependencies["lodash"]' > /dev/null 2>&1; then
    line=$(echo "$content" | grep -n '"lodash"' | head -1 | cut -d: -f1)
    add_issue "$path" "${line:-1}" "Consider using lodash-es or individual lodash functions for better tree-shaking"
  fi

  if echo "$content" | jq -e '.dependencies["moment"]' > /dev/null 2>&1; then
    line=$(echo "$content" | grep -n '"moment"' | head -1 | cut -d: -f1)
    add_issue "$path" "${line:-1}" "Consider using date-fns or dayjs instead of moment (smaller bundle)"
  fi
done < <(echo "$INPUT" | jq -c '.files[]')

echo "{\"issues\": $ISSUES}"
