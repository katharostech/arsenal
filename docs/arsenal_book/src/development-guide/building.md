# Building Arsenal

> **Note:** Currently, building Arsenal involves manually compiling the separate components and copying them to the correct spot in the Blender plugin. This process will soon be automated using a makefile so that you don't have to manually structure the project during development.

## Prerequisites

### All OSes

For every OS you will need to have [Rust][rust] installed. The easiest way to do that is with [Rustup][rustup].

You also need Rust nightly and Rust 1.35 which can be installed with Rustup:

```bash
rustup toolchain install nightly
rustup toolchain install stable-2019-05-23
```

[rust]: https://rust-lang.org
[rustup]: https://rustup.rs/

### Windows and MacOS

On Windows and MacOS you need to install Python 3.7 which can be downloaded from [Python.org][py_download].

[py_download]: https://www.python.org/downloads/

### Linux

On Linux you will need to install the [Amethyst Dependencies][amethyst_deps] and `python3-dev`.

[amethyst_deps]: https://github.com/amethyst/amethyst#dependencies

## Building Arsenal Blender Core

The [Arsenal Blender Core][core] (Core) is in the [`arsenal_blender_core`][core_dir] directory in the git repository. The Core is a normal rust crate and can be built with Cargo.

```bash
cd arsenal_blender_core
cargo +nightly build
```

### Cargo Features

There is an optional feature for Core called `enable_profiling` that will dump a [flamegraph] to the current directory when executing the Blender export. This can be useful for investigation potential performance optimizations ( see [Systems Performance Work Guided By Flamegraphs][flamegraph_profiling] ).

```bash
cargo +nightly build --features enable_profiling
```

Once that finishes we can build the Arsenal Runtime.

[core]: ./architecture.md#arsenal-blender-core
[core_dir]: https://github.com/katharostech/arsenal/tree/master/arsenal_blender_core
[flamegraph]: https://github.com/TyOverby/flame
[flamegraph_profiling]: https://github.com/ferrous-systems/flamegraph#systems-performance-work-guided-by-flamegraphs

## Building Arsenal Runtime

The [Arsenal Runtime][arsenal_runtime] is in the [`arsenal_runtime`][runtime_dir] dir. The Arsenal Runtime is also built with Cargo:

```bash
cd arsenal_runtime
cargo +stable-2019-05-23 build
```

[arsenal_runtime]: ./architecture.md#arsenal-runtime
[runtime_dir]: https://github.com/katharostech/arsenal/tree/master/arsenal_runtime

## Packaging Arsenal Blender Plugin

After you have built Core and the Runtime, you have to copy the built assets to the [`arsenal_blender`][arsenal_blender] dir. On Linux and MacOS you can create symlinks to avoid having to re-copy the assets every time you rebuild them during development.

```bash
# Do this in the root of the repository. We assume you are using Git Bash on
# Windows.

# This is the same for all platforms
mkdir arsenal_blender/bin

# The library names are different for different platforms

# Windows
cp arsenal_runtime/target/debug/arsenal_runtime arsenal_blender/bin
cp arsenal_blender_core/target/debug/core.dll arsenal_blender/core.pyd

# Linux
ln -s ../../arsenal_runtime/target/debug/arsenal_runtime arsenal_blender/bin
ln -s ../arsenal_blender_core/target/debug/libcore.so arsenal_blender/core.so

# MacOS
ln -s ../../arsenal_runtime/target/debug/arsenal_runtime arsenal_blender/bin
ln -s ../arsenal_blender_core/target/debug/libcore.dylib arsenal_blender/core.so
```

After those are in place, you can copy, on Windows, or symlink, on Mac and Linux, the `arsenal_blender` dir to your blender Addons directory.

```bash
# Windows
cp arsenal_blender ${APPDATA}/Blender\ Foundation/Blender/2.80/scripts/addons

# Linux
ln -s /path/to/arsenal/arsenal_blender ~/.config/blender/2.8/scripts/addons

# MacOS
ln -s /path/to/arsenal/arsenal_blender /Users/yourusername/Library/Application\ Support/Blender/2.80/scripts/addons/
```

When that is done you just have to enable the plugin from the Blender interface and you are finished!

[arsenal_blender]: https://github.com/katharostech/arsenal/tree/master/arsenal_blender
