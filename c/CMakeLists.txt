cmake_minimum_required(VERSION 3.11)
project(substrait-validator)

include(FetchContent)

# Use Corrosion to make a shared librarywith target name
# "substrait-validator-c". Note that static linking is not (easily) possible
# due to the many dependencies introduced by libcurl.
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5.1
)
FetchContent_MakeAvailable(Corrosion)
corrosion_import_crate(
    MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
)

# Add the include directory with the header file generated by build.rs.
target_include_directories(
    substrait_validator_c
    INTERFACE "$<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}/include>"
)

# Handle testing.
option(
    SUBSTRAIT_VALIDATOR_BUILD_TESTS
    "Whether to build tests for the Substrait validator C bindings"
    OFF
)
if(SUBSTRAIT_VALIDATOR_BUILD_TESTS)

    # GoogleTest requires at least C++11
    set(CMAKE_CXX_STANDARD 11)

    include(FetchContent)
    FetchContent_Declare(
        googletest
        URL https://github.com/google/googletest/archive/609281088cfefc76f9d0ce82e1ff6c30cc3591e5.zip
        DOWNLOAD_EXTRACT_TIMESTAMP true
    )
    set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
    FetchContent_MakeAvailable(googletest)

    enable_testing()

    add_executable(
        substrait-validator-c-test
        ${CMAKE_CURRENT_SOURCE_DIR}/tests/test.cc
    )

    # For OSX, link in CoreFoundation, needed for some system symbols like
    # _CFRelease, _CFStringGetBytes, …
    set(EXTRA_FRAMEWORK_OPTIONS "")
    if(APPLE)
        set(EXTRA_FRAMEWORK_OPTIONS "-framework CoreFoundation")
    endif()


    target_link_libraries(
        substrait-validator-c-test
        gtest_main
        substrait_validator_c
        "${EXTRA_FRAMEWORK_OPTIONS}"
    )


    include(GoogleTest)
    gtest_discover_tests(substrait-validator-c-test)

endif()
