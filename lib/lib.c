/*
 * Example library that has some features written in Rust.
 *
 * Copyright (C) 2020 Micah Snyder.
 */

#include <stdio.h>
#include <string.h>

#include "lib_private.h"
#include "cmakerust.h"
#include "colorlog.h"

void cmakerust_init() {
    clog_debug(init_message, strlen(init_message));
}

void cmakerust_fini() {
    clog_debug(fini_message, strlen(fini_message));
}
