#include "print.h"
#include "io.h"

void kernel_main(){
    // Welcome message
    print_clear();
    print_set_color(PRINT_COLOR_PURPLE, PRINT_COLOR_BLACK);
    print_str("Welcome to KlarityOS!\n\n");

    print_str("Number: ");
    print_num(1234567890);

    print_str("\n\nHex value: ");
    print_hex(0xb8000);
}