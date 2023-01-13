# Impro
Image Editor, the coolest of all.

This repo contains rust code written for learning purposss for most common image processing tasks.

## Benchmarks:
Note: "against" reading below refers to the time taken by image crate to do the same operation.

Note: There *might* be a more efficient way in which the crate saves image as file that saves time for it.

!(benchmark pic)[https://github.com/skndash96/impro/benchmark.png]

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
    overlay(img, img2, dim, dim2, offset, alpha)
    ```

WHERE
- img: `Vec<u8>`
- kernel: `Vec<i32>`
- dim, new_dim: `(u32, u32)`
- offset: `(i32, i32)`
- alpha: `u8`
- minimize: `bool` (whether to reduce image size)