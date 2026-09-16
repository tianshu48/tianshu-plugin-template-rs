# 扩展槽在 Play 里是什么

[English](play-node.md)

扩展槽画在图上时，看起来和其他逻辑节点一样：有管道口，可以填字段。以前宿主对所有扩展槽用同一套跑法：管道踩到它，调用 `step`，把返回文本写进 Play 日志，下游拉不到值，也不能改主体属性。图上有格子，管道语义里它更像探针。

现在每个 kind 在 `ui.json` 里声明自己在 Play 里做什么。宿主按声明执行，guest 仍然不能拿文档、不能调 `play_*`。

## 声明

写在 `nodes[]` 上，和 `kind` / `title` 同一层。省略等于 `none`。

| 字段 | 取值 | 宿主做什么 |
|---|---|---|
| `output` | `none`（默认）、`data` | `data`：这一格有可拉的数据输出。下游数据边可以接到它。 |
| `write` | `none`（默认）、`attr` | `attr`：管道步进时，宿主按 `step` 返回里的 `set` 改主体属性。 |

放上画布时，这两项会抄进节点线格式（`output` / `write`）。没装插件打开这份文档，字段还在；没有运行时则步进仍会失败。

口的形状暂时固定：管道 `in` + `pipe_out`，和记录节点相同。数据输出走节点自身，不另开句柄名。

## `step` 返回什么

`output` 和 `write` 都是 `none` 时，返回普通文本即可，和以前一样，只出现在日志里。

需要数据或写属性时，返回一段 JSON 对象：

```json
{
  "log": "结算完毕",
  "out": {"type":"Number","value":12},
  "set": [{"subject":"小明","attr":"hp","value":{"type":"Number","value":7}}]
}
```

`log` 仍是管道上给人看的句子。`out` 是 IR 值（`String` / `Number` / `List`）。`set` 是一次步进里要改的属性；没声明 `write: attr` 却带了 `set`，步进失败。

只声明了 `output: data`、返回仍是普通字符串时，整段字符串当作 `String` 值，方便回声这类节点。

也可以直接返回一个 IR 值，例如 `{"type":"Number","value":1}`。宿主把它既当日志文本，也当 `out`。

`host.apply` 在 `step` 里仍然是空字符串。改属性只走上面的 `set`，由宿主写。

## 模板里的例子

语言脚手架（`tianshu-plugin-template-ts` / `-rs`）带三个 kind：

| kind | 声明 | `step` 做什么 |
|---|---|---|
| `echo` | 默认 | 把入边或 `text` 写成日志句子 |
| `sum` | `output: data` | 入边数字加上字段 `add`，返回 `out` 里的 Number。`add` 默认 `0`，加法在 `step` 里做。 |
| `set_attr` | `write: attr` | 用字段里的主体名、属性名、值组一条 `set`，管道踩到时宿主去写 |

源码在 TS 的 `src/play.ts`、Rust 的 `src/play.rs`。

## 和内建节点的差别

公式在拉值时求值，不一定要先被管道踩到。扩展槽在声明了 `output: data` 之后，拉值会再调一次 `step`（只取 `out`，不执行 `set`）。所以纯计算可以当数据源；改 HP 仍然只发生在管道步进。

校验条还不会读这些声明。非法的 `set` 或空的 `out` 出现在 Play 时，而不是画布红字。
