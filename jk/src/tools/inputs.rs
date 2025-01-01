use std::ffi::OsStr;
use std::fs;

use anyhow::{Error, Result};
use inquire::{
    ui::{Color, RenderConfig, Styled},
    Editor,
};

pub fn pr_comment(template_fp: String) -> Result<String> {
    let template = fs::read_to_string(template_fp)?;
    Editor::new("Commit/PR message:")
        .with_editor_command(OsStr::new("nvim"))
        .with_predefined_text(&template)
        .with_formatter(&|submission| {
            let char_count = submission.chars().count();
            if char_count == 0 {
                String::from("<skipped>")
            } else if char_count <= 20 {
                submission.into()
            } else {
                let mut substr: String = submission.chars().take(17).collect();
                substr.push_str("...");
                substr
            }
        })
        .with_render_config(description_render_config())
        .prompt()
        .map_err(Error::from)
}

fn description_render_config() -> RenderConfig<'static> {
    RenderConfig::default()
        .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(Color::DarkYellow))
}
