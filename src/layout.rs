use tao::dpi::{LogicalPosition, LogicalSize};
use wry::Rect;

#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub best_cols: usize,
    pub best_rows: usize,
}

pub fn compute_optimal_window_layout(area: LogicalSize<u32>, n_windows: usize) -> Layout {
    let total_width = area.width;
    let total_height = area.height;

    let (_, best_cols, best_rows) = (1..=n_windows)
        .map(|cols| {
            let rows = (n_windows + cols - 1) / cols; // ceil(n_windows / cols)
            let window_width = total_width as f32 / cols as f32;
            let window_height = total_height as f32 / rows as f32;
            let window_area = window_width * window_height;

            (window_area, cols, rows)
        })
        .max_by(|(lhs_area, _, _), (rhs_area, _, _)| {
            lhs_area
                .partial_cmp(rhs_area)
                .expect("window size should not be NaN")
        })
        .unwrap_or_default(); // when user gives n_windows = 0

    Layout {
        best_cols,
        best_rows,
    }
}

pub fn compute_bounds<const N: usize>(area: LogicalSize<u32>, layout: Layout) -> [Rect; N] {
    let cell_width = area.width as f32 / layout.best_cols as f32;
    let cell_height = area.height as f32 / layout.best_rows as f32;

    std::array::from_fn(|i| {
        let row = i / layout.best_cols;
        let col = i % layout.best_cols;

        let x = col as f32 * cell_width;
        let y = row as f32 * cell_height;

        let width = cell_width;
        let height = cell_height;

        Rect {
            position: LogicalPosition::new(x, y).into(),
            size: LogicalSize::new(width, height).into(),
        }
    })
}
