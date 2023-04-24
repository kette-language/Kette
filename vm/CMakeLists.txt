cmake_minimum_required(VERSION 3.21)

project(Kette)

add_executable(${PROJECT_NAME}
    "src/main.cpp"
    "src/mem.cpp"
    "src/context.cpp"
    "src/cfgs.cpp"
    "src/stacks.cpp"
    "src/reader/reader.cpp"
    "src/reader/symbols.cpp"
    "src/compiler/artifacts.cpp"
)

target_include_directories(${PROJECT_NAME} PRIVATE "${CMAKE_CURRENT_LIST_DIR}/include")
target_compile_features(${PROJECT_NAME} PRIVATE cxx_std_20)
