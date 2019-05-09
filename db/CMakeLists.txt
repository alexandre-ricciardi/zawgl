cmake_minimum_required (VERSION 3.3.1)
project (OrangeDB)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
#set(CMAKE_CXX_EXTENSIONS OFF)
set(BOOST_INSTALL_PATH STRING "C:/libs/boost")
set(ANTLR4_INSTALL_PATH "C:/libs/antlr4")



#set(CMAKE_MODULE_PATH  ${CMAKE_MODULE_PATH} ${XERCES_INSTALL_PATH} )
#set(CMAKE_INSTALL_PREFIX "install")
message("Searching for boost install in ${BOOST_INSTALL_PATH}")


add_library(antlr4_shared SHARED IMPORTED )
set_target_properties(antlr4_shared PROPERTIES IMPORTED_LOCATION ${ANTLR4_INSTALL_PATH}/antlr4-runtime.dll)
set_target_properties(antlr4_shared PROPERTIES IMPORTED_IMPLIB ${ANTLR4_INSTALL_PATH}/antlr4-runtime.lib)
target_include_directories(antlr4_shared INTERFACE ${ANTLR4_INSTALL_PATH}/include)
install(FILES antlr4_shared DESTINATION bin)

add_subdirectory(model)
add_subdirectory(graph-store)
add_subdirectory(graph-repository)
add_subdirectory(cypher-engine)
enable_testing()
add_subdirectory(tests-common)
add_subdirectory(tests-store)
add_subdirectory(tests-repository)
add_subdirectory(tests-cypher)
