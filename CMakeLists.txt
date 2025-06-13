cmake_minimum_required(VERSION 3.20)
project(prex)

set(CMAKE_C_STANDARD 90)
set(CMAKE_C_STANDARD_REQUIRED YES)

include_directories("core")

add_compile_options(-Wall -Wextra -Wpedantic)

file(GLOB_RECURSE DAEMON_SORUCES CONFIGURE_DEPENDS "prexd/*.c")
add_executable(prexd ${DAEMON_SORUCES})

file(GLOB_RECURSE CLIENT_SORUCES CONFIGURE_DEPENDS "prex/*.c")
add_executable(prex ${CLIENT_SORUCES})
