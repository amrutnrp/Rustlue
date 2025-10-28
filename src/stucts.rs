
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub window: WindowConfig,
    #[serde(default)]
    pub widgets: Vec<WidgetConfig>,
}

#[derive(Deserialize, Default)]
pub struct WindowConfig {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub width: i32,
    #[serde(default)]
    pub height: i32,
    #[serde(default)]
    pub x: Option<i32>,
    #[serde(default)]
    pub y: Option<i32>,
    #[serde(default)]
    pub font: Option<String>,
	#[serde(default)]
    pub bgcolor: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct WidgetConfig {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub fontsize: Option<i32>,
    // #[serde(default)]
    // pub expand: Option<String>,
    #[serde(default)]
    pub bgcolor: Option<u32>,
    #[serde(default)]
    pub fgcolor: Option<u32>,
    // #[serde(default)]
    // pub action: Option<String>,
    #[serde(default)]
    pub x: Option<i32>,
    #[serde(default)]
    pub y: Option<i32>,
    #[serde(default)]
    pub width: Option<i32>,
    #[serde(default)]
    pub height: Option<i32>,
    // #[serde(default)]
    // pub checksize: Option<i32>,
    #[serde(default)]
    pub children: Option<Vec<String>>,
}

