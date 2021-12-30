# rename-photos

## About

`rename-photos` is a small application that takes all the `*.jpg`s in the current working
directory and moves them into a chronologically sorted directory tree, renaming the files
along the way.

In other words, if you had the following files in a directory:

```
DCIM_0123.jpg
DCIM_0124.jpg
DCIM_0125.jpg
DCIM_0126.jpg
DCIM_0127.jpg
```

...and they were all taken on the 20th November 2021, after running `rename-photos` (there are no arguments
or configuration), those files will end up with in a new directory tree composed
of the year, month number and name: `2021/11_November/` with following files:

```
2021_11_20_13_25_22.jpg
2021_11_20_13_25_26.jpg
2021_11_20_13_25_28.jpg
2021_11_20_13_25_31.jpg
2021_11_20_13_26_47.jpg
```

It's as simple as that.

__I wrote this to replace a shell script I had used previously and to practise some Rust. But essentially
this fits a single use case that I use all the time to organise photos locally.__

## Installation

```
cargo install --git https://github.com/roryrjb/rename-photos
```

Tested on Linux and Windows.