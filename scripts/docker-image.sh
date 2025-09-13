#!/usr/bin/env bash

set -euo pipefail

BOT_VERSION=$(grep -m1 "^version = " Cargo.toml | cut -d'"' -f2)
# BOT_VERSION=$(cargo metadata --no-deps --format-version=1 | jq -r .packages[0].version)

docker build --platform linux/amd64,linux/arm64 -t "zoobestik/gamebot:$BOT_VERSION" .
