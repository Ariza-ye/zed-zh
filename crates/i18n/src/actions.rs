//! action 全名精确词条。
//!
//! 用于精确短语和逐词兜底都无法给出地道表达的命令。

pub(crate) fn lookup(action_name: &str) -> Option<&'static str> {
    Some(match action_name {
        "workspace::NewFileSplit" => "工作区: 新建文件并拆分",
        "workspace::ToggleZoom" => "工作区: 切换面板最大化",
        "editor::ToggleSplitDiff" => "编辑器: 切换并排差异视图",
        _ => return None,
    })
}
