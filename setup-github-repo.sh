#!/usr/bin/env bash
# setup-github-repo.sh — configure a GitHub OSS Rust library repo via gh CLI
#
# Usage: ./setup-github-repo.sh <owner/repo> [crates-io-token]
#
# Requirements: gh CLI (https://cli.github.com) authenticated with repo scope.
#
# What it does:
#   1. Sets repo description, homepage, and topics
#   2. Configures repo settings (issues, discussions, wiki, security tab)
#   3. Creates standard labels
#   4. Protects the main branch
#   5. Creates the crates-io environment with CARGO_REGISTRY_TOKEN secret

set -euo pipefail

# ── Args ──────────────────────────────────────────────────────────────────────
REPO="${1:-}"
CARGO_TOKEN="${2:-}"

if [[ -z "$REPO" ]]; then
  echo "Usage: $0 <owner/repo> [cargo-registry-token]" >&2
  exit 1
fi

echo "Configuring $REPO …"

# ── 1. Repo metadata ──────────────────────────────────────────────────────────
DESCRIPTION=$(cargo metadata --no-deps --format-version 1 2>/dev/null \
  | python3 -c "import sys,json; d=json.load(sys.stdin); print(d['packages'][0]['description'])" \
  || echo "A Rust library")

gh repo edit "$REPO" \
  --description "$DESCRIPTION" \
  --homepage "https://docs.rs/${REPO##*/}" \
  --enable-issues \
  --enable-discussions \
  --delete-branch-on-merge \
  --no-enable-wiki

echo "  repo metadata set"

# Topics must be set via the API (gh repo edit --add-topic is available in newer gh versions)
CRATE_NAME="${REPO##*/}"
gh api "repos/$REPO/topics" \
  --method PUT \
  --field "names[]=rust" \
  --field "names[]=rust-library" \
  --field "names[]=$CRATE_NAME" \
  --silent

echo "  topics set"

# ── 2. Security / vulnerability reporting ─────────────────────────────────────
gh api "repos/$REPO" \
  --method PATCH \
  --field private_vulnerability_reporting_enabled=true \
  --silent 2>/dev/null || true   # not available on all plans

echo "  private vulnerability reporting enabled (if available)"

# ── 3. Labels ─────────────────────────────────────────────────────────────────
create_label() {
  local name="$1" color="$2" description="$3"
  gh label create "$name" --color "$color" --description "$description" --repo "$REPO" --force
}

# Remove noisy defaults
for label in "duplicate" "invalid" "wontfix" "help wanted" "good first issue"; do
  gh label delete "$label" --repo "$REPO" --yes 2>/dev/null || true
done

create_label "bug"              "d73a4a" "Something is broken"
create_label "enhancement"      "a2eeef" "New feature or improvement"
create_label "documentation"    "0075ca" "Documentation only"
create_label "question"         "d876e3" "Usage question — should go to Discussions"
create_label "triage"           "e4e669" "Needs maintainer triage"
create_label "security"         "b60205" "Security-related issue"
create_label "breaking-change"  "e11d48" "Requires a semver major bump"
create_label "good-first-issue" "7057ff" "Good for newcomers"
create_label "help-wanted"      "008672" "Extra attention is needed"
create_label "dependencies"     "0366d6" "Dependency update"
create_label "ci"               "f9d0c4" "CI/CD changes"
create_label "performance"      "fbca04" "Performance improvement"

echo "  labels created"

# ── 4. Branch protection ───────────────────────────────────────────────────────
# Requires the repo to have at least one commit on main.
gh api "repos/$REPO/branches/main/protection" \
  --method PUT \
  --header "Accept: application/vnd.github+json" \
  --field "required_status_checks[strict]=true" \
  --field "required_status_checks[contexts][]=CI" \
  --field "enforce_admins=false" \
  --field "required_pull_request_reviews[required_approving_review_count]=1" \
  --field "required_pull_request_reviews[dismiss_stale_reviews]=true" \
  --field "required_pull_request_reviews[require_code_owner_reviews]=false" \
  --field "restrictions=null" \
  --field "allow_force_pushes=false" \
  --field "allow_deletions=false" \
  --field "required_linear_history=false" \
  --silent

echo "  branch protection set on main"

# ── 5. crates-io environment + secret ─────────────────────────────────────────
gh api "repos/$REPO/environments/crates-io" \
  --method PUT \
  --field "wait_timer=0" \
  --silent

echo "  environment 'crates-io' created"

if [[ -n "$CARGO_TOKEN" ]]; then
  gh secret set CARGO_REGISTRY_TOKEN \
    --body "$CARGO_TOKEN" \
    --env crates-io \
    --repo "$REPO"
  echo "  CARGO_REGISTRY_TOKEN secret set"
else
  echo "  (no token provided — set CARGO_REGISTRY_TOKEN in the crates-io environment manually)"
fi

# ── Done ──────────────────────────────────────────────────────────────────────
echo ""
echo "Done! Next steps:"
echo "  • Verify: https://github.com/$REPO/settings"
echo "  • Check branch protection: https://github.com/$REPO/settings/branches"
echo "  • Check labels: https://github.com/$REPO/labels"
if [[ -z "$CARGO_TOKEN" ]]; then
  echo "  • Add CARGO_REGISTRY_TOKEN to the crates-io environment:"
  echo "    https://github.com/$REPO/settings/environments"
fi
