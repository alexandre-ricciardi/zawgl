project (model)

file(GLOB src "src/*.cpp")
file(GLOB headers "src/*.h")

add_library(model INTERFACE)
target_include_directories(model INTERFACE src/)
set (BOOST_ROOT ${BOOST_INSTALL_PATH})
set (Boost_NO_SYSTEM_PATHS ON)
set (Boost_USE_MULTITHREADED ON)
set (Boost_USE_STATIC_LIBS ON)
set (Boost_USE_STATIC_RUNTIME OFF)
set (BOOST_ALL_DYN_LINK OFF)

find_package(Boost 1.68.0 REQUIRED COMPONENTS graph)

if (Boost_FOUND)
#target_include_directories(model PRIVATE ${Boost_INCLUDE_DIR})
endif()
#target_link_libraries(graph-store Qt5::Core Qt5::Widgets Qt5::OpenGL Qt5::3DCore ${OPENGL_LIBRARIES} connectors)
install(TARGETS model DESTINATION bin)
