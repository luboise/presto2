cmake_minimum_required(VERSION 3.40)

set(CMAKE_EXPERIMENTAL_CXX_IMPORT_STD "d0edc3af-4c50-42ea-a356-e2862fe7a444")
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(Presto)

set(CMAKE_CONFIGURATION_TYPES
    "Debug;Release;Dist"
    CACHE STRING "Build Configurations" FORCE)

if(CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
  set(CMAKE_INSTALL_PREFIX
      "${CMAKE_BINARY_DIR}/install"
      CACHE PATH "Install path" FORCE)
endif()

option(PRESTO_BUILD_EDITOR "Enables the PrestoEditor target." OFF)
option(PRESTO_DEBUG_BUILD "Enables debug macros and modules." OFF)
option(PRESTO_ENABLE_TESTS "Enables testing." ON)

set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

# ##############################################################################
# ##############################################################################
# ##############################################################################

set(PRESTO_ENGINE_NAME "PrestoEngine")

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
  message(STATUS "Creating a debug build of Presto")
  set(PRESTO_DEBUG_BUILD ON)

  set(IMGUI_DIR ${CMAKE_CURRENT_SOURCE_DIR}/ext/imgui)

  add_compile_definitions(PR_DEBUG_BUILD)
endif()

find_program(
  GLSLC_PATH
  NAMES glslc
  HINTS Vulkan::glslc)

add_compile_definitions(SPDLOG_USE_STD_FORMAT)

if(NOT TARGET spdlog)
  # find_package(spdlog REQUIRED)
  add_subdirectory(ext/spdlog)
endif()

set_property(TARGET spdlog PROPERTY POSITION_INDEPENDENT_CODE ON)

set(JSON_BuildTests
    OFF
    CACHE INTERNAL "")
add_subdirectory(ext/json)

find_package(Vulkan REQUIRED)
include_directories(${Vulkan_INCLUDE_DIR})

# OpenGL Libraries
find_package(OpenGL REQUIRED)

# add_subdirectory(ext/GLEW/build/cmake)
find_package(GLEW REQUIRED)

add_subdirectory(ext/glm)

set(TINYGLTF_BUILD_LOADER_EXAMPLE OFF)
add_subdirectory(ext/TinyGLTF)

if(WIN32)
  set(PLATFORM Windows)

  add_compile_definitions(PR_PLATFORM_WINDOWS GTE_USE_MSWINDOWS GTE_USE_OPENGL)
  add_compile_definitions(NOMINMAX)

  set(CMAKE_GENERATOR_PLATFORM
      "x64"
      CACHE STRING "Platform (x64)" FORCE)
elseif(UNIX)
  set(PLATFORM Unix)
  add_compile_definitions(PR_PLATFORM_UNIX GTE_USE_LINUX GTE_USE_OPENGL
                          GLFW_USE_WAYLAND)
  set(GLFW_BUILD_WAYLAND
      ON
      CACHE BOOL "Enable Wayland support")
else()
  message(FATAL_ERROR "Platform not selected."
                      "Expected Windows or Unix platform for compilation.")
endif()

add_subdirectory(engine)

if(PRESTO_BUILD_EDITOR)
  add_subdirectory(editor)
endif()
