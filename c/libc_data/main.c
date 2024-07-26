#include "libc_data.h"

int main() {
    libc_data data = init_libc_data();
    print_libc_data(data);
    return 0;
}