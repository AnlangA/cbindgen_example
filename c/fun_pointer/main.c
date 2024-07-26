#include "stdio.h"
#include "fun_pointer.h"

void my_printer(const char* message) {
    printf("%s\n", message);
}

int main() {
    const char* message = "fun_pointer:Hello from Rust!";
    call_printer(my_printer, message);
    return 0;
}