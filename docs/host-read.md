# host.read

[中文](host-read.zh.md)

In `step`, call `host.read(kind, a, b)` for values on the current pipe and document. Empty string means nothing is there.

## kind

| kind | a | b | Meaning |
|---|---|---|---|
| `in` | inbound handle, usually `"in"` | `""` | Current inbound pipe value |
| `param` | param key, `ui.json` `fields[].id` | `""` | Value filled on this node |
| `attr` | subject id | attribute name | Subject attribute |
| `info` | key in the table below | see below | Read-only environment |

For `info`, leave `b` empty except `graph.sha256`.

## info

| a | b |
|---|---|
| `plugin.id` | `""` |
| `plugin.kind` | `""` |
| `plugin.abi` | `""` |
| `doc.version` | `""` |
| `doc.graphs` | `""` |
| `graph.id` | `""` |
| `graph.sha256` | `""` for the current graph, otherwise a graph id |
| `node.id` | `""` |
| `play.seed` | `""` |

## Return value

The host returns an IR JSON string:

```json
{"type":"String","value":"ping"}
{"type":"Number","value":1}
```

The return value of `step` is plain text for the log when the node declares no Play caps. With `output` / `write` it may be JSON (`log` / `out` / `set`). See [What an extension node is in Play](play-node.md).
