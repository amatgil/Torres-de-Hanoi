
web n:
	just all_webm {{n}}
	cp output.webm per_a_web/hanoi_render_{{n}}.webm

all_webm n:
	mkdir -p output/
	rm -f output/*
	cargo run --release -- {{n}}
	just video_webm

video_webm:
	rm -f output.webm
	ffmpeg -framerate 60 -i output/frame_%06d.ppm -deadline realtime -threads 13 output.webm

all n:
	mkdir -p output/
	rm -f output/*
	cargo run --release -- {{n}}
	just video

video:
	rm -f output.mp4
	ffmpeg -framerate 60 -i output/frame_%06d.ppm -deadline realtime -threads 13 output.mp4

make_loop:
	rm -f output_proper.mp4
	ffmpeg -stream_loop 3 -i output.mp4 -c h264_qsv output_looped.mp4

clean:
	cargo clean
	rm -f output/*
