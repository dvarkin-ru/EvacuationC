add_library(arraylist STATIC
    arraylist.c arraylist.h
    )

if(MSVC)
    target_compile_options(arraylist PRIVATE "/Wall")
else()
    target_compile_options(arraylist
        PRIVATE
            -Wno-unused-variable
            -Wno-unused-but-set-variable
            -Wno-pedantic
            -Wunused-parameter
            -Wno-unused-parameter
    )
endif()
