#include <stdlib.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

static const int BUFF_LEN=5;

int foo_alloc(int key, int** data, int* len)
{
    *len = BUFF_LEN;
    *data = (int*)calloc(BUFF_LEN, sizeof(int));
    for (int i = 0; i < BUFF_LEN; ++i) {
        (*data)[i] = i*i;
    }

    printf("allocated for %d with addr %x\n", key, *data);

    return key;
}

void foo_free(int *data)
{
    printf("free with addr %x\n", data);

    free(data);
}

#ifdef __cplusplus
}
#endif

#if 0

#include <iostream>
#include <iomanip>

int main(void)
{
    int *arr=nullptr;
    int len=0;
    foo_alloc(&arr, &len);

    std::cout << std::hex << std::setfill('0');
    std::cout << arr << std::endl;
    for (int i = 0; i < len; ++i) {
        std::cout << arr[i] << std::endl;
    }
    foo_free(arr);

    return 0;
}

#endif

