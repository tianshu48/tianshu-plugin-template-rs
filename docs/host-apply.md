# host.apply

[中文](host-apply.zh.md)

When the user clicks one of your toolbox buttons, `on-tool` can call `host.apply(op, args-json)` and edit the open document. A call from `step` returns an empty string. Pipe stepping cannot change the graph. Play verbs such as `play_begin` are not `apply` ops.

`op` is one of the names below. `args-json` is a JSON object.

## op

| op | Example args-json |
|---|---|
| `add_subject` | `{"title":"Alice"}` |
| `add_subjects` | `{"subjects":[{"title":"Alice"},{"title":"Bob"}]}` |
| `add_graph` | `{"title":"Setup"}` |
| `move_nodes` | `{"graphId":"<id>","nodes":["$selected"]}` |
| `set_attr` | `{"subject":"Alice","key":"hp","value":10}` |
| `add_node` | `{"kind":"echo"}` |
| `set_path` | `{"node":"$selected","path":"Alice.hp"}` |
| `set_value` | `{"node":"$selected","value":1}` |
| `set_formula` | `{"node":"$selected","expr":"1+1"}` |
| `set_roll_call` | `{"node":"$selected","attributeKey":"hp","order":"least_first"}` |
| `set_restrict` | `{"node":"$selected","count":1}` |
| `set_anchor` | `{"node":"$selected","title":"act"}` |
| `set_listen` | `{"subject":"Alice","anchor":"act"}` |
| `remove_node` | `{"node":"$selected"}` |
| `remove_edge` | `{"edge":"<edge id>"}` |
| `check` | `{}` |
| `bind_mapped` | `{"node":"$selected","subject":"Alice"}` |
| `bind_resource` | `{"node":"$selected","title":"gold"}` |
| `connect_pipe` | `{}` |
| `connect_data` | `{"source":"$selected","target":"$selected2"}` |
| `batch` | `{"ops":[{"tool":"add_node","arguments":{"kind":"echo"}}]}` |
| `undo` | `{}` |

For `add_node`, `kind` is a short name. In this template that is `"echo"`, the node you declared in `ui.json`. Kinds you can already drag from the left library work the same way, for example `"formula"`. Optional `x`, `y`, and `graphId`.

Empty `connect_pipe` args use the first two selected nodes as source and target. `set_restrict` can also take `filter`, `range`, `attributeKey`, `templateId`, or `sortKey`. `set_attr` `subject` is the subject title on the canvas.

## Graphs

`add_graph` creates a free graph in an open workspace. `title` is the display name. The host sets `folderPath` to `plugin/<your plugin id>`; a path you pass in args is ignored. That folder tree may hold 4 graphs. A fifth call fails with `ok: false`. A single-file document cannot create free graphs.

On success, `value` has `id` and `name`. Later `add_node` calls need that `graphId` or they land on the tab that is open.

`move_nodes` moves nodes onto the graph in `graphId` and keeps node ids. Edges stay only when both ends are in the same `nodes` list. The destination graph must already sit under `plugin/<your plugin id>`. Every listed node must already live on one of those graphs. The time graph, a subject subgraph, and the user's own scenes are not valid destinations. If `nodes` is empty, the host uses the current selection.

Each `add_graph` inside `batch` counts toward the 4. `batch` allows 64 ops and cannot contain another `batch`.

## Placeholders

After wasm returns, the workbench rewrites these strings:

| Write | Becomes |
|---|---|
| `$selected` | First selected node id |
| `$selected2` | Second selected node id |
| `$selection` | Array of selected ids |

## Return value

```json
{"ok":true,"value":{}}
{"ok":false,"error":"…"}
```

An empty string means the call did not run. That is what you get from `step`.

## ui.json

`tools` use the same `op` and `args`. At most 16 tools. `icon` must be an `.svg` next to `ui.json`. `on-tool` looks up the button `id` and calls `apply`.

```json
{
  "id": "place_echo",
  "title": "Echo",
  "icon": "icon.svg",
  "op": "add_node",
  "args": { "kind": "echo" }
}
```

```json
{
  "id": "link",
  "title": "Pipe",
  "icon": "icon.svg",
  "op": "connect_pipe",
  "args": {}
}
```
