#!/usr/bin/env sh
set -e

TIMESTAMP=/home/state/timestamp.txt
if [ ! -f "$TIMESTAMP" ]; then
    echo "$TIMESTAMP not found"
    date --iso-8601=seconds --utc | tr --delete '\n' > "$TIMESTAMP"
fi

echo "$TIMESTAMP contents:"
cat $TIMESTAMP

exec sleep infinity
