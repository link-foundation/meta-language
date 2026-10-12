#include "tree_sitter/parser.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>

#define MAX(a, b) ((a) > (b) ? (a) : (b))

#define VEC_RESIZE(vec, _cap)                                                  \
    void *tmp = realloc((vec).data, (_cap) * sizeof((vec).data[0]));           \
    assert(tmp != NULL);                                                       \
    (vec).data = tmp;                                                          \
    (vec).cap = (_cap);

#define VEC_GROW(vec, _cap)                                                    \
    if ((vec).cap < (_cap)) {                                                  \
        VEC_RESIZE((vec), (_cap));                                             \
    }

#define VEC_PUSH(vec, el)                                                      \
    if ((vec).cap == (vec).len) {                                              \
        VEC_RESIZE((vec), MAX(16, (vec).len * 2));                             \
    }                                                                          \
    (vec).data[(vec).len++] = (el);

#define VEC_POP(vec) (vec).len--;

#define VEC_NEW                                                                \
    { .len = 0, .cap = 0, .data = NULL }

#define VEC_BACK(vec) ((vec).data[(vec).len - 1])

#define VEC_FREE(vec)                                                          \
    {                                                                          \
        if ((vec).data != NULL)                                                \
            free((vec).data);                                                  \
    }

#define VEC_CLEAR(vec) (vec).len = 0;

#define QUEUE_RESIZE(queue, _cap)                                              \
    do {                                                                       \
        uint32_t count = (queue).tail - (queue).head;                          \
        uint16_t *tmp = malloc((_cap) * sizeof((queue).data[0]));              \
        assert(tmp != NULL);                                                   \
        for (uint32_t index = 0; index < count; index++) {                     \
            tmp[index] = (queue).data[((queue).head + index) % (queue).cap];   \
        }                                                                      \
        free((queue).data);                                                    \
        (queue).data = tmp;                                                    \
        (queue).cap = (_cap);                                                  \
        (queue).head = 0;                                                      \
        (queue).tail = count;                                                  \
    } while (0)

#define QUEUE_GROW(queue, _cap)                                                \
    do {                                                                       \
        if ((queue).cap < (_cap)) {                                            \
            QUEUE_RESIZE((queue), (_cap));                                     \
        }                                                                      \
    } while (0)

#define QUEUE_PUSH(queue, el)                                                  \
    do {                                                                       \
        if ((queue).cap == 0) {                                                \
            QUEUE_RESIZE((queue), 16);                                         \
        } else if ((queue).cap == ((queue).tail - (queue).head)) {             \
            QUEUE_RESIZE((queue), (queue).cap * 2);                            \
        }                                                                      \
        (queue).data[(queue).tail % (queue).cap] = (el);                       \
        (queue).tail++;                                                        \
    } while (0)

#define QUEUE_POP(queue)                                                       \
    do {                                                                       \
        assert((queue).head < (queue).tail);                                   \
        (queue).head++;                                                        \
    } while (0)

#define QUEUE_FRONT(queue) (queue).data[(queue).head % (queue).cap]

#define QUEUE_EMPTY(queue) ((queue).head == (queue).tail)

#define QUEUE_NEW                                                              \
    { .head = 0, .tail = 0, .cap = 0, .data = NULL }

#define QUEUE_FREE(queue)                                                      \
    do {                                                                       \
        if ((queue).data != NULL)                                              \
            free((queue).data);                                                \
    } while (0)

#define QUEUE_CLEAR(queue)                                                     \
    do {                                                                       \
        (queue).head = 0;                                                      \
        (queue).tail = 0;                                                      \
    } while (0)

enum TokenType {
    NEWLINE,
    INDENT,
    DEDENT,
};

typedef struct {
    uint32_t len;
    uint32_t cap;
    uint16_t *data;
} indent_vec;

static indent_vec indent_vec_new() {
    indent_vec vec = VEC_NEW;
    vec.data = calloc(1, sizeof(uint16_t));
    vec.cap = 1;
    return vec;
}

typedef struct {
    uint32_t head;
    uint32_t tail;
    uint32_t cap;
    uint16_t *data;
} token_queue;

static token_queue token_queue_new() {
    token_queue queue = QUEUE_NEW;
    queue.data = calloc(1, sizeof(uint16_t));
    queue.cap = 1;
    return queue;
}

typedef struct {
    indent_vec indents;
    uint32_t queued_dedent_count;
    token_queue tokens;
} Scanner;

static inline void advance(TSLexer *lexer) { lexer->advance(lexer, false); }

static inline void skip(TSLexer *lexer) { lexer->advance(lexer, true); }

