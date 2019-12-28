project (cypher-model)

file(GLOB src "src/*.cpp")
file(GLOB headers "src/*.h")

add_library(cypher-model INTERFACE)
target_include_directories(cypher-model INTERFACE src/)

set (BOOST_ROOT ${BOOST_INSTALL_PATH})
set (Boost_NO_SYSTEM_PATHS ON)
set (Boost_USE_MULTITHREADED ON)
set (Boost_USE_STATIC_LIBS ON)
set (Boost_USE_STATIC_RUNTIME OFF)
set (BOOST_ALL_DYN_LINK OFF)

find_package(Boost 1.68.0 REQUIRED COMPONENTS system filesystem graph)

target_link_libraries(cypher-model INTERFACE model)
install(TARGETS cypher-model DESTINATION bin)