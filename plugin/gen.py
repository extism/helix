import re

def fix_rust_type(t):
    t = t.strip()
    if t == "extism::PTR" or t == "PTR":
        return "ExtismPointer"
    elif t == "extism::ValType::I64" or t == "ValType::I64":
        return "u64"
    return t


def fix_c_type(t):
    t = t.strip()
    if t == "extism::PTR" or t == "PTR":
        return "ExtismPointer"
    elif t == "extism::ValType::I64" or t == "ValType::I64":
        return "uint64_t"
    return t

with open("../helix-term/src/commands/plugin.rs") as f:
    text = f.read()
r = '"helix:editor/env",[ \n]*"([A-z_]*)",[ \n]*\[(.*)\],[ \n]*\[(.*)\]'
x = re.findall(r, text)

rust_output = '''
// Generated with gen.py

pub type ExtismPointer = u64;

#[link(wasm_import_module = "helix:editor/env")]
extern "C" {
'''

c_output = '''
// Generated with gen.py
#pragma once

#include "extism-pdk.h"

#define HELIX_HOST_FUNC(r, f, ...) \\
  IMPORT("helix:editor/env", #f) extern r hx_editor_##f(__VA_ARGS__)

'''

for (name, params, results) in x:
    params = list(filter(lambda x: len(x) > 0, params.split(',')))
    results = list(filter(lambda x: len(x) > 0, results.split(',')))
    nresults = len(results)
    nparams = len(params)
    rust_params = ', '.join(fix_rust_type(x) for x in params)
    rust_results = ', '.join(fix_rust_type(x) for x in results)
    c_params = ', '.join(fix_c_type(x) for x in params)
    if nparams > 0:
        c_params = ", " + c_params
    c_results = ', '.join(fix_c_type(x) for x in results)
    if nresults > 1:
        rust_results = '(' + rust_results + ')'
    else:
        c_results = "void"
    rust_output += f"  pub fn {name}({rust_params}) -> {rust_results};\n"
    c_output += f"HELIX_HOST_FUNC({c_results}, {name}{c_params});\n"

rust_output += "}"

with open("src/bindings.rs", 'w') as f:
    f.write(rust_output)
with open("helix-plugin.h", 'w') as f:
    f.write(c_output)