bool tree_sitter_agda_external_scanner_scan(void *payload, TSLexer *lexer,
                                            const bool *valid_symbols) {
    Scanner *scanner = (Scanner *)payload;

    if (QUEUE_EMPTY(scanner->tokens)) {
        if (valid_symbols[DEDENT] && scanner->queued_dedent_count > 0) {
            scanner->queued_dedent_count--;
            QUEUE_PUSH(scanner->tokens, DEDENT);
            QUEUE_PUSH(scanner->tokens, NEWLINE);
        } else {
            bool skipped_newline = false;

            while (lexer->lookahead == ' ' || lexer->lookahead == '\t' ||
                   lexer->lookahead == '\r' || lexer->lookahead == '\n') {
                if (lexer->lookahead == '\n') {
                    skipped_newline = true;
                    skip(lexer);
                } else {
                    skip(lexer);
                }
            }

            if (lexer->eof(lexer)) {
                if (valid_symbols[DEDENT] && scanner->indents.len > 1) {
                    VEC_POP(scanner->indents);
                    QUEUE_PUSH(scanner->tokens, DEDENT);
                    QUEUE_PUSH(scanner->tokens, NEWLINE);
                } else if (valid_symbols[NEWLINE]) {
                    QUEUE_PUSH(scanner->tokens, NEWLINE);
                }
            } else {
                bool next_token_is_comment = false;

                uint16_t indent_length = (uint16_t)lexer->get_column(lexer);

                bool indent = indent_length > VEC_BACK(scanner->indents);
                bool dedent = indent_length < VEC_BACK(scanner->indents);

                if (!next_token_is_comment) {
                    if (skipped_newline) {
                        if (indent) {
                            if (valid_symbols[INDENT]) {
                                VEC_PUSH(scanner->indents, indent_length);
                                QUEUE_PUSH(scanner->tokens, INDENT);
                            }
                        } else if (dedent) {
                            if (valid_symbols[NEWLINE]) {
                                QUEUE_PUSH(scanner->tokens, NEWLINE);
                            }
                        } else {
                            if (valid_symbols[NEWLINE]) {
                                QUEUE_PUSH(scanner->tokens, NEWLINE);
                            }
                        }
                    } else {
                        if (indent) {
                            if (valid_symbols[INDENT]) {
                                VEC_PUSH(scanner->indents, indent_length);
                                QUEUE_PUSH(scanner->tokens, INDENT);
                            }
                        } else if (dedent) {
                            VEC_POP(scanner->indents);
                            while (indent_length < VEC_BACK(scanner->indents)) {
                                VEC_POP(scanner->indents);
                                scanner->queued_dedent_count++;
                            }
                            if (valid_symbols[DEDENT]) {
                                QUEUE_PUSH(scanner->tokens, DEDENT);
                                QUEUE_PUSH(scanner->tokens, NEWLINE);
                            } else {
                                scanner->queued_dedent_count++;
                            }
                        }
                    }
                }
            }
        }
    }

    if (QUEUE_EMPTY(scanner->tokens)) {
        return false;
    }

    lexer->result_symbol = QUEUE_FRONT(scanner->tokens);
    QUEUE_POP(scanner->tokens);
    return true;
}

unsigned tree_sitter_agda_external_scanner_serialize(void *payload,
                                                     char *buffer) {
    Scanner *scanner = (Scanner *)payload;
    uint32_t token_count = scanner->tokens.tail - scanner->tokens.head;
    uint32_t indent_count = scanner->indents.len;
    unsigned header_size = 3 * sizeof(uint32_t);
    if (header_size + (indent_count + token_count) * sizeof(uint16_t) >
        TREE_SITTER_SERIALIZATION_BUFFER_SIZE) {
        return 0;
    }
    unsigned size = 0;
    memcpy(&buffer[size], &scanner->queued_dedent_count, sizeof(uint32_t));
    size += sizeof(uint32_t);
    memcpy(&buffer[size], &indent_count, sizeof(uint32_t));
    size += sizeof(uint32_t);
    memcpy(&buffer[size], &token_count, sizeof(uint32_t));
    size += sizeof(uint32_t);
    memcpy(&buffer[size], scanner->indents.data,
           indent_count * sizeof(uint16_t));
    size += indent_count * sizeof(uint16_t);
    for (uint32_t index = 0; index < token_count; index++) {
        uint16_t token = scanner->tokens.data[
            (scanner->tokens.head + index) % scanner->tokens.cap];
        memcpy(&buffer[size], &token, sizeof(uint16_t));
        size += sizeof(uint16_t);
    }
    return size;
}

void tree_sitter_agda_external_scanner_deserialize(void *payload,
                                                   const char *buffer,
                                                   unsigned length) {
    Scanner *scanner = (Scanner *)payload;

    scanner->queued_dedent_count = 0;
    VEC_CLEAR(scanner->indents);
    QUEUE_CLEAR(scanner->tokens);
    if (length == 0) {
        VEC_PUSH(scanner->indents, 0);
        return;
    }
    uint32_t indent_count, token_count;
    unsigned size = 3 * sizeof(uint32_t);
    assert(length >= size);
    memcpy(&scanner->queued_dedent_count, buffer, sizeof(uint32_t));
    memcpy(&indent_count, buffer + sizeof(uint32_t), sizeof(uint32_t));
    memcpy(&token_count, buffer + 2 * sizeof(uint32_t), sizeof(uint32_t));
    assert(length == size + (indent_count + token_count) * sizeof(uint16_t));
    VEC_GROW(scanner->indents, indent_count);
    scanner->indents.len = indent_count;
    memcpy(scanner->indents.data, &buffer[size], indent_count * sizeof(uint16_t));
    size += indent_count * sizeof(uint16_t);
    for (uint32_t index = 0; index < token_count; index++) {
        uint16_t token;
        memcpy(&token, &buffer[size], sizeof(uint16_t));
        size += sizeof(uint16_t);
        assert(token <= DEDENT);
        QUEUE_PUSH(scanner->tokens, token);
    }

    if (scanner->indents.len == 0) {
        VEC_PUSH(scanner->indents, 0);
        return;
    }

    assert(size == length);
}

void *tree_sitter_agda_external_scanner_create() {
    Scanner *scanner = calloc(1, sizeof(Scanner));
    scanner->indents = indent_vec_new();
    scanner->tokens = token_queue_new();
    tree_sitter_agda_external_scanner_deserialize(scanner, NULL, 0);
    return scanner;
}

void tree_sitter_agda_external_scanner_destroy(void *payload) {
    Scanner *scanner = (Scanner *)payload;
    VEC_FREE(scanner->indents);
    QUEUE_FREE(scanner->tokens);
    free(scanner);
}
