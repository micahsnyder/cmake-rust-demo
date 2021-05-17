/*
 * Sample application that uses a common lib to generate a UUID
 * and uses the CMakeRust library's colorlog featuer toe log the UUID.
 *
 * Copyright 2020 (C) Micah Snyder
 */

#include <stdio.h>
#include <string.h>

#include "demo-rust.h"
#include "gen_uuid.h"

int main(void)
{
    char *my_uuid = {0};

    my_uuid = gen_uuid();

    printf("%s\n", my_uuid);
    clog_debug((const uint8_t *)my_uuid, strlen(my_uuid));
    clog_info((const uint8_t *)my_uuid, strlen(my_uuid));
    clog_warning((const uint8_t *)my_uuid, strlen(my_uuid));
    clog_error((const uint8_t *)my_uuid, strlen(my_uuid));

    free_uuid(my_uuid);

    return 0;
}