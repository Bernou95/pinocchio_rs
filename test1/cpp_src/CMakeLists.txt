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
find_package(Eigen3 REQUIRED)

add_library(pinocchio_example SHARED pinocchio_example.cpp)

# Include directories
target_include_directories(pinocchio_example PRIVATE
    ${pinocchio_INCLUDE_DIRS}
    ${EIGEN3_INCLUDE_DIR}
)

target_link_libraries(pinocchio_example pinocchio::pinocchio Eigen3::Eigen)

install(TARGETS pinocchio_example
    RUNTIME DESTINATION bin          # For executables
    LIBRARY DESTINATION lib          # For shared libraries
    ARCHIVE DESTINATION lib          # For static libraries
)



