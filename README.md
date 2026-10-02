
# ASCII Video Player

A simple terminal-based video player that converts video frames into ASCII characters and plays them directly in the terminal.

## Features

* Plays video directly in the terminal using ASCII characters.
* Adjustable brightness mapping.
* Optional brightness bias for controlling contrast.
* Optional brightness inversion.
* Custom input, video, and output directories.
* Optional file overwriting.
* Configuration path command.

## Download

### Windows

Prebuilt releases are available as ZIP archives on the project's **Releases** page.

Download the ZIP, extract it, and run the executable.

The release ZIP already contains the required OpenCV DLL:

```text
opencv_world4130.dll
```

The DLL **must remain in the same directory as the executable**.

### Linux

Currently No direct downloadable. See Build section

## Usage

```bash
ascii-video <VIDEO>
```

Example:

```bash
ascii-video video.mp4
```

The video will be decoded frame-by-frame and rendered as ASCII in the terminal.

### Input Flag

The input video can also be provided using `-i` or `--input`:

```bash
ascii-video -i video.mp4
```

Both forms are equivalent:

```bash
ascii-video video.mp4
ascii-video --input video.mp4
```

## Options

| Option                   | Description                                   |
| ------------------------ | --------------------------------------------- |
| `<VIDEO>`                | Input video file                              |
| `-i, --input <VIDEO>`    | Input video file                              |
| `-v, --video-dir <DIR>`  | Directory containing video files              |
| `-o, --output-dir <DIR>` | Output directory                              |
| `-b, --bias <BIAS>`      | Adjust brightness threshold distribution      |
| `-f, --flip`             | Invert the brightness mapping                 |
| `-r, --overwrite`        | Allow existing output files to be overwritten |
| `-h, --help`             | Show help                                     |
| `-V, --version`          | Show version                                  |

## Brightness Bias

The `--bias` option changes how brightness levels are distributed across the ASCII characters.

A value of `1.0` produces a linear distribution.

```bash
ascii-video video.mp4 --bias 1.0
```

Values below `1.0` move the thresholds toward the brighter end, preserving more detail in darker areas.

```bash
ascii-video video.mp4 --bias 0.7
```

Values above `1.0` move the thresholds toward the darker end.

```bash
ascii-video video.mp4 --bias 1.5
```

A reasonable range to experiment with is approximately `0.5`–`2.0`.

## Flip

`--flip` reverses the brightness-to-character mapping.

```bash
ascii-video video.mp4 --flip
```

## Configuration

To see the configuration file path:

```bash
ascii-video config --show-path
```

## Examples

```bash
# Basic playback
ascii-video video.mp4

# Preserve more detail in dark areas
ascii-video video.mp4 --bias 0.7

# Invert brightness
ascii-video video.mp4 --flip

# Combine options
ascii-video video.mp4 --bias 0.7 --flip

# Allow overwriting existing files
ascii-video video.mp4 --overwrite
```

## Building

### Prerequisites
OpenCv 4130.

Clone the repository and build with Cargo:

```bash
git clone <repository-url>
cd <repository-directory>
cargo build --release
```

For a **manual build**, the OpenCV runtime DLL is required:

```text
opencv_world4130.dll
```

Place it in the **same directory as the compiled executable**:

```text
target/
└── release/
    ├── ascii-video.exe
    └── opencv_world4130.dll
```

Unlike the prebuilt release ZIP, the DLL is **not automatically included** when running a manual Cargo build.
