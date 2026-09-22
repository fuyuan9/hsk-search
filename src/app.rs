use crate::models::HskItem;
use crate::search::{search_items, KindFilter, LevelFilter};
use ratatui::layout::{Position, Rect};
use ratatui::widgets::TableState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    Insert,
}

impl AppMode {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Insert => "INSERT",
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AppLayoutAreas {
    pub search_bar: Rect,
    pub results_table: Rect,
    pub inspector_card: Rect,
}

pub struct App {
    pub items: Vec<HskItem>,
    pub filtered_items: Vec<HskItem>,
    pub search_query: String,
    pub cursor_position: usize,
    pub table_state: TableState,
    pub level_filter: LevelFilter,
    pub kind_filter: KindFilter,
    pub mode: AppMode,
    pub last_normal_key: Option<char>,
    pub layout_areas: AppLayoutAreas,
    pub show_help: bool,
    pub should_quit: bool,
}

impl App {
    #[must_use]
    pub fn new(items: Vec<HskItem>) -> Self {
        let mut app = Self {
            filtered_items: Vec::new(),
            items,
            search_query: String::new(),
            cursor_position: 0,
            table_state: TableState::default(),
            level_filter: LevelFilter::All,
            kind_filter: KindFilter::All,
            mode: AppMode::Insert, // Start in Insert mode for immediate typing
            last_normal_key: None,
            layout_areas: AppLayoutAreas::default(),
            show_help: false,
            should_quit: false,
        };
        app.update_search();
        app
    }

    /// Re-evaluates search query and filters, updating `filtered_items` and table selection.
    pub fn update_search(&mut self) {
        let matched = search_items(
            &self.items,
            &self.search_query,
            self.level_filter,
            self.kind_filter,
        );

        self.filtered_items = matched.into_iter().cloned().collect();

        if self.filtered_items.is_empty() {
            self.table_state.select(None);
        } else {
            let current = self.table_state.selected().unwrap_or(0);
            let clamped = current.min(self.filtered_items.len().saturating_sub(1));
            self.table_state.select(Some(clamped));
        }
    }

    #[must_use]
    pub fn selected_item(&self) -> Option<&HskItem> {
        self.table_state
            .selected()
            .and_then(|idx| self.filtered_items.get(idx))
    }

    pub fn next_item(&mut self) {
        if self.filtered_items.is_empty() {
            return;
        }
        let next_idx = match self.table_state.selected() {
            Some(i) if i + 1 < self.filtered_items.len() => i + 1,
            _ => 0,
        };
        self.table_state.select(Some(next_idx));
    }

    pub fn previous_item(&mut self) {
        if self.filtered_items.is_empty() {
            return;
        }
        let prev_idx = match self.table_state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => self.filtered_items.len().saturating_sub(1),
        };
        self.table_state.select(Some(prev_idx));
    }

    pub fn jump_to_top(&mut self) {
        if !self.filtered_items.is_empty() {
            self.table_state.select(Some(0));
        }
    }

    pub fn jump_to_bottom(&mut self) {
        if !self.filtered_items.is_empty() {
            self.table_state
                .select(Some(self.filtered_items.len().saturating_sub(1)));
        }
    }

    pub fn page_down(&mut self, page_size: usize) {
        if self.filtered_items.is_empty() {
            return;
        }
        let current = self.table_state.selected().unwrap_or(0);
        let next_idx = (current + page_size).min(self.filtered_items.len().saturating_sub(1));
        self.table_state.select(Some(next_idx));
    }

    pub fn page_up(&mut self, page_size: usize) {
        if self.filtered_items.is_empty() {
            return;
        }
        let current = self.table_state.selected().unwrap_or(0);
        let prev_idx = current.saturating_sub(page_size);
        self.table_state.select(Some(prev_idx));
    }

    pub fn insert_char(&mut self, c: char) {
        self.search_query.insert(self.cursor_position, c);
        self.cursor_position += 1;
        self.update_search();
    }

