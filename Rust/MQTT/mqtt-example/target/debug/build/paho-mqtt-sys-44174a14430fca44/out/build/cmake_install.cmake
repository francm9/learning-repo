# Install script for directory: /Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c

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
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/doc/Eclipse Paho C/samples" TYPE FILE FILES
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/samples/MQTTAsync_publish.c"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/samples/MQTTAsync_publish_time.c"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/samples/MQTTAsync_subscribe.c"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/samples/MQTTClient_publish.c"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/samples/MQTTClient_publish_async.c"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/samples/MQTTClient_subscribe.c"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/samples/paho_c_pub.c"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/samples/paho_c_sub.c"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/samples/paho_cs_pub.c"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/samples/paho_cs_sub.c"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/src/samples/pubsub_opts.c"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/doc/Eclipse Paho C" TYPE FILE FILES
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/CONTRIBUTING.md"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/epl-v20"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/edl-v10"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/README.md"
    "/Users/dev-francm/.cargo/registry/src/index.crates.io-6f17d22bba15001f/paho-mqtt-sys-0.9.0/paho.mqtt.c/notice.html"
    )
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/Users/dev-francm/Documents/dev/Rust/MQTT/mqtt-example/target/debug/build/paho-mqtt-sys-44174a14430fca44/out/build/src/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/Users/dev-francm/Documents/dev/Rust/MQTT/mqtt-example/target/debug/build/paho-mqtt-sys-44174a14430fca44/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
