
all:
	rm -f output/*
	rm -f output_png/*
	cargo run --release
	#just convert
	just video

convert:
	./convert.sh 

video:
	rm -f output.mp4
	ffmpeg -framerate 60 -i output/frame_%06d.ppm -deadline realtime -threads 13 output.mp4

make_loop:
	rm -f output_proper.mp4
	ffmpeg -stream_loop 3 -i output.mp4 -c h264_qsv output_looped.mp4

clean:
	cargo claen
	rm -f output/*
