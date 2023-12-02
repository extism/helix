
// Generated with gen.py
#pragma once

#include "extism-pdk.h"

#define HELIX_HOST_FUNC(r, f, ...) \
  IMPORT("helix:editor/env", #f) extern r hx_editor_##f(__VA_ARGS__)

HELIX_HOST_FUNC(, save, ExtismPointer);
HELIX_HOST_FUNC(, set_status, ExtismPointer);
HELIX_HOST_FUNC(, clear_status);
HELIX_HOST_FUNC(, set_path, ExtismPointer);
HELIX_HOST_FUNC(, undo);
HELIX_HOST_FUNC(, redo);
HELIX_HOST_FUNC(, open, ExtismPointer);
HELIX_HOST_FUNC(, close);
HELIX_HOST_FUNC(, vsplit);
HELIX_HOST_FUNC(, hsplit);
HELIX_HOST_FUNC(, focus_next);
HELIX_HOST_FUNC(, focus_prev);
HELIX_HOST_FUNC(, selection_insert_text_after, ExtismPointer);
HELIX_HOST_FUNC(, selection_insert_text_before, ExtismPointer);
HELIX_HOST_FUNC(, selection_replace_text, ExtismPointer);
HELIX_HOST_FUNC(void, selection_add, uint64_t, uint64_t);
HELIX_HOST_FUNC(, selection_reset);
HELIX_HOST_FUNC(void, selection_count);
HELIX_HOST_FUNC(void, selection_begin, uint64_t);
HELIX_HOST_FUNC(void, selection_end, uint64_t);
HELIX_HOST_FUNC(void, text, uint64_t, uint64_t);
