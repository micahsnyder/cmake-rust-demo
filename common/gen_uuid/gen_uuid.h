/* Copyright (C) 2020 Micah Snyder. */

#ifndef __GEN_UUID_H
#define __GEN_UUID_H

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void free_uuid(char *uuid_ptr);

char *gen_uuid(void);

#endif /* __GEN_UUID_H */
