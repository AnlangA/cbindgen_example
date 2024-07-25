path := "../../target/debug"
CC := "clang"

#<package> is the rust package. example: you can test the "add" package by "just run add"
run package:
    #!/usr/bin/env bash
    set -euxo pipefail
    
    cargo build -p {{package}}
    cd c/{{package}}
    {{CC}} main.c -L{{path}} -l{{package}} -o main
    ./main