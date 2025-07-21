// test_uxn11.c: Minimal test program to link and call a function from libuxn11.a
#include <stdio.h>
#include "uxn11.h"

int main(int argc, char **argv) {
    // Example: call uxn11_entry if available
    printf("Calling uxn11_entry...\n");
    int result = uxn11_entry(argc, argv);
    printf("uxn11_entry returned: %d\n", result);
    return 0;
}
