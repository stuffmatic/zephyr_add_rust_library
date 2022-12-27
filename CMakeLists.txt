# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(zephyr_add_rust_library)

target_sources(app PRIVATE src/main.c)

# Import the zephyr_add_rust_library function
include(zephyr_add_rust_library.cmake)

# Add rust-lib-1 dependency. This crate depends on std and does not
# build the C API by default. Pass extra cargo args to disable
# default features (std) and enable c_api.
zephyr_add_rust_library(
  CRATE_NAME rust-lib-1 # name with dashes
  CRATE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/rust_lib_1
  CRATE_HEADER_DIR ${CMAKE_CURRENT_SOURCE_DIR}/rust_lib_1/include
  EXTRA_CARGO_ARGS --no-default-features --features c_api
)

# Add rust_lib_2 dependency
zephyr_add_rust_library(
  CRATE_NAME rust_lib_2 # name with underscores
  CRATE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/rust_lib_2
  CRATE_HEADER_DIR ${CMAKE_CURRENT_SOURCE_DIR}/rust_lib_2/include
)