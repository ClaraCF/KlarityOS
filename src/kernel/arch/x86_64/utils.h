#pragma once

#include <stdbool.h>

#define MAX_HEAP_MEM 1048576    // 1MiB

void reverse(char str[], int length);
char *itoa(int num, char* str, int base);
char *itox(unsigned x, char *dest);
