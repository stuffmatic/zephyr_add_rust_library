# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(zephyr_add_rust_library)

target_sources(app PRIVATE src/main.c)

# Import the zephyr_add_rust_library function
include(zephyr_add_rust_library.cmake)

# Add rust-lib-1 dependency.
zephyr_add_rust_library(
  rust-lib-1 # name with dashes
  ${CMAKE_CURRENT_SOURCE_DIR}/rust_lib_1
  ${CMAKE_CURRENT_SOURCE_DIR}/rust_lib_1/include
)

# Add rust_lib_2 dependency.
zephyr_add_rust_library(
  rust_lib_2 # name with underscores
  ${CMAKE_CURRENT_SOURCE_DIR}/rust_lib_2
  ${CMAKE_CURRENT_SOURCE_DIR}/rust_lib_2/include
)