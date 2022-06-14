# KlarityOS Project Information
set(OS_NAME     "KlarityOS")
set(OS_VERSION  "0.1.0-alpha")


# Configure CMake not to suffer a brain aneurysm
cmake_minimum_required(VERSION 3.0)
enable_language(ASM-NASM)
set(CMAKE_TRY_COMPILE_TARGET_TYPE "STATIC_LIBRARY")

# Ask CMake to pass the linker script to the linker
set(CMAKE_EXE_LINKER_FLAGS "-T  '${PROJECT_SOURCE_DIR}/link.ld'")


# Kernel and C objects compilation flags
set(CMAKE_C_FLAGS "-ffreestanding -O2 -static -Ttext 0x8000 -m64")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wwrite-strings -mno-red-zone")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -W -Wall -Wextra -pedantic")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -W -Wall -Wextra -pedantic")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wno-unused-function")
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wno-unused-parameter")

set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS}")


# Configure target architecture to build
set(BUILD_ARCH_X86_64 YES)  # Only one available right now

# Build the selected targets
if(${BUILD_ARCH_X86_64})
    # Build x86_64 target
    include src
endif()
