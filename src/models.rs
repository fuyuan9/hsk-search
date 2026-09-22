use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryKind {
    Word,
    Hanzi,
}

impl EntryKind {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Word => "Word (词汇)",
            Self::Hanzi => "Hanzi (汉字)",
        }
    }

    #[must_use]
    pub fn badge(self) -> &'static str {
        match self {
            Self::Word => "词",
            Self::Hanzi => "字",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HskItem {
    pub id: u32,
    pub simplified: String,
    pub traditional: String,
    pub pinyin_display: String,
    pub pinyin_clean: String,
    pub pinyin_numbered: String,
    pub pinyin_initials: String,
    #[serde(default)]
    pub alt_pinyin: String,
    #[serde(default)]
    pub polyphones: Vec<String>,
    pub level: u8,
    #[serde(default)]
    pub meaning: String,
    pub kind: EntryKind,
}

impl HskItem {
    #[must_use]
    pub fn level_label(&self) -> &'static str {
        match self.level {
            1 => "HSK 1",
            2 => "HSK 2",
            3 => "HSK 3",
            4 => "HSK 4",
            5 => "HSK 5",
            6 => "HSK 6",
            7.. => "HSK 7-9",
            _ => "Unknown",
        }
    }
}
