project (tests-cypher)

file(GLOB src "src/*.cpp")
file(GLOB headers "src/*.h")


add_executable(tests-cypher ${headers} ${src})

set (BOOST_ROOT ${BOOST_INSTALL_PATH})
set (Boost_NO_SYSTEM_PATHS ON)
set (Boost_USE_MULTITHREADED ON)
set (Boost_USE_STATIC_LIBS ON)
set (Boost_USE_STATIC_RUNTIME OFF)
set (BOOST_ALL_DYN_LINK OFF)

find_package(Boost 1.68.0 REQUIRED COMPONENTS system filesystem graph)

if (Boost_FOUND)
target_include_directories(tests-cypher PRIVATE ${Boost_INCLUDE_DIR})
endif()
target_link_libraries(tests-cypher graph-store cypher-engine tests-common)

install(TARGETS tests-cypher DESTINATION bin)


add_test(NAME TESTS-CYPHER COMMAND tests-cypher)