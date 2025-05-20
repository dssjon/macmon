use ratatui::{
  prelude::*,
  symbols,
  widgets::{Widget, block::Block},
};

#[derive(Debug, Default, Clone)]
pub struct GradientGauge<'a> {
  block: Option<Block<'a>>,
  ratio: f64,
  style: Style,
  start: Color,
  mid: Color,
  end: Color,
}

impl<'a> GradientGauge<'a> {
  pub fn block(mut self, block: Block<'a>) -> Self {
    self.block = Some(block);
    self
  }

  pub fn ratio(mut self, ratio: f64) -> Self {
    self.ratio = ratio.clamp(0.0, 1.0);
    self
  }

  pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
    self.style = style.into();
    self
  }

  pub fn colors(mut self, start: Color, mid: Color, end: Color) -> Self {
    self.start = start;
    self.mid = mid;
    self.end = end;
    self
  }
}

impl Widget for GradientGauge<'_> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    buf.set_style(area, self.style);
    let gauge = if let Some(ref block) = self.block {
      block.render(area, buf);
      block.inner(area)
    } else {
      area
    };
    if gauge.is_empty() {
      return;
    }
    let filled = (gauge.width as f64 * self.ratio).round() as u16;
    let third = gauge.width.max(1) / 3;
    for y in gauge.top()..gauge.bottom() {
      for x in gauge.left()..gauge.right() {
        if x - gauge.left() < filled {
          let color = if x - gauge.left() < third {
            self.start
          } else if x - gauge.left() < third * 2 {
            self.mid
          } else {
            self.end
          };
          buf[(x, y)].set_symbol(symbols::block::FULL).set_fg(color);
        }
      }
    }
  }
}
