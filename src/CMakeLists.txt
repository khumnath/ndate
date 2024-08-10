cmake_minimum_required(VERSION 3.5)
project(ndate VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 11)

add_executable(ndate main.cpp)

set(CMAKE_INSTALL_PREFIX /usr/local)

# Install the executable (adjust the path if necessary)
install(TARGETS ndate)
