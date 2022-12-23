# zephyr\_add\_rust\_library

This repo contains

* the CMake function `zephyr_add_rust_library` used for adding Rust library crate dependencies to [Zephyr](https://zephyrproject.org/) apps. This funcition is defined in [`zephyr_add_rust_library.cmake`](zephyr_add_rust_library.cmake).
* a minimal Zephyr test app

Using `zephyr_add_rust_library` has the following benefits:

* crates are automatically compiled to static libraries as part of the Zephyr build process
* adding and removing Rust dependencies is easy
* debugging works seamlessly
* Rust build artefacts are output to the Zephyr build folder

⚠️ __The code in this repo should be considered work in progress.__ If you run into any issues, please consider [reporting them](https://github.com/stuffmatic/zephyr_add_rust_library/issues) or [submitting a PR](https://github.com/stuffmatic/zephyr_add_rust_library/pulls)!

## Prerequisites

The reader is assumed to be familiar with the basics of [`no_std` Rust](https://docs.rust-embedded.org/book/intro/no-std.html) and [how to call Rust code from C](https://docs.rust-embedded.org/book/interoperability/rust-with-c.html).

## Usage

First, make sure you have [installed Rust](https://www.rust-lang.org/tools/install). Then put [`zephyr_add_rust_library.cmake`](zephyr_add_rust_library.cmake) in the same directory as your `CMakeLists.txt`. To add a dependency on a library crate named `my-lib-crate`, add these lines to your `CMakeLists.txt`:

```
# Import the zephyr_add_rust_library function
include(zephyr_add_rust_library.cmake)

# Add my-lib-crate crate dependency.
zephyr_add_rust_library(
  CRATE_NAME my-lib-crate # Crate name as specified in Cargo.toml
  CRATE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/my-lib-crate # Crate root dir
  CRATE_HEADER_PATH ${CMAKE_CURRENT_SOURCE_DIR}/my-lib-crate/include # C header dir
)
```

`${CMAKE_CURRENT_SOURCE_DIR}` is the directory containing your `CMakeLists.txt`. For additional arguments and more detailed documentation, see [`zephyr_add_rust_library.cmake`](zephyr_add_rust_library.cmake).

Errors along the lines of `Could not find specification for target ...` indicate that you need to add a Rust target corresponding to the architechure you're building for. To do this, run `rustup target add [target]`, where `[target]` is the name of the target in the error message (a complete list of targets can be found [here](https://doc.rust-lang.org/nightly/rustc/platform-support.html)).


## Known issues and future work

`zephyr_add_rust_library.cmake` should be considered work in progress and has so far been tested on macOS using the ninja generator to build for nRF52/nRF53 SOCs with Cortex M4/M33 architectures. Known issues include:

* __Limited cargo target selection__ - `zephyr_add_rust_library` uses KConfig variables like `CONFIG_FPU`, `CONFIG_CPU_CORTEX_M4` etc to set a suitable `cargo` target. This mapping is not complete and only supports some Cortex M architectures at the time of writing.
* __Ignored duplicate symbols__ - Library crate dependencies are linked using `--allow-multiple-definition` to silence errors when when linking multiple libraries containing the same compiler builtins. Unfortunately, this means that legitimate multiple definition errors will also go unnoticed and it's up to the user to detect these.
