project (graph-store)

file(GLOB src "src/*.cpp")
file(GLOB headers "src/*.h")

add_library(graph-store SHARED ${headers} ${src})
target_include_directories(graph-store PUBLIC src/)
target_compile_definitions(graph-store PRIVATE DB_EXPORT)
set (BOOST_ROOT ${BOOST_INSTALL_PATH})
set (Boost_NO_SYSTEM_PATHS ON)
set (Boost_USE_MULTITHREADED ON)
set (Boost_USE_STATIC_LIBS ON)
set (Boost_USE_STATIC_RUNTIME OFF)
set (BOOST_ALL_DYN_LINK OFF)

find_package(Boost 1.68.0 REQUIRED COMPONENTS system filesystem graph)

if (Boost_FOUND)
target_include_directories(graph-store PRIVATE ${Boost_INCLUDE_DIR})
endif()
target_link_libraries(graph-store model)
install(TARGETS graph-store DESTINATION bin)
