# AGENTS.md - AI Agent Guidelines for `hsk-search`

This document provides essential technical context, architecture rules, and operational guidelines for autonomous AI coding agents working on this repository.

---

## 1. Project Overview & Architecture

`hsk-search` is a high-performance terminal user interface (TUI) client for searching and browsing the complete **HSK 3.0 (新版HSK考试大纲)** Chinese vocabulary and character syllabus.

### Architectural Philosophy: Hybrid Precision + Pure Safe Rust
- **Phonetic Precision**: Chinese polyphones (多音字, e.g. `银行 yínháng` vs `行走 xíngzǒu`) require context-aware segmentation. TypeScript's `pinyin-pro` achieves 99.85% accuracy.
- **Runtime Execution**: The application runs in **100% pure Safe Rust** (`#![forbid(unsafe_code)]`). The dataset of 14,041 entries is pre-indexed with `pinyin-pro` and embedded into the binary via `include_str!`.
- **Performance**: Zero external runtime dependencies (no Node.js/npm needed by end-users), startup time < 5ms, keystroke search latency < 1ms.
- **Modal Navigation & Mouse**: Vim-inspired modal state machine (`AppMode::Insert` for typing, `AppMode::Normal` for `hjkl` traversal) prevents search-key collisions. Mouse click routing enables direct area switching.
- **CLI Contract**: Non-interactive flags (`--version`, `--help`) are processed before terminal raw mode initialization to support Homebrew tests and scripted checks.

---

## 2. Repository Layout

```
hsk-search/
├── .github/workflows/
│   ├── ci.yml               # Automated CI pipeline (fmt, clippy, test, audit, data-integrity)
│   └── release.yml          # Multi-platform binary release pipeline on v* tags
├── Cargo.toml               # Minimal, strictly vetted dependencies ([[bin]] name = "hsk")
├── Cargo.lock               # Pinned crate versions and cryptographic hashes
├── deny.toml                # cargo-deny license and ban policies
├── rustfmt.toml             # Code style standards
├── LICENSE                  # MIT License (code)
├── LICENSE-DATA             # CC BY-SA 4.0 (dataset)
├── README.md                # User documentation and keybinding reference
├── AGENTS.md                # Guidelines for AI agents (this file)
├── Formula/
│   └── hsk.rb               # Local reference Homebrew Formula
├── scripts/
│   ├── package.json         # Pinned pinyin-pro dependency
│   ├── package-lock.json    # Pinned SHA-512 hashes
│   ├── fetch_raw_data.mjs   # Downloads raw HSK 3.0 files and generates checksums
│   ├── build_dataset.mjs    # SHA-256 verified data pipeline using pinyin-pro
│   └── generate_homebrew_formula.mjs # Queries release asset digests and updates Formula/hsk.rb
├── assets/
│   ├── checksums.sha256     # Pinned SHA-256 hashes of all 14 upstream raw data files
│   └── hsk30_index.json     # Pre-compiled high-precision index (embedded into binary)
├── src/
│   ├── main.rs              # CLI flag parsing, terminal setup, panic hook, event loop
│   ├── lib.rs               # Library root exposing modules and dataset loader
│   ├── app.rs               # App state container, AppMode (Normal/Insert), mouse & key actions
│   ├── models.rs            # HskItem and EntryKind data structures
│   ├── search.rs            # High-speed multi-key search engine (Safe Rust)
│   ├── ui.rs                # Ratatui UI rendering (header, search bar, table, inspector, help)
│   └── cjk.rs               # CJK full-width (2 columns) display width and truncation
└── tests/
    └── dataset_accuracy_tests.rs # Full integration tests verifying polyphones and search
```

---

## 3. Strict Development Rules

### Rule 1: Supply Chain Security & Dependency Minimization
- **Minimal Crate Footprint**: Do NOT add new third-party crates unless explicitly requested.
- **Permitted Crates**:
  - `ratatui` (TUI framework)
  - `crossterm` (Terminal backend)
  - `serde` / `serde_json` (Serialization)
  - `unicode-width` (CJK column width calculation)
- **Zero Network in `build.rs`**: Never perform network requests or compile external binaries during cargo build.
- **Data Integrity**: If modifying data ingestion, all files in `assets/raw/` must match `assets/checksums.sha256`.

### Rule 2: CJK Character Width & Terminal Alignment
- Chinese characters are **2 terminal columns wide (Fullwidth)**.
- **Modal / Popup Boundary Safety**:
  - When rendering popups over CJK text, a 2-width character starting at `area.left() - 1` will bleed into `area.left()`, destroying the left border.
  - Always use `clear_modal_area` (in `src/ui.rs`) to reset wide characters at `area.left() - 1` before rendering modal borders.
- **Avoid Ambiguous Symbols**:
  - Do not use East Asian Ambiguous width characters like `▶` (U+25B6) or `•` (U+2022) in UI lines. Use standard ASCII markers (`>`, `-`) to ensure consistent alignment across all terminal emulators.

### Rule 3: Quality Assurance & Zero Warnings Policy
Every change must pass all of the following checks before completion:
1. **Formatting**:
   ```bash
   cargo fmt --all -- --check
   ```
2. **Static Analysis (Clippy)**:
   Must produce **zero warnings** with `-D warnings`:
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```
3. **Tests**:
   All unit and integration tests must pass:
   ```bash
   cargo test --all-targets --all-features
   ```
4. **Supply Chain Audit**:
   ```bash
   cargo deny check
   ```
5. **Release Compilation**:
   ```bash
   cargo build --release
   ```

### Rule 4: Distribution & Release Pipeline

- **Supported Platforms**:
  - `aarch64-apple-darwin` (Apple Silicon Mac)
  - `x86_64-unknown-linux-gnu` (Linux x86_64)
  - `aarch64-unknown-linux-gnu` (Linux ARM64)
  - `x86_64-pc-windows-msvc` (Windows x64)
- **Deliberately Excluded Platforms**:
  - `x86_64-apple-darwin` (Intel Mac) is **not supported** to avoid long GitHub Actions runner queue times and prioritize modern hardware.
- **Homebrew Formula Synchronization**:
  - Formula definitions live in `Formula/hsk.rb` in both this repository and the dedicated tap repository (`fuyuan9/homebrew-tap`).
  - Use `node scripts/generate_homebrew_formula.mjs <version>` to query GitHub Releases asset digests via `gh release view` and populate formula SHA-256 hashes.
- **Binary Naming**:
  - The installed executable binary MUST always be named `hsk` (configured via `[[bin]] name = "hsk"` in `Cargo.toml`).

### Rule 5: Licensing & Attribution
- All source code is licensed under **MIT** (Copyright (c) 2026 fuyuan9).
- Dataset assets are licensed under **CC BY-SA 4.0**.
- Retain proper attributions for the Ministry of Education of PRC, Mani (`krmanik/HSK-3.0`), CC-CEDICT, Pleco, and `pinyin-pro`.
