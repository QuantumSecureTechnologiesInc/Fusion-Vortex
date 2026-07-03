# CMake generated Testfile for 
# Source directory: C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex
# Build directory: C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/build
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
if(CTEST_CONFIGURATION_TYPE MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
  add_test("test_blinding" "C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/build/Debug/test_blinding.exe")
  set_tests_properties("test_blinding" PROPERTIES  _BACKTRACE_TRIPLES "C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/CMakeLists.txt;238;add_test;C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/CMakeLists.txt;0;")
elseif(CTEST_CONFIGURATION_TYPE MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
  add_test("test_blinding" "C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/build/Release/test_blinding.exe")
  set_tests_properties("test_blinding" PROPERTIES  _BACKTRACE_TRIPLES "C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/CMakeLists.txt;238;add_test;C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/CMakeLists.txt;0;")
elseif(CTEST_CONFIGURATION_TYPE MATCHES "^([Mm][Ii][Nn][Ss][Ii][Zz][Ee][Rr][Ee][Ll])$")
  add_test("test_blinding" "C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/build/MinSizeRel/test_blinding.exe")
  set_tests_properties("test_blinding" PROPERTIES  _BACKTRACE_TRIPLES "C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/CMakeLists.txt;238;add_test;C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/CMakeLists.txt;0;")
elseif(CTEST_CONFIGURATION_TYPE MATCHES "^([Rr][Ee][Ll][Ww][Ii][Tt][Hh][Dd][Ee][Bb][Ii][Nn][Ff][Oo])$")
  add_test("test_blinding" "C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/build/RelWithDebInfo/test_blinding.exe")
  set_tests_properties("test_blinding" PROPERTIES  _BACKTRACE_TRIPLES "C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/CMakeLists.txt;238;add_test;C:/Users/Matth/Downloads/Fusion v2.0 Vortex/v2.0 Vortex/CMakeLists.txt;0;")
else()
  add_test("test_blinding" NOT_AVAILABLE)
endif()
