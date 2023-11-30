#pragma once

#include "extism-pdk.h"

#define HELIX_HOST_FUNC(r, f, ...)                                             \
  IMPORT("helix:editor/env", #f) extern r hx_editor_##f(__VA_ARGS__)

typedef uint64_t HelixSelection;
typedef uint64_t HelixPosition;

HELIX_HOST_FUNC(void, set_status, ExtismPointer);
HELIX_HOST_FUNC(void, clear_status);
HELIX_HOST_FUNC(void, set_path, ExtismPointer);
HELIX_HOST_FUNC(ExtismPointer, text, HelixPosition, HelixPosition);
HELIX_HOST_FUNC(void, selection_insert_text_before, ExtismPointer);
HELIX_HOST_FUNC(void, selection_insert_text_after, ExtismPointer);
HELIX_HOST_FUNC(void, selection_replace_text, ExtismPointer);
HELIX_HOST_FUNC(HelixSelection, selection_add, HelixPosition, HelixPosition);
HELIX_HOST_FUNC(void, selection_reset);
HELIX_HOST_FUNC(HelixPosition, selection_begin, HelixSelection);
HELIX_HOST_FUNC(HelixPosition, selection_end, HelixSelection);
HELIX_HOST_FUNC(uint64_t, selection_count);
HELIX_HOST_FUNC(void, vsplit);
HELIX_HOST_FUNC(void, hsplit);
HELIX_HOST_FUNC(void, focus_next);
HELIX_HOST_FUNC(void, focus_prev);
HELIX_HOST_FUNC(void, close);
HELIX_HOST_FUNC(void, undo);
HELIX_HOST_FUNC(void, redo);
HELIX_HOST_FUNC(void, save, ExtismPointer);
HELIX_HOST_FUNC(void, open, ExtismPointer);
