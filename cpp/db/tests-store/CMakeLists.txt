project (tests-store)

file(GLOB src "src/*.cpp")
file(GLOB headers "src/*.h")


add_executable(tests-store ${headers} ${src})

set (BOOST_ROOT ${BOOST_INSTALL_PATH})
set (Boost_NO_SYSTEM_PATHS ON)
set (Boost_USE_MULTITHREADED ON)
set (Boost_USE_STATIC_LIBS ON)
set (Boost_USE_STATIC_RUNTIME OFF)
set (BOOST_ALL_DYN_LINK OFF)

find_package(Boost 1.68.0 REQUIRED COMPONENTS system filesystem graph)

if (Boost_FOUND)
target_include_directories(tests-store PRIVATE ${Boost_INCLUDE_DIR})
endif()
target_link_libraries(tests-store graph-store graph-repository tests-common)
install(TARGETS tests-store DESTINATION bin)

add_test(NAME TESTS-STORE COMMAND tests-store)