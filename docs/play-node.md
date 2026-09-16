# What an extension node is in Play

[中文](play-node.zh.md)

On the canvas an extension looks like any other logic node: pipe ports and fields. The host used to run every extension the same way. The pipe called `step`, the return text went into the Play log, nothing downstream could pull a value, and the node could not change subject attributes. It sat on the graph. In the pipe it behaved like a probe.

Each kind now declares what it does in Play, in `ui.json`. The host honours that declaration. The guest still does not hold the document and still cannot call `play_*`.

## Declaration

These fields sit on `nodes[]` next to `kind` and `title`. Omit them for `none`.

| Field | Values | Host |
|---|---|---|
| `output` | `none` (default), `data` | `data`: this cell has a pullable data output. Downstream data edges can use it. |
| `write` | `none` (default), `attr` | `attr`: on a pipe step, the host applies `set` from the `step` return to subject attributes. |

When you place the node, those two fields are copied onto the wire (`output` / `write`). They survive in a document opened without the plugin. Stepping still fails if the runtime is missing.

Port shape is still fixed: pipe `in` + `pipe_out`, same as record. A data output is the node itself, not a named extra handle.

## What `step` returns

If both caps are `none`, return plain text, as before. It only shows in the log.

If you need data or attribute writes, return a JSON object:

```json
{
  "log": "settled",
  "out": {"type":"Number","value":12},
  "set": [{"subject":"Hero","attr":"hp","value":{"type":"Number","value":7}}]
}
```

`log` is the sentence on the pipe. `out` is an IR value (`String` / `Number` / `List`). `set` is the attributes to change on that step. Returning `set` without `write: attr` fails the step.

If you declared `output: data` and still return a plain string, the whole string becomes a `String` value. That is enough for echo-style nodes.

You may also return an IR value by itself, such as `{"type":"Number","value":1}`. The host uses it as both log text and `out`.

`host.apply` is still empty during `step`. Attribute writes go through `set` and the host.

## Examples in the templates

The language scaffolds (`tianshu-plugin-template-ts` / `-rs`) ship three kinds:

| kind | Declaration | What `step` does |
|---|---|---|
| `echo` | defaults | Writes inbound or `text` as a log line |
| `sum` | `output: data` | Adds inbound number to field `add`, returns that Number in `out`. `add` defaults to `0`; the addition happens in `step`. |
| `set_attr` | `write: attr` | Builds a `set` from subject, attr, and value fields. The host writes on a pipe step. |

Code: `src/play.ts` (TS) and `src/play.rs` (Rust).

## Versus built-in nodes

A formula evaluates when something pulls it; the pipe does not have to visit it first. An extension with `output: data` runs `step` again on pull (for `out` only, never `set`). Pure calculation can be a data source. HP changes still happen only on a pipe step.

The issue list does not read these declarations yet. A bad `set` or a missing `out` shows up in Play, not as a red canvas diagnostic.
