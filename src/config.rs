use crate::theme::{TOKYO_NIGHT, Theme, ThemePalette};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

const COLORS_OPTIONS: [Color; 7] =
  [Color::Green, Color::Yellow, Color::Red, Color::Blue, Color::Magenta, Color::Cyan, Color::Reset];

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ViewType {
  Sparkline,
  Gauge,
}

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  #[serde_inline_default(ViewType::Sparkline)]
  pub view_type: ViewType,

  #[serde_inline_default(Theme::Default)]
  pub theme: Theme,

  #[serde_inline_default(COLORS_OPTIONS[0])]
  pub color: Color,

  #[serde_inline_default(1000)]
  pub interval: u32,
}

impl Default for Config {
  fn default() -> Self {
    serde_json::from_str("{}").unwrap()
  }
}

impl Config {
  fn get_config_path() -> Option<String> {
    let home = match std::env::var("HOME") {
      Ok(home) => home,
      Err(_) => return None,
    };

    let filepath = format!("{}/.config/macmon.json", home);
    let _ = std::fs::create_dir_all(std::path::Path::new(&filepath).parent().unwrap());
    Some(filepath)
  }

  pub fn load() -> Self {
    if let Some(path) = Self::get_config_path() {
      let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return Self::default(),
      };

      let reader = std::io::BufReader::new(file);
      return serde_json::from_reader(reader).unwrap_or_default();
    }

    Self::default()
  }

  pub fn save(&self) {
    if let Some(path) = Self::get_config_path() {
      let file = match std::fs::File::create(path) {
        Ok(file) => file,
        Err(_) => return,
      };

      let writer = std::io::BufWriter::new(file);
      let _ = serde_json::to_writer_pretty(writer, self);
    }
  }

  pub fn next_theme(&mut self) {
    match self.theme {
      Theme::Default => {
        if let Some(idx) = COLORS_OPTIONS.iter().position(|&c| c == self.color) {
          if idx + 1 < COLORS_OPTIONS.len() {
            self.color = COLORS_OPTIONS[idx + 1];
          } else {
            self.theme = Theme::TokyoNight;
          }
        } else {
          self.color = COLORS_OPTIONS[0];
        }
      }
      Theme::TokyoNight => {
        self.theme = Theme::Default;
        self.color = COLORS_OPTIONS[0];
      }
    }
    self.save();
  }

  pub fn palette(&self) -> ThemePalette {
    match self.theme {
      Theme::Default => ThemePalette {
        border: self.color,
        text: Color::Reset,
        start: self.color,
        mid: self.color,
        end: self.color,
        start256: self.color,
        mid256: self.color,
        end256: self.color,
      },
      Theme::TokyoNight => TOKYO_NIGHT,
    }
  }

  pub fn next_view_type(&mut self) {
    self.view_type = match self.view_type {
      ViewType::Sparkline => ViewType::Gauge,
      ViewType::Gauge => ViewType::Sparkline,
    };
    self.save();
  }

  pub fn dec_interval(&mut self) {
    let step = 250;
    self.interval = (self.interval.saturating_sub(step).div_ceil(step) * step).max(step);
    self.save();
  }

  pub fn inc_interval(&mut self) {
    let step = 250;
    self.interval = (self.interval.saturating_add(step) / step * step).min(10_000);
    self.save();
  }
}
