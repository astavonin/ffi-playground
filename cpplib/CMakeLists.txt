cmake_minimum_required(VERSION 3.4)

set(PROJECT_NAME cpp2rs)
project(${PROJECT_NAME})

SET (CMAKE_LIBRARY_OUTPUT_DIRECTORY
        ${PROJECT_BINARY_DIR}/bin
        CACHE PATH
        "Single Directory for all"
    )

SET (CMAKE_RUNTIME_OUTPUT_DIRECTORY
        ${PROJECT_BINARY_DIR}/bin
        CACHE PATH
        "Single Directory for all"
    )

SET (CMAKE_ARCHIVE_OUTPUT_DIRECTORY
        ${PROJECT_BINARY_DIR}/lib
        CACHE PATH
        "Single Directory for all"
    )


IF(APPLE)
    SET(CMAKE_INSTALL_NAME_DIR @loader_path)
    SET(CMAKE_BUILD_WITH_INSTALL_RPATH ON)
ENDIF(APPLE)

set(cpp2rs_sources main.cpp)

#add_library(${PROJECT_NAME} SHARED ${cpp2rs_sources})
add_executable(${PROJECT_NAME} ${cpp2rs_sources})


set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD 11)
set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD_REQUIRED ON)


