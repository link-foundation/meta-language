// This harness supplies input and context to the authenticated original scanner.
// It contains no delimiter decisions. Native compilation runs only in CI.
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include "scanner.c"

typedef struct {
    TSLexer lexer;
    unsigned char *bytes;
    size_t size;
    size_t cursor;
    ptrdiff_t marked;
} SourceInput;

static unsigned read_hex_digit(char value) {
    if (value >= '0' && value <= '9') return (unsigned)(value - '0');
    if (value >= 'a' && value <= 'f') return (unsigned)(value - 'a' + 10);
    abort();
}

static size_t read_source_scalar(SourceInput *input, int32_t *scalar) {
    if (input->cursor == input->size) { *scalar = 0; return 0; }
    unsigned char first = input->bytes[input->cursor];
    size_t width = first < 0x80 ? 1 : first < 0xe0 ? 2 : first < 0xf0 ? 3 : 4;
    if (input->cursor + width > input->size) abort();
    uint32_t value = width == 1 ? first : first & (0x7fU >> width);
    for (size_t index = 1; index < width; index++) {
        unsigned char next = input->bytes[input->cursor + index];
        if ((next & 0xc0U) != 0x80U) abort();
        value = (value << 6) | (next & 0x3fU);
    }
    *scalar = (int32_t)value;
    return width;
}

static void advance_source_lexer(TSLexer *lexer, bool skip) {
    SourceInput *input = (SourceInput *)lexer;
    if (skip || input->cursor == input->size) abort();
    int32_t scalar;
    input->cursor += read_source_scalar(input, &scalar);
    read_source_scalar(input, &lexer->lookahead);
}

static void mark_source_lexer_end(TSLexer *lexer) {
    SourceInput *input = (SourceInput *)lexer;
    input->marked = (ptrdiff_t)input->cursor;
}

static bool source_input_ended(const TSLexer *lexer) {
    const SourceInput *input = (const SourceInput *)lexer;
    return input->cursor == input->size;
}

int main(int count, char **arguments) {
    if (count != 7) return 2;
    unsigned long mask = strtoul(arguments[1], NULL, 10);
    if (mask > 32767) return 2;
    char initial[4];
    for (unsigned index = 0; index < 4; index++) {
        unsigned long value = strtoul(arguments[index + 2], NULL, 10);
        if (value > 255) return 2;
        initial[index] = (char)value;
    }
    const char *hex = arguments[6];
    if (strlen(hex) % 2 != 0) return 2;
    SourceInput input = {0};
    input.size = strlen(hex) / 2;
    input.bytes = malloc(input.size == 0 ? 1 : input.size);
    if (input.bytes == NULL) return 2;
    input.marked = -1;
    for (size_t index = 0; index < input.size; index++) {
        input.bytes[index] = (unsigned char)((read_hex_digit(hex[index * 2]) << 4) | read_hex_digit(hex[index * 2 + 1]));
    }
    input.lexer.advance = advance_source_lexer;
    input.lexer.mark_end = mark_source_lexer_end;
    input.lexer.eof = source_input_ended;
    read_source_scalar(&input, &input.lexer.lookahead);
    bool valid[15];
    for (unsigned index = 0; index < 15; index++) valid[index] = (mask & (1UL << index)) != 0;
    Scanner scanner;
    deserialize(&scanner, initial, 4);
    bool accepted = scan(&scanner, &input.lexer, valid);
    char final[4];
    if (serialize(&scanner, final) != 4) abort();
    printf("%d %d %zu %td %u %u %u %u\n", accepted ? 1 : 0,
           accepted ? (int)input.lexer.result_symbol : -1, input.cursor, input.marked,
           (unsigned)(uint8_t)final[0], (unsigned)(uint8_t)final[1],
           (unsigned)(uint8_t)final[2], (unsigned)(uint8_t)final[3]);
    free(input.bytes);
    return 0;
}