    pub fn delete_char_before_cursor(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.search_query.remove(self.cursor_position);
            self.update_search();
        }
    }

    pub fn delete_char_at_cursor(&mut self) {
        if self.cursor_position < self.search_query.len() {
            self.search_query.remove(self.cursor_position);
            self.update_search();
        }
    }

    pub fn delete_word_backward(&mut self) {
        if self.cursor_position == 0 {
            return;
        }
        let chars: Vec<char> = self.search_query.chars().collect();
        let mut new_pos = self.cursor_position;
        while new_pos > 0 && chars[new_pos - 1].is_whitespace() {
            new_pos -= 1;
        }
        while new_pos > 0 && !chars[new_pos - 1].is_whitespace() {
            new_pos -= 1;
        }
        let remove_count = self.cursor_position - new_pos;
        for _ in 0..remove_count {
            self.search_query.remove(new_pos);
        }
        self.cursor_position = new_pos;
        self.update_search();
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.search_query.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.search_query.len();
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.cursor_position = 0;
        self.update_search();
    }

    pub fn next_level(&mut self) {
        self.level_filter = self.level_filter.next();
        self.update_search();
    }

    pub fn prev_level(&mut self) {
        self.level_filter = self.level_filter.prev();
        self.update_search();
    }

    pub fn next_kind(&mut self) {
        self.kind_filter = self.kind_filter.next();
        self.update_search();
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn enter_insert(&mut self) {
        self.mode = AppMode::Insert;
        self.last_normal_key = None;
    }

    pub fn enter_insert_append(&mut self) {
        if self.cursor_position < self.search_query.len() {
            self.cursor_position += 1;
        }
        self.enter_insert();
    }

    pub fn enter_insert_start(&mut self) {
        self.cursor_position = 0;
        self.enter_insert();
    }

    pub fn enter_insert_end(&mut self) {
        self.cursor_position = self.search_query.len();
        self.enter_insert();
    }

    pub fn enter_insert_clear(&mut self) {
        self.clear_search();
        self.enter_insert();
    }

    pub fn enter_normal(&mut self) {
        self.mode = AppMode::Normal;
        self.last_normal_key = None;
    }

    pub fn handle_mouse_click(&mut self, col: u16, row: u16) {
        if self.show_help {
            self.show_help = false;
            return;
        }

        let pos = Position::new(col, row);

        // 1. Click on Search Bar -> switch to INSERT mode
        if self.layout_areas.search_bar.contains(pos) {
            self.enter_insert();
            return;
        }

        // 2. Click on Results Table -> switch to NORMAL mode & select item
        if self.layout_areas.results_table.contains(pos) {
            self.enter_normal();
            let table_rect = self.layout_areas.results_table;
            // Data rows start 3 rows below top: border (1), header (1), margin (1)
            if row >= table_rect.y + 3 && row < table_rect.y + table_rect.height.saturating_sub(1) {
                let row_offset = (row - (table_rect.y + 3)) as usize;
                let target_idx = self.table_state.offset() + row_offset;
                if target_idx < self.filtered_items.len() {
                    self.table_state.select(Some(target_idx));
                }
            }
            return;
        }

        // 3. Click on Inspector Card -> switch to NORMAL mode
        if self.layout_areas.inspector_card.contains(pos) {
            self.enter_normal();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::EntryKind;

    fn sample_item(id: u32, simp: &str, level: u8) -> HskItem {
        HskItem {
            id,
            simplified: simp.to_string(),
            traditional: simp.to_string(),
            pinyin_display: "test".to_string(),
            pinyin_clean: "test".to_string(),
            pinyin_numbered: "test1".to_string(),
            pinyin_initials: "t".to_string(),
            alt_pinyin: String::new(),
            polyphones: Vec::new(),
            meaning: "sample meaning".to_string(),
            level,
            kind: EntryKind::Word,
        }
    }

    #[test]
    fn test_mode_transitions() {
        let items = vec![sample_item(1, "一", 1)];
        let mut app = App::new(items);

        assert_eq!(app.mode, AppMode::Insert);
        app.enter_normal();
        assert_eq!(app.mode, AppMode::Normal);
        app.enter_insert();
        assert_eq!(app.mode, AppMode::Insert);
    }

    #[test]
    fn test_mouse_click_mode_switching() {
        let items = vec![
            sample_item(1, "一", 1),
            sample_item(2, "二", 1),
            sample_item(3, "三", 1),
        ];
        let mut app = App::new(items);

        app.layout_areas = AppLayoutAreas {
            search_bar: Rect::new(1, 4, 80, 3),
            results_table: Rect::new(1, 7, 50, 15),
            inspector_card: Rect::new(52, 7, 30, 15),
        };

        // Start in Insert mode
        assert_eq!(app.mode, AppMode::Insert);

        // Click in results table -> switches to Normal mode and selects item
        app.handle_mouse_click(10, 10);
        assert_eq!(app.mode, AppMode::Normal);

        // Click in search bar -> switches back to Insert mode
        app.handle_mouse_click(10, 5);
        assert_eq!(app.mode, AppMode::Insert);

        // Click in inspector card -> switches to Normal mode
        app.handle_mouse_click(60, 10);
        assert_eq!(app.mode, AppMode::Normal);
    }

    #[test]
    fn test_delete_word_backward() {
        let items = vec![sample_item(1, "你好", 1)];
        let mut app = App::new(items);

        app.search_query = "ni hao".to_string();
        app.cursor_position = 6;
        app.delete_word_backward();
        assert_eq!(app.search_query, "ni ");

        app.delete_word_backward();
        assert_eq!(app.search_query, "");
    }
}
