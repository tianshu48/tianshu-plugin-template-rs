# host.apply

[English](host-apply.md)

点工具栏上你插件的按钮时，guest 的 `on-tool` 可以调 `host.apply(op, args-json)`，改当前打开的文档。`step` 里调用会得到空字符串，管道步进改不了图。`play_begin` 这类 Play 动词不是 `apply` 的 op。

`op` 用下表的名字。`args-json` 是一个 JSON 对象。

## op

| op | args-json 示例 |
|---|---|
| `add_subject` | `{"title":"小明"}` |
| `add_subjects` | `{"subjects":[{"title":"小明"},{"title":"小红"}]}` |
| `add_graph` | `{"title":"布置"}` |
| `move_nodes` | `{"graphId":"<id>","nodes":["$selected"]}` |
| `set_attr` | `{"subject":"小明","key":"hp","value":10}` |
| `add_node` | `{"kind":"echo"}` |
| `set_path` | `{"node":"$selected","path":"小明.hp"}` |
| `set_value` | `{"node":"$selected","value":1}` |
| `set_formula` | `{"node":"$selected","expr":"1+1"}` |
| `set_roll_call` | `{"node":"$selected","attributeKey":"hp","order":"least_first"}` |
| `set_restrict` | `{"node":"$selected","count":1}` |
| `set_anchor` | `{"node":"$selected","title":"行动"}` |
| `set_listen` | `{"subject":"小明","anchor":"行动"}` |
| `remove_node` | `{"node":"$selected"}` |
| `remove_edge` | `{"edge":"<边的id>"}` |
| `check` | `{}` |
| `bind_mapped` | `{"node":"$selected","subject":"小明"}` |
| `bind_resource` | `{"node":"$selected","title":"金币"}` |
| `connect_pipe` | `{}` |
| `connect_data` | `{"source":"$selected","target":"$selected2"}` |
| `batch` | `{"ops":[{"tool":"add_node","arguments":{"kind":"echo"}}]}` |
| `undo` | `{}` |

`add_node` 的 `kind` 写短名。模板里就是 `"echo"`，对应你在 `ui.json` 声明的那个节点。左侧库里已经能拖出来的节点也可以这样放，例如 `"formula"`。可选 `x`、`y`、`graphId`。

`connect_pipe` 的 `args` 为空时，用当前选中的前两个节点当起点和终点。`set_restrict` 还可以带 `filter`、`range`、`attributeKey`、`templateId`、`sortKey`。`set_attr` 的 `subject` 是主体在图上的名字。

## 图

`add_graph` 在已打开的工作空间里建一张自由图。`title` 是显示名。宿主会把 `folderPath` 写成 `plugin/<你的插件id>`，args 里带的路径不会生效。这个目录树里最多 4 张图，第 5 次会 `ok: false`。单文件文档建不了自由图。

成功时 `value` 里有 `id` 和 `name`。之后 `add_node` 要带这个 `graphId`，否则会落到当前打开的标签上。

`move_nodes` 把节点挪到 `graphId` 那张图，节点 id 不变。只有两端都在这次 `nodes` 列表里的边会一起走。目标图必须已经在 `plugin/<你的插件id>` 下面，被挪的节点也必须已经在你的图上。时间图、主体子图、用户自己的场景都不能当目标。`nodes` 为空时用当前选区。

`batch` 里每张 `add_graph` 都算进那 4 张。`batch` 最多 64 步，不能再套一层 `batch`。

## 占位符

wasm 返回之后，工作台会替换这些字符串：

| 写 | 换成 |
|---|---|
| `$selected` | 当前选中的第一个节点 id |
| `$selected2` | 第二个 |
| `$selection` | 选中 id 的数组 |

## 返回值

```json
{"ok":true,"value":{}}
{"ok":false,"error":"…"}
```

空字符串表示这次没有执行，常见原因是在 `step` 里调用。

## 写在 ui.json 里

`tools` 里的 `op` / `args` 和这里是同一套。最多 16 个按钮。`icon` 必须是同目录 `.svg`。`on-tool` 按按钮的 `id` 找到那一项，再 `apply`。

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
