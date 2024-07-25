#include "enum_data.h"
#include <stdint.h>
#include <stdlib.h>
int main(){
    uint16_t u16data = 16;
    const int8_t u8data[3] = {1,2,3};
    size_t len = 3;
    struct EnumData *data1 = new_enum_data(Data1, "hi", u16data, u8data, len);
    print_enum_data(data1);
    struct EnumData *data2 = new_enum_data(Data2, "hi", u16data, u8data, len);
    print_enum_data(data2);
    struct EnumData *data3 = new_enum_data(Data3, "hi", u16data, u8data, len);
    print_enum_data(data3);

    //new_enum_data return null
    struct EnumData *data4 = new_enum_data(3u, "hi", u16data, u8data, len);
    print_enum_data(data4);
}
