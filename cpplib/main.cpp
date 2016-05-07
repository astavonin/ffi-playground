#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

static const int BUFF_LEN=5;

int foo_alloc(int** data, int* len)
{
    *len = BUFF_LEN;
    *data = (int*)calloc(BUFF_LEN, sizeof(int));
    for (int i = 0; i < BUFF_LEN; ++i) {
        (*data)[i] = i*i;
    }
    return 0;
}

void foo_free(int **data, int len)
{
    free(*data);
}

#ifdef __cplusplus
}
#endif

#if 1

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
    foo_free(&arr, len);

    return 0;
}

#endif

