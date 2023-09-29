
all:
	rm -f output/*
	rm -f output_png/*
	cargo run --release
	just convert
	just video

convert:
	./convert.sh 

video:
	rm -f output.mp4
	ffmpeg -framerate 20 -i output_png/frame_%06d.ppm.png -deadline realtime -threads 13 output.mp4

make_loop:
	rm -f output_proper.mp4
	ffmpeg -stream_loop 3 -i output.mp4 -c copy output_proper.mp4

clean:
	cargo claen
	rm -f output/*
	rm -f output_png/*
