#!/bin/env bash

echo "removing"
rm output_png/*
echo "removed"

for filename in ./output/*; do
	SURT="$(basename ${filename}).png"
	echo "\rConvertint ${filename}"
	convert ${filename} -interpolate Integer -filter point -resize 400% "output_png/$SURT";
done
