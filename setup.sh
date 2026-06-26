#!/usr/bin/env sh
set -e
cd "$(dirname "$0")"

echo "Creating incidents and timestamp files"
cp incidents.xmpl incidents.json
date --iso-8601=seconds --utc | tr --delete '\n' > timestamp.txt
echo "Successfully created incidents and timestamp files"
