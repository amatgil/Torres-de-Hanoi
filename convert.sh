#!/bin/env bash

echo "removing"
rm output_png/*
echo "removed"

for filename in ./output/*; do
	SURT="$(basename ${filename}).png"
	echo "Convertint ${filename}"
	convert ${filename} -interpolate Integer -filter point -scale 1920x1080 "output_png/$SURT";
done
