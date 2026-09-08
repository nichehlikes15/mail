use serde::Deserialize;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub inbox_background: u32,
    pub inbox_header_border: u32,
    pub inbox_border: u32,
    pub inbox_header_text: u32,
    pub inbox_text: u32,
    pub sidebar_background: u32,
    pub sidebar_border: u32,
    pub sidebar_header_text: u32,
    pub sidebar_tree_border: u32,
    pub sidebar_text: u32,
    pub sidebar_selected_background: u32,
    pub sidebar_button_background: u32,
    pub topbar_background: u32,
    pub topbar_active_text: u32,
    pub topbar_active_background: u32,
    pub topbar_inactive_text: u32,
    pub topbar_border: u32,
    pub topbar_hover_background: u32,
    pub topbar_hover_text: u32,
}

#[derive(Debug, Deserialize)]
struct ThemeFile {
    style: Vec<ThemeColors>,
}

#[derive(Debug, Deserialize)]
struct ThemeColors {
    #[serde(rename = "inbox-background")]
    inbox_background: String,
    #[serde(rename = "inbox-header-border")]
    inbox_header_border: String,
    #[serde(rename = "inbox-border")]
    inbox_border: String,
    #[serde(rename = "inbox-header-text")]
    inbox_header_text: String,
    #[serde(rename = "inbox-text")]
    inbox_text: String,
    #[serde(rename = "sidebar-background")]
    sidebar_background: String,
    #[serde(rename = "sidebar-border")]
    sidebar_border: String,
    #[serde(rename = "sidebar-header-text")]
    sidebar_header_text: String,
    #[serde(rename = "sidebar-tree-border")]
    sidebar_tree_border: String,
    #[serde(rename = "sidebar-text")]
    sidebar_text: String,
    #[serde(rename = "sidebar-selected-background")]
    sidebar_selected_background: String,
    #[serde(rename = "sidebar-button-background")]
    sidebar_button_background: String,
    #[serde(rename = "topbar-background")]
    topbar_background: String,
    #[serde(rename = "topbar-active-text")]
    topbar_active_text: String,
    #[serde(rename = "topbar-active-background")]
    topbar_active_background: String,
    #[serde(rename = "topbar-inactive-text")]
    topbar_inactive_text: String,
    #[serde(rename = "topbar-border")]
    topbar_border: String,
    #[serde(rename = "topbar-hover-background")]
    topbar_hover_background: String,
    #[serde(rename = "topbar-hover-text")]
    topbar_hover_text: String,
}

impl Theme {
    pub fn load() -> Self {
        let file: ThemeFile = serde_json::from_str(include_str!("../../assets/themes/dark.json"))
            .expect("Failed to parse the dark theme");
        let colors = file
            .style
            .into_iter()
            .next()
            .expect("The dark theme must define a style");

        Self {
            inbox_background: parse_color(&colors.inbox_background),
            inbox_header_border: parse_color(&colors.inbox_header_border),
            inbox_border: parse_color(&colors.inbox_border),
            inbox_header_text: parse_color(&colors.inbox_header_text),
            inbox_text: parse_color(&colors.inbox_text),
            sidebar_background: parse_color(&colors.sidebar_background),
            sidebar_border: parse_color(&colors.sidebar_border),
            sidebar_header_text: parse_color(&colors.sidebar_header_text),
            sidebar_tree_border: parse_color(&colors.sidebar_tree_border),
            sidebar_text: parse_color(&colors.sidebar_text),
            sidebar_selected_background: parse_color(&colors.sidebar_selected_background),
            sidebar_button_background: parse_color(&colors.sidebar_button_background),
            topbar_background: parse_color(&colors.topbar_background),
            topbar_active_text: parse_color(&colors.topbar_active_text),
            topbar_active_background: parse_color(&colors.topbar_active_background),
            topbar_inactive_text: parse_color(&colors.topbar_inactive_text),
            topbar_border: parse_color(&colors.topbar_border),
            topbar_hover_background: parse_color(&colors.topbar_hover_background),
            topbar_hover_text: parse_color(&colors.topbar_hover_text),
        }
    }
}

fn parse_color(value: &str) -> u32 {
    u32::from_str_radix(value.trim_start_matches("0x"), 16)
        .unwrap_or_else(|_| panic!("Invalid theme color: {value}"))
}