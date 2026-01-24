#!/usr/bin/env bash
set -euo pipefail

format="${1:-png}"
input_dir="${2:-docs/diagrams}"
output_dir="${3:-docs/diagrams/rendered}"

if ! command -v mmdc >/dev/null 2>&1; then
  echo "error: mmdc not found. Install with: npm install -g @mermaid-js/mermaid-cli" >&2
  exit 1
fi

mkdir -p "${output_dir}"

shopt -s nullglob
files=("${input_dir}"/*.mmd)
if [ ${#files[@]} -eq 0 ]; then
  echo "no .mmd files found in ${input_dir}" >&2
  exit 1
fi

for file in "${files[@]}"; do
  base="$(basename "${file}" .mmd)"
  mmdc -i "${file}" -o "${output_dir}/${base}.${format}"
done

echo "rendered ${#files[@]} diagram(s) to ${output_dir}"
