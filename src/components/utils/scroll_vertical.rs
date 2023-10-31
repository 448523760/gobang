use std::cell::Cell;

use tui::{backend::Backend, layout::Rect, Frame};

use crate::ui::scrollbar::draw_scrollbar;

pub struct VerticalScroll {
  top: Cell<usize>,
  max_top: Cell<usize>,
  inside: bool,
  border: bool,
}

impl VerticalScroll {
  pub const fn new(border: bool, inside: bool) -> Self {
    Self { top: Cell::new(0), max_top: Cell::new(0), border, inside }
  }

  pub fn get_top(&self) -> usize {
    self.top.get()
  }

  pub fn reset(&self) {
    self.top.set(0);
  }

  pub fn update(&self, selection: usize, selection_max: usize, visual_height: usize) -> usize {
    let new_top = calc_scroll_top(self.get_top(), visual_height, selection, selection_max);
    self.top.set(new_top);

    if visual_height == 0 {
      self.max_top.set(0);
    } else {
      let new_max = selection_max.saturating_sub(visual_height);
      self.max_top.set(new_max);
    }

    new_top
  }

  pub fn draw<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
    draw_scrollbar(f, r, self.max_top.get(), self.top.get(), self.border, self.inside);
  }
}

/// 计算滚动条位置
/// current_top，表示当前滚动条的位置；
/// height_in_lines，表示可见区域的高度（以行为单位）；
/// selection，表示当前选择的位置；
/// selection_max，表示可选择的最大位置。
/// 函数的返回类型是usize，表示滚动条的新位置。
const fn calc_scroll_top(
  current_top: usize,
  height_in_lines: usize,
  selection: usize,
  selection_max: usize,
) -> usize {
  if height_in_lines == 0 {
    return 0;
  }

  // 这个条件判断语句检查选择的最大位置是否小于等于可见区域的高度。
  // 如果是的话，表示所有的内容都可以在可见区域内显示，不需要滚动，
  // 所以函数直接返回0。
  if selection_max <= height_in_lines {
    return 0;
  }

  if current_top + height_in_lines <= selection {
    // 如果当前滚动条的位置加上可见区域的高度小于等于选择位置，
    // 表示选择位置在可见区域之下，滚动条需要向下滚动。
    // 计算滚动条的新位置是选择位置减去可见区域的高度，
    // 然后再加上1（因为索引从0开始）。
    // 这里使用了saturating_sub函数来确保结果不会小于0。
    selection.saturating_sub(height_in_lines) + 1
  } else if current_top > selection {
    // 如果当前滚动条的位置大于选择位置，表示选择位置在可见区域之上，
    // 滚动条需要向上滚动。 滚动条的新位置直接就是选择位置。
    selection
  } else {
    // 如果以上两个条件都不满足，表示选择位置在可见区域内，滚动条位置不需要改变，
    // 保持当前滚动条的位置。
    current_top
  }
}

#[cfg(test)]
mod tests {
  use super::calc_scroll_top;

  #[test]
  fn test_scroll_no_scroll_to_top() {
    assert_eq!(calc_scroll_top(1, 10, 4, 4), 0);
  }

  #[test]
  fn test_scroll_zero_height() {
    assert_eq!(calc_scroll_top(4, 0, 4, 3), 0);
  }
}
