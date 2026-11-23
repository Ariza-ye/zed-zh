# i18n

Zed 界面的中文词条。

## 工作方式

上游 Zed 没有本地化框架,界面文字是散落在各处的英文字面量。这里不去改每一条调用,
而是在几个必经的渲染入口做查表替换:

| 入口 | 覆盖范围 |
| --- | --- |
| `ui::Label::new` / `Label::set_text` | 绝大多数标签、面板标题、设置项、右键菜单项 |
| `ui::Button::new` 的 label 参数 | 按钮文字 |
| `zed::app_menus::app_menus` | macOS / Linux / Windows 应用菜单 |
| `command_palette::humanize_action_name` | 命令面板、快捷键提示、按键映射编辑器 |

未收录的文本原样显示英文,不会出现空白或占位符。

## 词条组织

| 文件 | key 形式 | 用途 |
| --- | --- | --- |
| `src/ui.rs` | 原始英文字面量,如 `"Save"` | 按钮、标签、菜单、设置项标题 |
| `src/phrases.rs` | `humanize_action_name` 输出的小写短语,如 `"go to definition"` | 命令面板里的固定短语 |
| `src/words.rs` | 单个单词 | 命令名的逐词兜底,整条短语全部命中才生效 |
| `src/actions.rs` | action 全名,如 `"editor::GoToDefinition"` | 短语和逐词都表达不到位的个别命令 |

## 添加词条

在对应文件的 `match` 里加一行即可,不需要改动调用方:

```rust
"Save All" => "全部保存",
```

找还没翻译的文案:

```sh
script/i18n-missing        # 列出源码中未收录的文案,按出现次数排序
script/i18n-missing --stale  # 列出源码已不再使用的词条
```

## 关闭翻译

```sh
ZED_DISABLE_I18N=1 cargo test
```

跑测试或想用回英文界面时设置该变量。测试里大量断言英文文案,汉化开启时会失败。

## 已知未覆盖

- `crates/settings_ui/src/page_data.rs` 里 478 条设置项描述(标题已覆盖)
- 通知、错误提示等运行时拼接的消息
- `gpui` 输入框的 placeholder
- 直接写成 `.child("文本")` 的少量元素
