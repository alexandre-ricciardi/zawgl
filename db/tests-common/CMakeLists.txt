project (tests-common)

file(GLOB src "src/*.cpp")
file(GLOB headers "src/*.h")


add_library(tests-common INTERFACE)
target_include_directories(tests-common INTERFACE src/)