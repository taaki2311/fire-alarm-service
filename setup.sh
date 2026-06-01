#!/usr/bin/sh
set -e
cd "$(dirname "$0")"

echo "Creating incidents and timestamp files"
cp incidents.xmpl incidents.json
cp timestamp.xmpl timestamp.txt
echo "Successfully created incidents and timestamp files"
