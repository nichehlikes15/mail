use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Theme {
    pub inbox_background: String,
    pub inbox_header_border: String,
    pub inbox_border: String,
    pub inbox_header_text: String,
    pub inbox_text: String,
    pub sidebar_background: String,
    pub sidebar_border: String,
    pub sidebar_header_text: String,
    pub sidebar_tree_border: String,
    pub sidebar_text: String,
    pub sidebar_selected_background: String,
    pub sidebar_button_background: String,
    pub mailtopbar_background: String,
    pub mailtopbar_active_text: String,
    pub mailtopbar_active_background: String,
    pub mailtopbar_inactive_text: String,
    pub mailtopbar_border: String,
    pub mailtopbar_hover_background: String,
    pub mailtopbar_hover_text: String,
    pub topbar_background: String,
    pub topbar_active_text: String,
    pub topbar_active_background: String,
    pub topbar_inactive_text: String,
    pub topbar_border: String,
    pub topbar_hover_background: String,
    pub topbar_hover_text: String,
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
    #[serde(rename = "mailtopbar-background")]
    mailtopbar_background: String,
    #[serde(rename = "mailtopbar-active-text")]
    mailtopbar_active_text: String,
    #[serde(rename = "mailtopbar-active-background")]
    mailtopbar_active_background: String,
    #[serde(rename = "mailtopbar-inactive-text")]
    mailtopbar_inactive_text: String,
    #[serde(rename = "mailtopbar-border")]
    mailtopbar_border: String,
    #[serde(rename = "mailtopbar-hover-background")]
    mailtopbar_hover_background: String,
    #[serde(rename = "mailtopbar-hover-text")]
    mailtopbar_hover_text: String,
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
            inbox_background: colors.inbox_background,
            inbox_header_border: colors.inbox_header_border,
            inbox_border: colors.inbox_border,
            inbox_header_text: colors.inbox_header_text,
            inbox_text: colors.inbox_text,
            sidebar_background: colors.sidebar_background,
            sidebar_border: colors.sidebar_border,
            sidebar_header_text: colors.sidebar_header_text,
            sidebar_tree_border: colors.sidebar_tree_border,
            sidebar_text: colors.sidebar_text,
            sidebar_selected_background: colors.sidebar_selected_background,
            sidebar_button_background: colors.sidebar_button_background,
            mailtopbar_background: colors.mailtopbar_background,
            mailtopbar_active_text: colors.mailtopbar_active_text,
            mailtopbar_active_background: colors.mailtopbar_active_background,
            mailtopbar_inactive_text: colors.mailtopbar_inactive_text,
            mailtopbar_border: colors.mailtopbar_border,
            mailtopbar_hover_background: colors.mailtopbar_hover_background,
            mailtopbar_hover_text: colors.mailtopbar_hover_text,
            topbar_background: colors.topbar_background,
            topbar_active_text: colors.topbar_active_text,
            topbar_active_background: colors.topbar_active_background,
            topbar_inactive_text: colors.topbar_inactive_text,
            topbar_border: colors.topbar_border,
            topbar_hover_background: colors.topbar_hover_background,
            topbar_hover_text: colors.topbar_hover_text,
        }
    }

    pub fn color(value: &str) -> u32 {
        u32::from_str_radix(value.trim_start_matches('#'), 16)
            .unwrap_or_else(|_| panic!("Invalid theme color: {value}"))
    }
}