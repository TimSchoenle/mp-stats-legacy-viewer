#!/usr/bin/env bash
# Check a built image's `dev.terrace.config.*` labels against the contract that was built with it.
#
# This is the checkpoint the whole scheme rests on. The three labels are constants, so the
# Dockerfile carries them as a hand-written `LABEL` block - and a hand-written block can be wrong
# in ways no source diff can see: a line deleted on a branch nobody diffed, a base image
# contributing its own, a build that produced a different path than the block claims. So the check
# is on the *image*, against `contract.labels` from the same generator run that produced the
# document the labels point at.
#
#   check-image-labels.sh <labels.json> <contract.labels> [<what>]
#
# `labels.json` is the label map as a container tool reports it. Two spellings, and reading the
# wrong one yields `null` - which a careless comparison treats as "nothing to compare" and passes:
#
#   docker inspect --format '{{json .Config.Labels}}' "$image"                 # .Config.Labels
#   docker buildx imagetools inspect --format '{{json .Image}}' "$image"       # .config.Labels,
#                                                                             # keyed by platform
#
# Extra labels are ignored on purpose. Every image carries `org.opencontainers.image.*` and
# whatever its base contributed, and none of that is this document's business - so this mirrors
# `Contract::verify_labels` exactly: presence and equality of the three, nothing more.
#
# Every violation is reported before it exits. A build that names one missing label and hides two
# is a second round trip.
set -euo pipefail

labels="${1:?usage: check-image-labels.sh <labels.json> <contract.labels> [<what>]}"
expected_file="${2:?usage: check-image-labels.sh <labels.json> <contract.labels> [<what>]}"
what="${3:-the image}"

if [ ! -s "${labels}" ]; then
  echo "error: ${what}: ${labels} is empty, so no label could be read. The inspect step that" \
       "writes it did not produce a label map." >&2
  exit 1
fi

if [ ! -s "${expected_file}" ]; then
  echo "error: ${expected_file} is empty, so there is nothing to check against. It comes from" \
       "\`--format labels\` in the same builder stage that wrote contract.json." >&2
  exit 1
fi

status=0
while IFS='=' read -r name expected; do
  [ -n "${name}" ] || continue
  actual="$(jq -r --arg n "${name}" '.[$n] // ""' "${labels}")"
  if [ "${actual}" != "${expected}" ]; then
    if [ -z "${actual}" ]; then
      echo "error: ${what} carries no '${name}', so nothing can discover this contract from its" \
           "config blob" >&2
    else
      echo "error: ${what}'s '${name}' is '${actual}', and this contract's is '${expected}'" >&2
    fi
    status=1
  fi
done < "${expected_file}"

if [ "${status}" -eq 0 ]; then
  echo "${what}: the three dev.terrace.config.* labels match the contract built with it"
fi

exit "${status}"
