path := "../../target/debug"
CC := "cc"

#<package> is the rust package. example: you can test the "add" package by "just run add"
run package:
    #!/usr/bin/env bash
    set -euxo pipefail
    
    cargo build -p {{package}}
    cd A_c_call_rust/{{package}}
    {{CC}} main.c -L{{path}} -l{{package}} -o main
    ./main