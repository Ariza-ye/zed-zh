# i18n

Zed 界面的中文词条。

## 工作方式

上游 Zed 没有本地化框架,界面文字是散落在各处的英文字面量。这里不去改每一条调用,
而是在几个必经的渲染入口做查表替换:

| 入口 | 覆盖范围 |
| --- | --- |
| `ui::Label::new` / `Label::set_text` | 绝大多数标签、面板标题、设置项、右键菜单项 |
| `ui::Button::new` 的 label 参数 | 按钮文字 |
| `ui::Tooltip` / `ui::Headline` | 工具提示和页面大标题 |
| `Editor::set_placeholder_text` | 编辑器和选择器输入框的占位提示 |
| `Window::prompt` | 应用内及系统原生确认对话框 |
| `zed::app_menus::app_menus` | macOS / Linux / Windows 应用菜单 |
| `command_palette::humanize_action_name` | 命令面板、快捷键提示、按键映射编辑器 |

未收录的文本原样显示英文,不会出现空白或占位符。

## 词条组织

| 文件 | key 形式 | 用途 |
| --- | --- | --- |
| `src/ui.rs` | 原始英文字面量,如 `"Save"` | 按钮、标签、菜单、设置项标题 |
| `src/settings.rs` | 设置页原始英文说明 | 设置项说明 |
| `src/phrases.rs` | `humanize_action_name` 输出的小写短语,如 `"go to definition"` | 命令面板里的固定短语 |
| `src/words.rs` | 单个单词 | 命令名的逐词兜底,整条短语全部命中才生效 |
| `src/actions.rs` | action 全名,如 `"editor::GoToDefinition"` | 短语和逐词都表达不到位的个别命令 |

## 添加词条

在对应文件的词表中添加原文和译文:

```rust
"Save All" => "全部保存",
```

找还没翻译的文案:

```sh
script/i18n-missing             # 列出扫描到的未收录文案
script/i18n-missing --settings  # 检查设置页固定标题和说明
script/i18n-missing --stale     # 列出扫描器未找到引用的词条
```

## 关闭翻译

```sh
ZED_DISABLE_I18N=1 cargo test
```

跑测试或想用回英文界面时设置该变量。测试里大量断言英文文案,汉化开启时会失败。

## 已知未覆盖

- 部分通知和错误由运行时数据拼接，需要先翻译固定模板，再插入动态内容。
- 部分直接渲染的文字以及扩展、语言服务器和 AI 提供的内容。
