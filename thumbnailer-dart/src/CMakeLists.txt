# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(thumbnailer_library VERSION 0.0.1 LANGUAGES C)

add_library(thumbnailer SHARED
  "thumbnailer.c"
)

set_target_properties(thumbnailer PROPERTIES
  PUBLIC_HEADER thumbnailer.h
  OUTPUT_NAME "thumbnailer"
)

target_compile_definitions(thumbnailer PUBLIC DART_SHARED_LIB)
