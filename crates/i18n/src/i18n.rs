//! 界面文本汉化。
//!
//! 以英文原文为 key 查表,未命中时回退英文,保证任何缺失的翻译都不会
//! 让界面出现空白。词条按用途分模块维护,新增只需在对应模块里加一行。
//!
//! 设置环境变量 `ZED_DISABLE_I18N=1` 可以关掉全部翻译(跑依赖英文文案的测试时需要)。

use std::sync::OnceLock;

mod actions;
mod phrases;
mod ui;
mod words;

fn enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("ZED_DISABLE_I18N").is_none())
}

/// 翻译界面文本,未命中返回 `None`。适合需要避免额外分配的渲染路径。
pub fn translate(text: &str) -> Option<&'static str> {
    if !enabled() {
        return None;
    }
    ui::lookup(text)
}

/// 翻译界面文本(按钮、标签、提示等)。未命中返回原文。
pub fn tr(text: &str) -> &str {
    translate(text).unwrap_or(text)
}

/// 翻译 action 全名,例如 `editor::GoToDefinition`。
///
/// 返回 `None` 表示没有精确词条,调用方应回退到英文转换。
pub fn translate_action(action_name: &str) -> Option<&'static str> {
    if !enabled() {
        return None;
    }
    actions::lookup(action_name)
}

/// 翻译已经转换成可读形式的 action 名称,例如 `editor: go to definition`。
///
/// 只有在整条短语的每个词都能翻译时才返回结果:夹生中英混排比保留英文更糟。
pub fn translate_phrase(humanized: &str) -> Option<String> {
    if !enabled() {
        return None;
    }

    let (namespace, body) = match humanized.split_once(": ") {
        Some((namespace, body)) => (Some(namespace), body),
        None => (None, humanized),
    };

    let body = phrases::lookup(body)
        .map(str::to_string)
        .or_else(|| words::translate(body))?;

    Some(match namespace {
        Some(namespace) => format!("{}: {body}", words::namespace(namespace)?),
        None => body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn untranslated_text_falls_back_to_english() {
        assert_eq!(tr("zzz_not_a_real_string_zzz"), "zzz_not_a_real_string_zzz");
    }

    #[test]
    fn action_names_translate() {
        assert_eq!(
            translate_phrase("editor: go to definition").as_deref(),
            Some("编辑器: 转到定义")
        );
        assert_eq!(
            translate_action("editor::ToggleSplitDiff"),
            Some("编辑器: 切换并排差异视图")
        );
        assert_eq!(translate_action("editor::GoToDefinition"), None);
    }

    #[test]
    fn phrases_do_not_mix_languages() {
        // 含未收录词汇时整条保留英文,不产出中英混排。
        assert_eq!(
            translate_phrase("editor: flibbertigibbet the widget"),
            None
        );
    }
}
