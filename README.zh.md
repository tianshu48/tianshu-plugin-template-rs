# Rust 插件模板

[English](README.md)

这是一个 Rust 插件，可以装进天枢。`scripts/pack.sh`（Windows 上是 `scripts\pack.bat`）在 `dist/` 放下 `plugin.wasm`、`plugin.json`、`ui.json`、`USER.md`、`icon.svg`。

## 先跑通

```bash
rustup target add wasm32-wasip2
cargo test
./scripts/pack.sh
```

Windows cmd：

```bat
scripts\pack.bat
```

`cargo test` 测三个 kind，并核对 `id()` 和 `plugin.json` 的 `id` 一致。打包时会加上 `--features guest` 再编 wasm。

打开天枢：设置 → 插件 → 添加本地插件，选这个仓库的 `dist/`。应用会拷一份到自己的插件目录。

## 源码对着看

| 文件 | 作用 |
|---|---|
| `plugin.json` | id、标题、版本。`id()` 读这份。`readme` 指向设置页说明（模板是 `USER.md`） |
| `ui.json` | 三个节点：`echo`、`sum`（`output: data`）、`set_attr`（`write: attr`），外加工具栏「Echo」 |
| `src/lib.rs` | WIT 导出。`step` 按 `kind` 分发 |
| `src/echo.rs` | `echo`：日志 |
| `src/play.rs` | `sum` 算出 `out`；`set_attr` 组好 `set` |
| `docs/` | `host.read`、`host.apply`、Play 扩展槽 |

## 改成你自己的

1. 改 `plugin.json` 的 `id`、`title`、`description`、`version`。`abi` 保持 `3`。
2. 在 `ui.json` 的 `nodes` 里写节点。每个 `kind` 要在 `src/lib.rs` 的 `step` 里有分支。下游要拉值时写 `output: "data"`；管道步进里改属性时写 `write: "attr"`。细则：[扩展槽在 Play 里是什么](docs/play-node.zh.md)。
3. 步进用 `host.read`，工具栏用 `host.apply`。参数见 [host.read](docs/host-read.zh.md) 和 [host.apply](docs/host-apply.zh.md)。
4. 要工具栏按钮时，在 `ui.json` 加 `tools`。点按钮会跑 `on_tool`。
5. 发给别人之前，改 `USER.md`（以及 `USER.zh.md`）。不要把这篇脚手架步骤发给用户。
6. 工具栏若要自己的图：在 `on_tool` 里 `add_graph`，记下返回的 `id`，再 `add_node` 带 `graphId`。图落在 `plugin/<你的id>` 下，最多 4 张。细则见 [host.apply](docs/host-apply.zh.md) 的「图」。

图上完整 kind 是 `p:<你的id>:<kind>`。`abi-version()` 为 `3`。`ui-json()` 返回的就是 `ui.json`。

## 上架

打一个和 `plugin.json` 里 `version` 对应的 tag（`0.1.0` 打 `v0.1.0`）。工作流把 `.tsz` 插件包挂到 GitHub Release，并写出 `proposals/<id>/<version>.json`。

插件包本地签名：先在这台电脑上登录天枢，打包后跑 `node scripts/sign.mjs --user-id <天枢用户id>`。Windows 默认是 `%APPDATA%\com.tianshu.desktop\identity\user.sk.hex`。不在默认位置就用 `--sk` 指定路径。签过名的包在 `dist/`。

有 `INDEX_PR_TOKEN` 时，工作流会对 [tianshu48/tianshu-plugin-index](https://github.com/tianshu48/tianshu-plugin-index) 的 `main` 开 PR。没有 token 就 fork 索引仓，把那份 json 放进去再开。

`example.community.template` 和以 `tianshu` 开头的 id 会被拒绝。审核通过后，没签名的包还会再要你签一份；已经签过的直接合并。

## plugin.json

| 字段 | 必填 | 含义 |
|---|---|---|
| `id` | 是 | 插件 id，字母开头，可含 `.` `_` `-` |
| `title` | 是 | 显示名 |
| `version` | 是 | 版本字符串 |
| `abi` | 是 | `3` |
| `description` | 否 | 说明 |
| `readme` | 否 | 同目录说明文件名（设置页） |
| `icon` | 否 | 同目录图标文件名 |

## ui.json

根对象是 `nodes`，可加 `tools`。节点：`kind`、`title`、`category` 必填；`categoryLabel` 可选；`fields` 为参数列表；可选 `output`（`data` 则可拉）和 `write`（`attr` 则可改属性）。field：`id`、`label`、`type`（模板用 `"string"`）、可选 `default`。`echo` 两项都不写，只打日志；`sum` / `set_attr` 见 [扩展槽在 Play 里是什么](docs/play-node.zh.md)。

`tools`：`id`、`title`、`icon`（同目录 `.svg`）、`op`、`args`。完整动词表见 [host.apply](docs/host-apply.zh.md)。`args` 里的 `$selected` / `$selected2` 会换成当前选中的节点。图标把工具栏文字挤着了，文字会藏起来。

入边 `read("in", "in", "")`，字段 `read("param", "<id>", "")`，返回值先走 `ir_text`。工具栏点击走 `on_tool`，里面调 `host::apply`。
