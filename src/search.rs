use crate::models::{EntryKind, HskItem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelFilter {
    All,
    Level(u8), // 1..=7
}

impl LevelFilter {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Level(1) => "HSK 1",
            Self::Level(2) => "HSK 2",
            Self::Level(3) => "HSK 3",
            Self::Level(4) => "HSK 4",
            Self::Level(5) => "HSK 5",
            Self::Level(6) => "HSK 6",
            Self::Level(7..) => "HSK 7-9",
            Self::Level(0) => "Unknown",
        }
    }

    #[must_use]
    pub fn short_label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Level(1) => "L1",
            Self::Level(2) => "L2",
            Self::Level(3) => "L3",
            Self::Level(4) => "L4",
            Self::Level(5) => "L5",
            Self::Level(6) => "L6",
            Self::Level(7..) => "L7-9",
            Self::Level(0) => "Unknown",
        }
    }

    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::All => Self::Level(1),
            Self::Level(1) => Self::Level(2),
            Self::Level(2) => Self::Level(3),
            Self::Level(3) => Self::Level(4),
            Self::Level(4) => Self::Level(5),
            Self::Level(5) => Self::Level(6),
            Self::Level(6) => Self::Level(7),
            Self::Level(_) => Self::All,
        }
    }

    #[must_use]
    pub fn prev(self) -> Self {
        match self {
            Self::All => Self::Level(7),
            Self::Level(1) => Self::All,
            Self::Level(2) => Self::Level(1),
            Self::Level(3) => Self::Level(2),
            Self::Level(4) => Self::Level(3),
            Self::Level(5) => Self::Level(4),
            Self::Level(6) => Self::Level(5),
            Self::Level(_) => Self::Level(6),
        }
    }

    #[must_use]
    pub fn matches(self, level: u8) -> bool {
        match self {
            Self::All => true,
            Self::Level(l) => l == level,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KindFilter {
    All,
    WordsOnly,
    HanziOnly,
}

impl KindFilter {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All (全部)",
            Self::WordsOnly => "Words Only (词汇)",
            Self::HanziOnly => "Hanzi Only (汉字)",
        }
    }

    #[must_use]
    pub fn short_label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::WordsOnly => "Words",
            Self::HanziOnly => "Hanzi",
        }
    }

    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::All => Self::WordsOnly,
            Self::WordsOnly => Self::HanziOnly,
            Self::HanziOnly => Self::All,
        }
    }

    #[must_use]
    pub fn matches(self, kind: EntryKind) -> bool {
        match self {
            Self::All => true,
            Self::WordsOnly => kind == EntryKind::Word,
            Self::HanziOnly => kind == EntryKind::Hanzi,
        }
    }
}

/// Computes relevance score for a given item and query. Returns `Some(score)` if matches.
#[must_use]
pub fn score_item(item: &HskItem, query_lower: &str, clean_query: &str) -> Option<u32> {
    if query_lower.is_empty() {
        return Some(100);
    }

    let mut score = 0u32;
    let mut matched = false;

    // 1. Exact Hanzi match (simplified or traditional)
    if item.simplified == query_lower || item.traditional == query_lower {
        score = score.max(10_000);
        matched = true;
    } else if item.simplified.starts_with(query_lower) || item.traditional.starts_with(query_lower)
    {
        score = score.max(6_000);
        matched = true;
    } else if item.simplified.contains(query_lower) || item.traditional.contains(query_lower) {
        score = score.max(3_000);
        matched = true;
    }

    // 2. Pinyin matching using alphanumeric clean query
    if !clean_query.is_empty() {
        // Exact clean pinyin match (e.g. "yinhang")
        if item.pinyin_clean == clean_query {
            score = score.max(5_000);
            matched = true;
        } else if item.pinyin_clean.starts_with(clean_query) {
            score = score.max(4_000);
            matched = true;
        } else if item.pinyin_clean.contains(clean_query) {
            score = score.max(2_500);
            matched = true;
        }

        // Pinyin initials (e.g. "yh" for "yinhang", "zg" for "zhongguo")
        if item.pinyin_initials == clean_query {
            score = score.max(4_500);
            matched = true;
        } else if item.pinyin_initials.starts_with(clean_query) && clean_query.len() > 1 {
            score = score.max(3_200);
            matched = true;
        }

        // Alternative pinyin / polyphones for single characters or variants
        if !item.alt_pinyin.is_empty() {
            for alt in item.alt_pinyin.split_whitespace() {
                if alt == clean_query {
                    score = score.max(4_200);
                    matched = true;
                } else if alt.starts_with(clean_query) {
                    score = score.max(3_100);
                    matched = true;
                }
            }
        }

        // Numbered pinyin (e.g. "yin2hang2")
        if item.pinyin_numbered.starts_with(clean_query) {
            score = score.max(3_000);
            matched = true;
        }
    }

    // 3. English definition match
    if item.meaning.to_ascii_lowercase().contains(query_lower) {
        // Higher bonus if meaning starts with query (e.g. "china" matches "China...")
        let bonus = if item.meaning.to_ascii_lowercase().starts_with(query_lower) {
            2_000
        } else {
            1_200
        };
        score = score.max(bonus);
        matched = true;
    }

    if matched {
        // Prioritize lower HSK levels (HSK 1 over HSK 6)
        let level_penalty = u32::from(item.level).saturating_mul(10);
        Some(score.saturating_sub(level_penalty))
    } else {
        None
    }
}

