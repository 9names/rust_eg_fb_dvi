#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define WIDTH 320

#define HEIGHT 240

#define BYTES_PER_PIXEL 2

typedef struct framebuffer framebuffer;

extern struct framebuffer FRAME;

void framebuffer_draw(void);

uint8_t *framebuffer_get_raw_pointer(void);
