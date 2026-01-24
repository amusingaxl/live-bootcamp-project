#!/usr/bin/env bash
set -euo pipefail

input_dir="${1:-docs/diagrams}"
output_dir="${2:-docs/diagrams/rendered}"
format="${3:-png}"

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg (ripgrep) not found; install it or change the script" >&2
  exit 1
fi

render_all() {
  ./scripts/render-mermaid.sh "${format}" "${input_dir}" "${output_dir}"
}

render_all
echo "watching ${input_dir} for .mmd changes..."

while true; do
  rg --files "${input_dir}" -g '*.mmd' \
    | while IFS= read -r file; do
        printf '%s %s\n' "$(stat -c '%Y' "${file}")" "${file}"
      done \
    | sort -n \
    | awk '{print $2":"$1}' \
    | md5sum > /tmp/mermaid_watch.sum

  sleep 1

  rg --files "${input_dir}" -g '*.mmd' \
    | while IFS= read -r file; do
        printf '%s %s\n' "$(stat -c '%Y' "${file}")" "${file}"
      done \
    | sort -n \
    | awk '{print $2":"$1}' \
    | md5sum > /tmp/mermaid_watch.sum.new

  if ! cmp -s /tmp/mermaid_watch.sum /tmp/mermaid_watch.sum.new; then
    render_all
  fi
done
