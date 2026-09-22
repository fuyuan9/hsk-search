# HSK 3.0 Chinese Hanzi & Word Search Client

[![CI & Supply Chain Security](https://github.com/fuyuan9/hsk-search/actions/workflows/ci.yml/badge.svg)](https://github.com/fuyuan9/hsk-search/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Data License: CC BY--SA 4.0](https://img.shields.io/badge/Data%20License-CC%20BY--SA%204.0-lightgrey.svg)](LICENSE-DATA)

A blazingly fast, terminal-based Chinese character and vocabulary search client for **HSK 3.0 (新版HSK考试大纲)**, built with Rust and [ratatui](https://docs.rs/ratatui/latest/ratatui/).

It combines the unrivaled **99.85% polyphone precision** of TypeScript's [pinyin-pro](https://github.com/zh-lx/pinyin-pro) with the speed, memory safety, and zero-dependency portability of a **100% pure Rust** application.

---

## Features

- **Multi-Dimensional Search**:
  - **Toneless Pinyin**: Type `nihao` &rarr; matches `你好 (nǐ hǎo)`
  - **Pinyin Initials (拼音首字母)**: Type `yh` &rarr; matches `银行 (yínháng)`, `zg` &rarr; matches `中国 (zhōngguó)`
  - **Numbered Pinyin**: Type `yin2hang2` &rarr; matches `银行`
  - **Hanzi Direct Match**: Type `中国` or `国`
  - **English Meanings**: Type `bank` &rarr; matches `银行`, `china` &rarr; matches `中国`
- **PinyinPro High-Precision Polyphone Support**:
  - Correctly disambiguates tricky Chinese polyphones (多音字) based on linguistic context (e.g. `银行 yínháng` vs `行走 xíngzǒu`, `长大 zhǎngdà` vs `长城 chángchéng`, `音乐 yīnyuè` vs `快乐 kuàilè`).
  - Lists all legitimate pronunciations for polyphonic single characters (e.g. `行: xíng, háng, hàng, héng`).
- **Complete HSK 3.0 Dataset (14,041 entries)**:
  - **11,042 Vocabulary Words** across HSK Level 1 to Level 7-9.
  - **2,999 Hanzi (Chinese Characters)** covering the official standard syllabus.
- **Modern Terminal User Interface**:
  - Color-coded HSK level badges.
  - Full CJK display width awareness (no broken tables or cut-off Chinese characters).
  - Split-view layout: Searchable results table on the left, rich inspector card on the right.
  - Help / About modal dialog (`F2` or `?`).
- **Sub-millisecond Search Performance**:
  - Embedded zero-copy dataset, instant startup (< 5ms), < 1ms keystroke search response.

---

## Supply Chain Security & Reliability

This project adopts a strict **Defense-in-Depth** supply chain security strategy:

1. **Upstream Data Tamper Protection (SHA-256 Pinning)**:
   - All 14 raw dataset files from [`krmanik/HSK-3.0`](https://github.com/krmanik/HSK-3.0) are cryptographically verified against pinned SHA-256 hashes recorded in [`assets/checksums.sha256`](assets/checksums.sha256). Any unauthorized modification triggers an immediate abort.
2. **Strictly Minimal & Reputable Dependencies**:
   - Only **5 battle-tested, official Rust crates** are used:
     - `ratatui` (TUI framework standard)
     - `crossterm` (Pure Rust terminal abstraction)
     - `serde` & `serde_json` (De-facto serialization standard)
     - `unicode-width` (Official Unicode CJK width handling)
   - Search matching and ranking algorithms are implemented in **100% safe Rust (`src/search.rs`)** without untrusted third-party fuzzy matching crates.
3. **Reproducible Builds & Lockfile Pinning**:
   - `Cargo.lock` and `scripts/package-lock.json` are committed with exact cryptographic checksums.
   - Build script (`build.rs`) is eliminated to guarantee zero network access during compilation.
4. **Automated Security Auditing**:
   - `cargo audit` (RustSec Advisory Database) checks for CVEs.
   - `cargo deny` ensures strict license compliance and bans unauthorized crates.

---

## Quality Assurance: rustfmt, Clippy & CI

### Code Formatting
```bash
cargo fmt --all -- --check
```

### Static Analysis with Clippy
Zero-warning policy with strict pedantic lints enabled:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Unit & Integration Tests
```bash
cargo test --all-targets --all-features
```

### CI Workflow
All commits and pull requests are verified via GitHub Actions (`.github/workflows/ci.yml`), running fmt, clippy, tests, cargo audit, and data integrity checks.

---

## Keybindings

| Key | Action |
| :--- | :--- |
| `[Any character]` | Type into search box |
| `[Esc]` | Clear search query (if typed) / Exit application |
| `[Ctrl+C]` / `[Ctrl+Q]` | Force quit |
| `[Ctrl+U]` | Clear search box |
| `[Tab]` / `[Shift+Tab]` | Cycle HSK Level filter (`All` &rarr; `HSK 1` ... `HSK 7-9`) |
| `[F1]` | Cycle Entry Kind (`All` &rarr; `Words` &rarr; `Hanzi`) |
| `[↑]` / `[↓]` | Select previous / next item |
| `[PgUp]` / `[PgDn]` | Scroll table by 10 items |
| `[Left]` / `[Right]` | Move cursor in search box |
| `[Home]` / `[End]` | Move cursor to start / end of search box |
| `[Backspace]` / `[Del]`| Delete character |
| `[F2]` or `[?]` | Toggle Help & About modal window |

---

## Building and Running

### Prerequisites
- Rust 1.74+ (`cargo`)

### Run directly:
```bash
cargo run --release
```

### (Optional) Regenerate dataset with PinyinPro:
```bash
cd scripts
npm install
node build_dataset.mjs
```

---

## Licenses & Attributions

This project employs a dual-licensing structure to respect all upstream licenses:

- **Source Code**: [MIT License](LICENSE) (Copyright (c) 2026 fuyuan9)
- **HSK-3.0 Dataset & Derived Index**: [Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)](LICENSE-DATA)
  - Syllabus data courtesy of the Ministry of Education of the PRC.
  - Curated word lists by Mani ([krmanik/HSK-3.0](https://github.com/krmanik/HSK-3.0)).
  - Definitions from [CC-CEDICT](https://cc-cedict.org/) and [Pleco Software](https://plecoforums.com/).
  - Phonetic assistance powered by [pinyin-pro](https://github.com/zh-lx/pinyin-pro) (MIT License).
