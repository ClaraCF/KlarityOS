#include "print.h"

void kernel_main(){
    // Welcome message
    print_clear();
    print_set_color(PRINT_COLOR_PURPLE, PRINT_COLOR_BLACK);
    print_str("Welcome to KlarityOS!\n\n");
}