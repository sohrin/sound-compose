#include <stdio.h>

__declspec(dllimport) void repeat_hello(int);

int main(void)
{
    repeat_hello(5);
    return 0;
}
