#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

bool do_the_thing(uint8_t *inout, size_t inout_size)
{
    // manipulate the input
    size_t i;

    for (i = 0; i < inout_size; i++) {
        inout[i] *= 2;
    }
}
