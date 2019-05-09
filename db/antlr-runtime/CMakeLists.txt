project (antlr-runtime)


include_directories(
  runtime/src
  runtime/src/atn
  runtime/src/dfa
  runtime/src/misc
  runtime/src/support
  runtime/src/tree
  runtime/src/tree/pattern
  runtime/src/tree/xpath
)


file(GLOB libantlrcpp_SRC
  "runtime/src/*.cpp"
  "runtime/src/atn/*.cpp"
  "runtime/src/dfa/*.cpp"
  "runtime/src/misc/*.cpp"
  "runtime/src/support/*.cpp"
  "runtime/src/tree/*.cpp"
  "runtime/src/tree/pattern/*.cpp"
  "runtime/src/tree/xpath/*.cpp"
)

add_library(antlr4_shared SHARED ${libantlrcpp_SRC})
add_library(antlr4_static STATIC ${libantlrcpp_SRC})
target_include_directories(antlr4_static PUBLIC runtime/src/)
target_include_directories(antlr4_shared PUBLIC runtime/src/)

if(CMAKE_SYSTEM_NAME MATCHES "Linux")
  target_link_libraries(antlr4_shared ${UUID_LIBRARIES})
  target_link_libraries(antlr4_static ${UUID_LIBRARIES})
elseif(APPLE)
  target_link_libraries(antlr4_shared ${COREFOUNDATION_LIBRARY})
  target_link_libraries(antlr4_static ${COREFOUNDATION_LIBRARY})
endif()

if(CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
  set(disabled_compile_warnings "/wd4251")
else()
  set(disabled_compile_warnings "-Wno-overloaded-virtual")
endif()


if("${CMAKE_CXX_COMPILER_ID}" MATCHES "Clang")
  set(disabled_compile_warnings "${disabled_compile_warnings} -Wno-dollar-in-identifier-extension -Wno-four-char-constants")
elseif("${CMAKE_CXX_COMPILER_ID}" MATCHES "GNU" OR "${CMAKE_CXX_COMPILER_ID}" MATCHES "Intel")
  set(disabled_compile_warnings "${disabled_compile_warnings} -Wno-multichar")
endif()

set(extra_share_compile_flags "")
set(extra_static_compile_flags "")
if(WIN32)
  set(extra_share_compile_flags "-DANTLR4CPP_EXPORTS")
  set(extra_static_compile_flags "-DANTLR4CPP_STATIC")
endif(WIN32)
if(CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
  if(WITH_STATIC_CRT)
    target_compile_options(antlr4_shared PRIVATE "/MT$<$<CONFIG:Debug>:d>")
    target_compile_options(antlr4_static PRIVATE "/MT$<$<CONFIG:Debug>:d>")
  else()
    target_compile_options(antlr4_shared PRIVATE "/MD$<$<CONFIG:Debug>:d>")
    target_compile_options(antlr4_static PRIVATE "/MD$<$<CONFIG:Debug>:d>")
  endif()
endif()

set(static_lib_suffix "")
if(CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
  set(static_lib_suffix "-static")
endif()

if(CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
  set(extra_share_compile_flags "-DANTLR4CPP_EXPORTS -MP /wd4251")
  set(extra_static_compile_flags "-DANTLR4CPP_STATIC -MP")
endif()

set_target_properties(antlr4_shared
                      PROPERTIES VERSION   ${ANTLR_VERSION}
                                 SOVERSION ${ANTLR_VERSION}
                                 OUTPUT_NAME antlr4-runtime
                                 COMPILE_FLAGS "${disabled_compile_warnings} ${extra_share_compile_flags}")

set_target_properties(antlr4_static
                      PROPERTIES VERSION   ${ANTLR_VERSION}
                                 SOVERSION ${ANTLR_VERSION}
                                 OUTPUT_NAME "antlr4-runtime${static_lib_suffix}"
                                 COMPILE_FLAGS "${disabled_compile_warnings} ${extra_static_compile_flags}")


install(TARGETS antlr4_shared DESTINATION bin)