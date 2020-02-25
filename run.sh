#!/bin/bash

set -e

build="$1"

if [ "$build" = "" ]; then
	echo "Usage: $0 <build_name>"
	exit 0
fi

cargo b --release

outdir="target/bench/$build"
echo "Callgrind output dir: $outdir"
mkdir -p "$outdir"

for input in test-data/*.html; do
	output="$outdir/$(basename $input).callgrind"
	(
		set -x
		valgrind --tool=callgrind --toggle-collect=parse_html --callgrind-out-file="$output" target/release/html5ever-bench "$input"
	)
done