/// Executes a search query over the items with level and kind filtering.
#[must_use]
pub fn search_items<'a>(
    items: &'a [HskItem],
    query: &str,
    level_filter: LevelFilter,
    kind_filter: KindFilter,
) -> Vec<&'a HskItem> {
    let query_lower = query.trim().to_ascii_lowercase();
    let clean_query: String = query_lower
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect();

    let mut scored: Vec<(&'a HskItem, u32)> = items
        .iter()
        .filter(|item| level_filter.matches(item.level))
        .filter(|item| kind_filter.matches(item.kind))
        .filter_map(|item| score_item(item, &query_lower, &clean_query).map(|s| (item, s)))
        .collect();

    // Sort by:
    // 1. Score descending
    // 2. Level ascending (beginner levels first)
    // 3. ID ascending
    scored.sort_by(|(a_item, a_score), (b_item, b_score)| {
        b_score
            .cmp(a_score)
            .then_with(|| a_item.level.cmp(&b_item.level))
            .then_with(|| a_item.id.cmp(&b_item.id))
    });

    scored.into_iter().map(|(item, _)| item).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_dataset() -> Vec<HskItem> {
        vec![
            HskItem {
                id: 1,
                simplified: "银行".to_string(),
                traditional: "銀行".to_string(),
                pinyin_display: "yín háng".to_string(),
                pinyin_clean: "yinhang".to_string(),
                pinyin_numbered: "yin2hang2".to_string(),
                pinyin_initials: "yh".to_string(),
                alt_pinyin: String::new(),
                polyphones: vec![],
                level: 2,
                meaning: "bank; CL:家[jia1]".to_string(),
                kind: EntryKind::Word,
            },
            HskItem {
                id: 2,
                simplified: "行走".to_string(),
                traditional: "行走".to_string(),
                pinyin_display: "xíng zǒu".to_string(),
                pinyin_clean: "xingzou".to_string(),
                pinyin_numbered: "xing2zou3".to_string(),
                pinyin_initials: "xz".to_string(),
                alt_pinyin: String::new(),
                polyphones: vec![],
                level: 5,
                meaning: "to walk".to_string(),
                kind: EntryKind::Word,
            },
            HskItem {
                id: 3,
                simplified: "行".to_string(),
                traditional: "行".to_string(),
                pinyin_display: "xíng, háng".to_string(),
                pinyin_clean: "xing".to_string(),
                pinyin_numbered: "xing2".to_string(),
                pinyin_initials: "x".to_string(),
                alt_pinyin: "hang".to_string(),
                polyphones: vec!["xíng".to_string(), "háng".to_string()],
                level: 1,
                meaning: "all right; capable; line".to_string(),
                kind: EntryKind::Hanzi,
            },
            HskItem {
                id: 4,
                simplified: "中国".to_string(),
                traditional: "中國".to_string(),
                pinyin_display: "zhōng guó".to_string(),
                pinyin_clean: "zhongguo".to_string(),
                pinyin_numbered: "zhong1guo2".to_string(),
                pinyin_initials: "zg".to_string(),
                alt_pinyin: String::new(),
                polyphones: vec![],
                level: 1,
                meaning: "China".to_string(),
                kind: EntryKind::Word,
            },
        ]
    }

    #[test]
    fn test_search_by_pinyin() {
        let dataset = create_sample_dataset();
        let results = search_items(&dataset, "yinhang", LevelFilter::All, KindFilter::All);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].simplified, "银行");
    }

    #[test]
    fn test_search_by_initials() {
        let dataset = create_sample_dataset();
        let results = search_items(&dataset, "yh", LevelFilter::All, KindFilter::All);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].simplified, "银行");

        let zg_results = search_items(&dataset, "zg", LevelFilter::All, KindFilter::All);
        assert_eq!(zg_results.len(), 1);
        assert_eq!(zg_results[0].simplified, "中国");
    }

    #[test]
    fn test_search_polyphone_single_char() {
        let dataset = create_sample_dataset();
        // "hang" should match "行" via alt_pinyin
        let hang_results = search_items(&dataset, "hang", LevelFilter::All, KindFilter::HanziOnly);
        assert_eq!(hang_results.len(), 1);
        assert_eq!(hang_results[0].simplified, "行");

        // "xing" should also match "行"
        let xing_results = search_items(&dataset, "xing", LevelFilter::All, KindFilter::HanziOnly);
        assert_eq!(xing_results.len(), 1);
        assert_eq!(xing_results[0].simplified, "行");
    }

    #[test]
    fn test_search_by_english() {
        let dataset = create_sample_dataset();
        let results = search_items(&dataset, "china", LevelFilter::All, KindFilter::All);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].simplified, "中国");
    }

    #[test]
    fn test_level_filtering() {
        let dataset = create_sample_dataset();
        let hsk1_results = search_items(&dataset, "", LevelFilter::Level(1), KindFilter::All);
        assert_eq!(hsk1_results.len(), 2); // "行" and "中国"
        let hsk2_results = search_items(&dataset, "", LevelFilter::Level(2), KindFilter::All);
        assert_eq!(hsk2_results.len(), 1); // "银行"
    }

    #[test]
    fn test_filter_labels() {
        assert_eq!(LevelFilter::All.label(), "All");
        assert_eq!(LevelFilter::Level(1).label(), "HSK 1");
        assert_eq!(LevelFilter::Level(7).label(), "HSK 7-9");
        assert_eq!(KindFilter::All.label(), "All (全部)");
        assert_eq!(KindFilter::WordsOnly.label(), "Words Only (词汇)");
        assert_eq!(KindFilter::HanziOnly.label(), "Hanzi Only (汉字)");
    }
}
