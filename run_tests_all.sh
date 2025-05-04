#!/bin/bash

set -eu

readarray -t dirs < <(find ./problems -mindepth 1 -maxdepth 1 -type d)

for d in "${dirs[@]}"; do
	echo "run tests in $d"
	pushd "$d"
	cargo test --verbose
	popd
done
