cmake_minimum_required(VERSION 3.16)

project(EvacuationC VERSION 0.1.0)

option(build_tests "Build all of own tests" OFF)

set(CMAKE_VERBOSE_MAKEFILE TRUE)

if(MSVC)
    set(CMAKE_C_FLAGS "/Wall")
else()
    set(CMAKE_C_FLAGS "-std=gnu18 -Wall -D_GNU_SOURCE -Wparentheses -Wshadow -funsigned-char")
endif()
set(CMAKE_C_FLAGS_DEBUG     "-g -O0")
#set(CMAKE_C_FLAGS_DEBUG     "${CMAKE_C_FLAGS_DEBUG} -fsanitize=address -fsanitize=pointer-subtract -fsanitize=shift")
set(CMAKE_C_FLAGS_RELEASE   "-O3 -DNDEBUG")

if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

# corrosion
add_subdirectory(thirdparty/corrosion)
corrosion_import_crate(MANIFEST_PATH src/bim_cli/Cargo.toml)
corrosion_import_crate(MANIFEST_PATH src/bim_json_object/Cargo.toml)

add_executable(${PROJECT_NAME}
    src/bim_tools.c         src/bim_tools.h
    src/bim_graph.c         src/bim_graph.h
    src/bim_evac.c          src/bim_evac.h
    src/bim_polygon_tools.c src/bim_polygon_tools.h
    src/bim_configure.c     src/bim_configure.h
    src/bim_json_object.c   src/bim_json_object.h
                            src/bim_uuid.h
                            src/bim_cli/bim_cli.h
    src/bim_output.c        src/bim_output.h
    src/main.c
    )

target_include_directories(${PROJECT_NAME}
    PUBLIC
        ./thirdparty/triangle
        ./thirdparty/arraylist
        ./thirdparty/json-c
        ./thirdparty/c-logger
    )

target_link_libraries(${PROJECT_NAME}
    PUBLIC
        bim_cli
        bim_json_object
    PRIVATE
        logger_static
        triangle
        pthread
        arraylist
        json-c
        m
    )


add_subdirectory(thirdparty/triangle)
add_subdirectory(thirdparty/arraylist)
add_subdirectory(thirdparty/c-logger)

# json-c config
set(CMAKE_POLICY_DEFAULT_CMP0077 NEW)
#set(BUILD_STATIC_LIBS ON)
set(BUILD_SHARED_LIBS OFF)
add_subdirectory(thirdparty/json-c)

### Test
if(build_tests)
    enable_testing()
    add_subdirectory(test)
endif()