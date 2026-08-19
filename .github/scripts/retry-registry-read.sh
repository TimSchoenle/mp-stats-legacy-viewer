#!/usr/bin/env bash
# Retry a registry read that failed for a reason having nothing to do with the image.
#
#   retry-registry-read.sh <command> [args...]
#
# Docker Hub serves manifests and config blobs from a CDN that resets connections under load, and a
# reset reaches the caller as a plain non-zero exit. That is what failed the v0.17.1 release: one
# second into the step that verifies the contract labels, `oras manifest fetch-config` died with
# `read: connection reset by peer` on an image that had been pushed correctly, and the release
# stopped before the contract was ever attached.
#
# Reads only. Everything this may wrap fetches an immutable object that is already in the registry,
# so repeating it either returns the same bytes or fails the same way - there is no half-created
# state a second attempt could compound. Do not wrap `oras attach` with it: an attach that failed
# after storing its manifest would, on a retry, leave a second referrer of the same artifact type
# on the digest, which is exactly what the consuming chart repository refuses to pick between.
#
# Each attempt's stdout is collected and released only once one succeeds. A reset mid-response
# leaves a truncated document behind, and streaming that straight through would hand the caller's
# `jq` the failed attempt's fragment glued to the next attempt's full answer. stderr is not
# collected, so the underlying tool's own diagnosis stays in the log as it happens.
set -euo pipefail

attempts=5
backoff=5 # seconds, multiplied by the attempt number: 5, 10, 15, 20 - 50s of waiting at worst

[ "$#" -gt 0 ] || {
  echo "usage: retry-registry-read.sh <command> [args...]" >&2
  exit 2
}

output="$(mktemp)"
trap 'rm -f "${output}"' EXIT

attempt=1
while :; do
  # Not `if "$@"; then`: after such a block `$?` is the `if`'s own status, which is 0 when no
  # branch ran, and a retry loop that reports 0 for a command that never succeeded exits 0.
  status=0
  "$@" > "${output}" || status=$?

  if [ "${status}" -eq 0 ]; then
    cat "${output}"
    exit 0
  fi

  if [ "${attempt}" -ge "${attempts}" ]; then
    echo "error: \`$*\` exited ${status} on all ${attempts} attempts, so this is not a transient" \
         "registry error" >&2
    exit "${status}"
  fi

  delay=$(( attempt * backoff ))
  echo "warning: \`$*\` exited ${status} (attempt ${attempt}/${attempts}); retrying in ${delay}s" >&2
  sleep "${delay}"
  attempt=$(( attempt + 1 ))
done
