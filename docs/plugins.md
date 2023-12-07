# Plugins

WebAssembly plugins have been implemented using [Extism](https://github.com/extism/extism).

## Configuration

A plugin can be called by path or added to the config file. To add a plugin using the config file you can embed an Extism manifest directly:

```toml
[plugins.testing]
functions = ["vsplit-sel", "open-sel"]
[[plugins.testing.wasm]]
path = "/path/to/plugin.wasm"
```

The `functions` field, if set, limits the functions that can be called for a particular plugin, they also provide some additional autocompletion when calling plugins.

## Calling a plugin

The `vsplit-sel` function of the `testing` plugin can be called be executing the following command:

```
:plugin testing:vsplit-sel
```

## Shortcuts

It's also possible to assign shortcuts keys to plugins:

```toml
[keys.normal."+"]
v = ":plugin testing:vsplit-sel"
o = ":plugin testing:open-sel"
```
