# mandelbrot

An algorithm that draws an image of the mandelbrot set. 

Inspired by [Coding Train](https://www.youtube.com/watch?v=6z7GQewK-Ks).

More information on mandelbrot sets: [Wikipedia](https://en.wikipedia.org/wiki/Mandelbrot_set)

## How to use this tool

* Clone this repository with `git clone git@github.com/jakobdannel/mandelbrot.git`
* Install rust (https://www.rust-lang.org/learn/get-started)
* Build project with `cargo build --release`
* Execute file `/targets/release/mandelbrot`
    - Flag usage: add `-w {} -h {}`, replace `{}` with the desired width and height, `-f {}` to set the amount of frames, and `-c` to make the image colorful 
* The output is generated as a .png file under `/output`

## Example images (5000x5000)

![Example image](/examples/example1.png)
![Example image](/examples/example2.png)
![Example image](/examples/example3.png)
![Example image](/examples/example4.png)