#include "spec_data.h"
#include "stdio.h"

// C语言包装函数
int sum_numbers(int count, ...) {
    va_list args;
    va_start(args, count);
    int result = use_va_list(count, args);
    va_end(args);
    return result;
}

int main(){
    void* data = use_maybe_uninit();
    printf("data: %x\n",*(int*)data);
    free_int(data);
    printf("free mem");

    int sum = sum_numbers(4, 1, 1, 1);
    printf("sum of 1..7 = %x", sum);
    return 0;
}