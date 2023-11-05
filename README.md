# Tower of Hanoi, visualizer

The outputs generated from `n` in `[1, 12]` may be seen under [my website](https://amatgil.cat/altres/torres_hanoi). 
(Written in catalan, pending translation).

# Usage
Basic instructions are included in the `justfile`. The main command is `just all [n]`, which 
- creates output folders
- executes rust (which will ask for `n`), generating all `*.ppm` files
- use fmmpeg to join all `*.ppm`s into a single `output.mp4`

Do note that the `output.mp4` has been set to play at 60fps, but this may be too slow for high `n`.
It may be adjusted in the `justfile`:

The `justfile` also contains other utils 

## Exemple
With n = 8 (8 blocks, 255 moves, ~4 seconds at 60fps)

```bash
just all 8 
```
Output will be `output.mp4`.

# Internals
It's a classic of recursion, like quicksort: to move a series of `n` blocks, we move the first `n - 1`
blocks to the third pile, move the base to the destination pile, and move the `n - 1` blocks to the 
destination. Of course, if `n == 1`, we move it directly.

On each block move, the state is saved a `frame_[gen].ppm`

# Notes
- With small `n`s, the initial and final delays become noticeable. This is too irrelevant to bother fixing
- With small `n`s, the frames are quite small so, when lossily upscaling, the video may seem blurry at fullscreen resolutions.
This is intended, as the video is meant to be embedded in small-screen formats.
