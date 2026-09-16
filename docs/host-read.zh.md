# host.read

[English](host-read.md)

`step` 里用 `host.read(kind, a, b)` 向宿主要当前管道和文档上的值。没有数据时返回空字符串。

## kind

| kind | a | b | 含义 |
|---|---|---|---|
| `in` | 入边 handle，常用 `"in"` | `""` | 管道入边当前值 |
| `param` | 参数键，对应 `ui.json` 里 `fields[].id` | `""` | 这个节点上填的参数 |
| `attr` | 主体 id | 属性名 | 主体上的属性 |
| `info` | 下表里的键 | 见下 | 只读环境 |

`info` 里除 `graph.sha256` 以外，`b` 留空。

## info

| a | b |
|---|---|
| `plugin.id` | `""` |
| `plugin.kind` | `""` |
| `plugin.abi` | `""` |
| `doc.version` | `""` |
| `doc.graphs` | `""` |
| `graph.id` | `""` |
| `graph.sha256` | `""` 表示当前图，否则填图 id |
| `node.id` | `""` |
| `play.seed` | `""` |

## 返回值

宿主给的是 IR JSON 字符串：

```json
{"type":"String","value":"ping"}
{"type":"Number","value":1}
```

`step` 自己的返回值：未声明 Play 能力时是给人看的普通文本。声明了 `output` / `write` 时可以是 JSON（`log` / `out` / `set`），见 [扩展槽在 Play 里是什么](play-node.zh.md)。
