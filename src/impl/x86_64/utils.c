#include "utils.h"


char heap[MAX_HEAP_MEM];
unsigned heap_counter = 0;


void swap(char *a, char *b)
{
    *a ^= *b;
    *b ^= *a;
    *a ^= *b;
}

/* A utility function to reverse a string  */
void reverse(char str[], int length)
{
    int start = 0;
    int end = length -1;
    while (start < end)
    {
        swap((str+start), (str+end));
        start++;
        end--;
    }
}

// Implementation of itoa()
char *itoa(int num, char *str, int base) {
    int i = 0;
    bool isNegative = false;

    /* Handle 0 explicitly, otherwise empty string is printed for 0 */
    if (num == 0)
    {
        str[i++] = '0';
        str[i] = '\0';
        return str;
    }

    // In standard itoa(), negative numbers are handled only with
    // base 10. Otherwise numbers are considered unsigned.
    if (num < 0 && base == 10)
    {
        isNegative = true;
        num = -num;
    }

    // Process individual digits
    while (num != 0)
    {
        int rem = num % base;
        str[i++] = (rem > 9)? (rem-10) + 'a' : rem + '0';
        num = num/base;
    }

    // If number is negative, append '-'
    if (isNegative)
        str[i++] = '-';

    str[i] = '\0'; // Append string terminator

    // Reverse the string
    reverse(str, i);

    return str;
}

// print digit at the end and return the next address
char *itox_helper(char *dest, unsigned x) {
    if (x >= 16) {
        dest = itox_helper(dest, x/16);
    }
    *dest++ = "0123456789abcdef"[x & 15];
    return dest;
}

char *itox(unsigned x, char *dest) {
    // Prepend the '0x' prefix
    *dest = '0';
    *(dest+1) = 'x';

    // Get the hex value as a string
    // Skip first 2 bytes for '0x'
    *itox_helper(dest+2, x) = '\0';

    return dest;
}