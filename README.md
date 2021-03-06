## Image Segmenter in Rust

Segment the image by color, applying a unique color to each segment.

## UPDATE 25/04/2022

It looks like most of the functionality can be replaced with [opencv::imgproc::flood_fill](https://docs.rs/opencv/0.19.2/opencv/imgproc/fn.flood_fill.html).

## Build

`cargo build --release`

## Run

`image_segmenter.exe -i input.png -o output.png`

## Examples

### Example 1

![colors](https://user-images.githubusercontent.com/1350889/164956248-60e306d6-bf6b-4eb2-a65a-5239b1923546.png)

![out](https://user-images.githubusercontent.com/1350889/164956263-22fece69-f1c4-4732-a5cd-19a8859ac319.png)

### Example 2

![colors2](https://user-images.githubusercontent.com/1350889/164956251-4319dc75-195d-4c42-a506-bd383257fc44.png)

![out](https://user-images.githubusercontent.com/1350889/164956277-5b49c5ec-c50c-4324-a8e5-85342c65e3a2.png)
