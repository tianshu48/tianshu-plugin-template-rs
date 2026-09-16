# Template

[中文](#中文)

Sample community plugin. Three nodes:

| Node | What it does |
|---|---|
| Echo | Writes inbound pipe or the Text field to the Play log |
| Sum | Adds inbound number to field Add; downstream can pull the result |
| Set attr | On a pipe step, writes Subject.Attr to Value |

Toolbox button Echo places an Echo node.

## 中文

社区插件示例。三个节点：Echo 把入边或「Text」写进运行日志；Sum 把入边数字和「Add」相加，下游可以拉结果；Set attr 在管道踩到时改主体属性。工具栏「Echo」会放一个 Echo 节点。
