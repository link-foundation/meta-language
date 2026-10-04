/* Replays traced issue #195 Lean runtime cases through the tree-sitter C
 * runtime the Rust adapter links (0.25.10), the way parse_incremental does:
 * fresh parse, edit, reparse with the old tree, fresh reparse when the result
 * has errors, and a walk that reads every node's type. Build with
 * -fsanitize=address,undefined to catch memory errors the Windows run hits. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include "tree_sitter/api.h"

const TSLanguage *tree_sitter_lean(void);

static unsigned long nodes = 0;

/* Rust's Node::kind() unwraps CStr::to_str, which fails on invalid UTF-8. */
static int valid_utf8(const char *text) {
  const unsigned char *c = (const unsigned char *)text;
  while (*c) {
    int extra = *c < 0x80 ? 0 : (*c >> 5) == 6 ? 1 : (*c >> 4) == 14 ? 2 : (*c >> 3) == 30 ? 3 : -1;
    if (extra < 0) return 0;
    c++;
    while (extra--) { if ((*c & 0xC0) != 0x80) return 0; c++; }
  }
  return 1;
}

static void walk(TSTree *tree, const TSLanguage *language) {
  TSTreeCursor cursor = ts_tree_cursor_new(ts_tree_root_node(tree));
  uint32_t names = ts_language_symbol_count(language);
  for (;;) {
    TSNode node = ts_tree_cursor_current_node(&cursor);
    TSSymbol symbol = ts_node_symbol(node);
    const char *type = ts_node_type(node);
    if (type == NULL || (symbol >= names && symbol != (TSSymbol)-1)) {
      fprintf(stderr, "invalid symbol %u (%s) at %u..%u\n", symbol, type ? type : "(null)",
              ts_node_start_byte(node), ts_node_end_byte(node));
      abort();
    }
    if (!valid_utf8(type)) {
      fprintf(stderr, "type name of symbol %u is not UTF-8\n", symbol);
      abort();
    }
    (void)ts_node_grammar_type(node);
    (void)ts_tree_cursor_current_field_name(&cursor);
    nodes++;
    if (ts_tree_cursor_goto_first_child(&cursor)) continue;
    while (!ts_tree_cursor_goto_next_sibling(&cursor)) {
      if (!ts_tree_cursor_goto_parent(&cursor)) { ts_tree_cursor_delete(&cursor); return; }
    }
  }
}

static TSPoint point_at(const char *text, uint32_t byte) {
  TSPoint point = {0, 0};
  for (uint32_t i = 0; i < byte; i++) {
    if (text[i] == '\n') { point.row++; point.column = 0; } else point.column++;
  }
  return point;
}

static uint32_t read_u32(FILE *file) {
  uint32_t value;
  if (fread(&value, 4, 1, file) != 1) exit(0);
  return value;
}

static char *read_text(FILE *file, uint32_t length) {
  char *text = malloc(length + 1);
  if (length && fread(text, 1, length, file) != length) { fprintf(stderr, "short read\n"); exit(2); }
  text[length] = 0;
  return text;
}

int main(int argc, char **argv) {
  FILE *file = fopen(argv[1], "rb");
  const TSLanguage *language = tree_sitter_lean();
  TSParser *parser = ts_parser_new();
  ts_parser_set_language(parser, language);
  unsigned long records = 0;
  for (;;) {
    uint32_t kind = read_u32(file);
    uint32_t length = read_u32(file);
    char *text = read_text(file, length);
    records++;
    if (kind == 0) {
      TSTree *tree = ts_parser_parse_string(parser, NULL, text, length);
      walk(tree, language);
      ts_tree_delete(tree);
    } else {
      uint32_t start = read_u32(file), end = read_u32(file);
      uint32_t replacement_length = read_u32(file);
      char *replacement = read_text(file, replacement_length);
      uint32_t edited_length = length - (end - start) + replacement_length;
      char *edited = malloc(edited_length + 1);
      memcpy(edited, text, start);
      memcpy(edited + start, replacement, replacement_length);
      memcpy(edited + start + replacement_length, text + end, length - end);
      edited[edited_length] = 0;
      TSTree *old_tree = ts_parser_parse_string(parser, NULL, text, length);
      TSPoint start_point = point_at(text, start);
      TSPoint new_end = start_point;
      for (uint32_t i = 0; i < replacement_length; i++) {
        if (replacement[i] == '\n') { new_end.row++; new_end.column = 0; } else new_end.column++;
      }
      TSInputEdit edit = {start, end, start + replacement_length, start_point, point_at(text, end), new_end};
      ts_tree_edit(old_tree, &edit);
      TSTree *tree = ts_parser_parse_string(parser, old_tree, edited, edited_length);
      walk(tree, language);
      if (ts_node_has_error(ts_tree_root_node(tree))) {
        ts_tree_delete(tree);
        tree = ts_parser_parse_string(parser, NULL, edited, edited_length);
        walk(tree, language);
      }
      ts_tree_delete(tree);
      ts_tree_delete(old_tree);
      free(replacement);
      free(edited);
    }
    free(text);
    if (records % 200 == 0) fprintf(stderr, "%lu records, %lu nodes\n", records, nodes);
  }
}
