use clap::ValueEnum;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Theme {
  Default,
  TokyoNight,
}

#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
  pub border: Color,
  pub text: Color,
  pub start: Color,
  pub mid: Color,
  pub end: Color,
  pub start256: Color,
  pub mid256: Color,
  pub end256: Color,
}

const TOKYO_NIGHT_START: Color = Color::Rgb(0x9e, 0xce, 0x6a);
const TOKYO_NIGHT_MID: Color = Color::Rgb(0xe0, 0xaf, 0x68);
const TOKYO_NIGHT_END: Color = Color::Rgb(0xf7, 0x76, 0x8e);

const TOKYO_NIGHT_START_256: Color = Color::Indexed(150);
const TOKYO_NIGHT_MID_256: Color = Color::Indexed(180);
const TOKYO_NIGHT_END_256: Color = Color::Indexed(211);

pub const TOKYO_NIGHT: ThemePalette = ThemePalette {
  border: Color::Rgb(0x7d, 0xcf, 0xff),
  text: Color::Rgb(0xcf, 0xc9, 0xc2),
  start: TOKYO_NIGHT_START,
  mid: TOKYO_NIGHT_MID,
  end: TOKYO_NIGHT_END,
  start256: TOKYO_NIGHT_START_256,
  mid256: TOKYO_NIGHT_MID_256,
  end256: TOKYO_NIGHT_END_256,
};

fn truecolor() -> bool {
  std::env::var("COLORTERM")
    .map(|v| v.contains("truecolor") || v.contains("24bit"))
    .unwrap_or(false)
}

impl ThemePalette {
  pub fn start_color(&self) -> Color {
    if truecolor() { self.start } else { self.start256 }
  }
  pub fn mid_color(&self) -> Color {
    if truecolor() { self.mid } else { self.mid256 }
  }
  pub fn end_color(&self) -> Color {
    if truecolor() { self.end } else { self.end256 }
  }
}
