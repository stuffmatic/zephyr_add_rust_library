`zephyr_add_rust_library` is an easy to use and self contained CMake function for adding Rust library crate dependencies to a [Zephyr](https://zephyrproject.org/) app. This repo contains a minimal Zephyr test app using this function, which is defined in [`zephyr_add_rust_library.cmake`](zephyr_add_rust_library.cmake).

With this workflow

* debugging works seamlessly
* build artefacts are output to the Zephyr build folder, which makes it easy to clean up build output
* the command to build a crate reduces to a no-op if the rust code hasn't changed (handled by `cargo`). 

⚠️ __The code in this repo should be considered [work in progress](#Improvements-and-future_work).__

## Usage

Put [`zephyr_add_rust_library.cmake`](zephyr_add_rust_library.cmake) in the same directory as your `CMakeLists.txt`. Then, to add a dependency on a library crate named `my-lib-crate`, add these lines to your `CMakeLists.txt`:

```
# Import the zephyr_add_rust_library function
include(zephyr_add_rust_library.cmake)

# Add my-lib-crate crate dependency.
zephyr_add_rust_library(
  my-lib-crate # Crate name as specified in Cargo.toml
  ${CMAKE_CURRENT_SOURCE_DIR}/my-lib-crate # Crate root dir
  ${CMAKE_CURRENT_SOURCE_DIR}/my-lib-crate/include # C header dir
)
```

`${CMAKE_CURRENT_SOURCE_DIR}` is the directory containing your `CMakeLists.txt`. For more details, see the documentation at the top of [`zephyr_add_rust_library.cmake`](zephyr_add_rust_library.cmake).

Errors along the lines of `Could not find specification for target ....` indicate that you need to add a Rust target for the architechure you're building for. To do this, run `rustup target add [target]`, where `[target]` one of the names [listed here](https://doc.rust-lang.org/nightly/rustc/platform-support.html).

If you're new to calling Rust code from C, you may find [this documentation](https://docs.rust-embedded.org/book/interoperability/rust-with-c.html) helpful.

## Improvements and future work

`zephyr_add_rust_library.cmake` should be considered work in progress and has so far been tested on MacOS using the ninja generator to build for nRF52/nRF53 SOCs with Cortex M4/M33 architectures. Known issues include:

* __Limited cargo target selection__ - `zephyr_add_rust_library` uses KConfig variables like `CONFIG_FPU`, `CONFIG_CPU_CORTEX_M4` etc to set a suitable `cargo` target. This mapping is by no means complete and only supports some Cortex M architectures at the time of writing.
* __Ignored duplicate symbols__ - Library crate dependencies are linked using `--allow-multiple-definition` to silence errors when when linking multiple rust libraries containing the same rust compiler builtins. Unfortunately, this means that other duplicate definitions will also go unnoticed and it's up to the user to detect these.

If you run into any issues, please consider [creating an issue](https://github.com/stuffmatic/zephyr_add_rust_library/issues) or [submitting a PR](https://github.com/stuffmatic/zephyr_add_rust_library/pulls)!