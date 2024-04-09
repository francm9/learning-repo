# Install script for directory: /Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/dev-francm/Documents/dev/Rust/MQTT/mqtt-example/target/debug/build/paho-mqtt-sys-44174a14430fca44/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/Users/dev-francm/Documents/dev/Rust/MQTT/mqtt-example/target/debug/build/paho-mqtt-sys-44174a14430fca44/out/build/src/libpaho-mqtt3c.a")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3c.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3c.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3c.a")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/Users/dev-francm/Documents/dev/Rust/MQTT/mqtt-example/target/debug/build/paho-mqtt-sys-44174a14430fca44/out/build/src/libpaho-mqtt3a.a")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3a.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3a.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3a.a")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE FILES
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/MQTTAsync.h"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/MQTTClient.h"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/MQTTClientPersistence.h"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/MQTTProperties.h"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/MQTTReasonCodes.h"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/MQTTSubscribeOpts.h"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/MQTTExportDeclarations.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/Users/dev-francm/Documents/dev/Rust/MQTT/mqtt-example/target/debug/build/paho-mqtt-sys-44174a14430fca44/out/build/src/libpaho-mqtt3cs.a")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3cs.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3cs.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3cs.a")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/Users/dev-francm/Documents/dev/Rust/MQTT/mqtt-example/target/debug/build/paho-mqtt-sys-44174a14430fca44/out/build/src/libpaho-mqtt3as.a")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3as.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3as.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libpaho-mqtt3as.a")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/eclipse-paho-mqtt-c/eclipse-paho-mqtt-cConfig.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/eclipse-paho-mqtt-c/eclipse-paho-mqtt-cConfig.cmake"
         "/Users/dev-francm/Documents/dev/Rust/MQTT/mqtt-example/target/debug/build/paho-mqtt-sys-44174a14430fca44/out/build/src/CMakeFiles/Export/dd175520bdcfdcc5f75bc4f14a6d7fe8/eclipse-paho-mqtt-cConfig.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/eclipse-paho-mqtt-c/eclipse-paho-mqtt-cConfig-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/eclipse-paho-mqtt-c/eclipse-paho-mqtt-cConfig.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/eclipse-paho-mqtt-c" TYPE FILE FILES "/Users/dev-francm/Documents/dev/Rust/MQTT/mqtt-example/target/debug/build/paho-mqtt-sys-44174a14430fca44/out/build/src/CMakeFiles/Export/dd175520bdcfdcc5f75bc4f14a6d7fe8/eclipse-paho-mqtt-cConfig.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/eclipse-paho-mqtt-c" TYPE FILE FILES "/Users/dev-francm/Documents/dev/Rust/MQTT/mqtt-example/target/debug/build/paho-mqtt-sys-44174a14430fca44/out/build/src/CMakeFiles/Export/dd175520bdcfdcc5f75bc4f14a6d7fe8/eclipse-paho-mqtt-cConfig-debug.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/eclipse-paho-mqtt-c" TYPE FILE FILES "/Users/dev-francm/Documents/dev/Rust/MQTT/mqtt-example/target/debug/build/paho-mqtt-sys-44174a14430fca44/out/build/src/eclipse-paho-mqtt-cConfigVersion.cmake")
endif()

