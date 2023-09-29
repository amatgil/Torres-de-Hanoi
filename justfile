
all:
	cargo run --release
	just convert
	just video
	just make_loop

convert:
	./convert.sh 

video:
	rm -f output.mp4
	ffmpeg -framerate 20 -i output_png/frame_%06d.ppm.png  -vf format=yuv420p output.mp4

make_loop:
	rm -f output_proper.mp4
	ffmpeg -stream_loop 3 -i output.mp4 -c copy output_proper.mp4
