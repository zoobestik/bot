#!/usr/bin/env bash

set -euo pipefail

BOT_VERSION=$(grep -m1 "^version = " Cargo.toml | cut -d'"' -f2)

docker build -t "zoobestik/gamebot:$BOT_VERSION" .
