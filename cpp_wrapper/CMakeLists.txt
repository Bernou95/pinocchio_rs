cmake_minimum_required(VERSION 3.20.2)

# Set the project name
project(pinocchio_wrapper)

# Set C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# Compiler flags
#target_compile_options(${PROJECT_NAME}  PUBLIC -Wall -Wno-sign-compare)

# Find Pinocchio
find_package(pinocchio REQUIRED)

# Create library
add_library(pinocchio_wrapper SHARED
    pinocchio_wrapper.cpp
)

# Include directories
target_include_directories(pinocchio_wrapper PRIVATE
    ${pinocchio_INCLUDE_DIRS}
)

# Link against pinocchio and its dependencies
target_link_libraries(pinocchio_wrapper
    pinocchio::pinocchio
)


