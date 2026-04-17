#!/usr/bin/env bash
# scripts/seed-issues.sh — create GitHub issues for every planned (⬜) type in ROADMAP.md
#
# Usage:
#   ./scripts/seed-issues.sh <owner/repo>
#   ./scripts/seed-issues.sh codegress-com/arvo
#
# Requirements: gh CLI authenticated with repo scope.
# Safe to re-run — skips issues that already exist.

set -euo pipefail

REPO="${1:-}"
if [[ -z "$REPO" ]]; then
  echo "Usage: $0 <owner/repo>" >&2
  exit 1
fi

ROADMAP="$(cd "$(dirname "$0")/.." && pwd)/ROADMAP.md"

if [[ ! -f "$ROADMAP" ]]; then
  echo "ROADMAP.md not found at $ROADMAP" >&2
  exit 1
fi

created=0
skipped=0
current_feature=""

while IFS= read -r line; do

  # Track current feature section: ## `contact` feature
  if [[ "$line" =~ ^##[[:space:]]+\`([a-z_-]+)\`[[:space:]]+feature ]]; then
    current_feature="${BASH_REMATCH[1]}"
    continue
  fi

  # Match planned rows: | `TypeName` | ⬜ | notes |
  if [[ -z "$current_feature" ]]; then continue; fi
  if [[ ! "$line" =~ ^\|[[:space:]]*\`([A-Za-z0-9]+)\`[[:space:]]*\|[[:space:]]*⬜ ]]; then continue; fi

  type_name="${BASH_REMATCH[1]}"

  # Extract notes (third column)
  notes=$(echo "$line" | awk -F'|' '{gsub(/^[[:space:]]+|[[:space:]]+$/, "", $4); print $4}')

  title="feat($current_feature): $type_name"

  # Skip if issue already exists (exact title match)
  existing=$(gh issue list \
    --repo "$REPO" \
    --state all \
    --search "in:title \"$title\"" \
    --json title \
    --jq '.[].title' 2>/dev/null || true)

  if echo "$existing" | grep -qF "$title"; then
    echo "  skip  $title"
    ((skipped++)) || true
    continue
  fi

  body="## Description

Implement \`$type_name\` as a \`ValueObject\` in the \`$current_feature\` module.

**Spec:** $notes

## Implementation checklist

- [ ] Create \`src/$current_feature/${type_name,,}.rs\`
- [ ] Implement \`ValueObject\` trait
- [ ] Add \`#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]\`
- [ ] Export from \`src/$current_feature/mod.rs\` and \`prelude\`
- [ ] Unit tests: valid input · empty input · invalid format · normalisation
- [ ] Doc comment with \`# Example\` block
- [ ] Update status in \`ROADMAP.md\` from ⬜ to ✅

## References

- [ROADMAP.md](https://github.com/$REPO/blob/main/ROADMAP.md)
- [CONTRIBUTING.md](https://github.com/$REPO/blob/main/CONTRIBUTING.md)"

  gh issue create \
    --repo "$REPO" \
    --title "$title" \
    --body "$body" \
    --label "enhancement" \
    --label "good-first-issue"

  echo "  created $title"
  ((created++)) || true

done < "$ROADMAP"

echo ""
echo "Done — created: $created  skipped (already exist): $skipped"
