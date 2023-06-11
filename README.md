# Impro
<<<<<<< HEAD
## [Archived]
=======
>>>>>>> 39c1da5fd4f4918aa19d8c7a28801a00c6cfc98e
Image Processor and Graph tool.

This repo contains rust code written for learning purposss for most common image processing tasks.

## Benchmarks:
Note: "against" reading below refers to the time taken by image crate to do the same operation.

Note: There *might* be a more efficient way in which the crate saves image as file that saves time for it.

<img alt="benchmark pic" width="200px" height="200px" src="https://raw.githubusercontent.com/skndash96/impro/main/benchmark.png">

Results are present in the `tmp` directory. Images prefixed `ops` are results of the image crate.

## Functions

- Crop
    ```
    crop(img, dim, offset, new_dim)
    ```
- brighten
    ```
    brighten(img, power)
    ```
- Convolution + Pixelate
    ```
    conv(img, dim, kernel, minimize)
    ```
- Overlay two images
    ```
    overlay(img, img2, dim, dim2, offset)
    ```

WHERE
- img: `Vec<u8>`
- kernel: `Vec<i32>`
- dim, new_dim: `(u32, u32)`
- offset: `(i32, i32)`
- minimize: `bool` (whether to reduce image size)