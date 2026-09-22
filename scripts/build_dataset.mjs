import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import { pinyin } from 'pinyin-pro';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const assetsDir = path.join(__dirname, '..', 'assets');
const rawDir = path.join(assetsDir, 'raw');
const checksumsFile = path.join(assetsDir, 'checksums.sha256');

// 1. Verify SHA-256 Checksums for Supply Chain Integrity
console.log('--- Step 1: Verifying SHA-256 Data Integrity ---');
if (!fs.existsSync(checksumsFile)) {
  console.error(`Error: Checksum file not found at ${checksumsFile}`);
  process.exit(1);
}

const checksumLines = fs.readFileSync(checksumsFile, 'utf-8')
  .split('\n')
  .map(l => l.trim())
  .filter(Boolean);

for (const line of checksumLines) {
  const [expectedHash, relativePath] = line.split(/\s+/);
  const filePath = path.join(assetsDir, relativePath);
  if (!fs.existsSync(filePath)) {
    console.error(`Error: Required data file missing: ${filePath}`);
    process.exit(1);
  }
  const fileBuffer = fs.readFileSync(filePath);
  const actualHash = crypto.createHash('sha256').update(fileBuffer).digest('hex');
  if (actualHash !== expectedHash) {
    console.error(`SECURITY ALERT: SHA-256 mismatch for ${relativePath}!`);
    console.error(`  Expected: ${expectedHash}`);
    console.error(`  Actual:   ${actualHash}`);
    process.exit(1);
  }
}
console.log(`Verified ${checksumLines.length} files successfully against pinned SHA-256 hashes.`);

// 2. Data Structures and Parsing
console.log('\n--- Step 2: Parsing and PinyinPro Indexing ---');

const items = [];
let nextId = 1;

// Map of single hanzi -> English meanings gathered from words, for fallback
const hanziMeanings = new Map();

// Helper to clean tone-stripped pinyin
function cleanPinyin(str) {
  return str.toLowerCase().replace(/[^a-z0-9]/g, '');
}

// Levels to process
const wordFiles = [
  { file: 'hsk1_words.tsv', level: 1 },
  { file: 'hsk2_words.tsv', level: 2 },
  { file: 'hsk3_words.tsv', level: 3 },
  { file: 'hsk4_words.tsv', level: 4 },
  { file: 'hsk5_words.tsv', level: 5 },
  { file: 'hsk6_words.tsv', level: 6 },
  { file: 'hsk7-9_words.tsv', level: 7 }, // 7 represents HSK 7-9
];

const hanziFiles = [
  { file: 'hsk1_hanzi.txt', level: 1 },
  { file: 'hsk2_hanzi.txt', level: 2 },
  { file: 'hsk3_hanzi.txt', level: 3 },
  { file: 'hsk4_hanzi.txt', level: 4 },
  { file: 'hsk5_hanzi.txt', level: 5 },
  { file: 'hsk6_hanzi.txt', level: 6 },
  { file: 'hsk7-9_hanzi.txt', level: 7 },
];

// Set of words already added to deduplicate if any
const seenWords = new Set();

for (const { file, level } of wordFiles) {
  const filePath = path.join(rawDir, file);
  const content = fs.readFileSync(filePath, 'utf-8');
  const lines = content.split('\n');

  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    const parts = trimmed.split('\t');
    if (parts.length < 4) continue;

    let [traditional, simplified, rawPinyin, meaning] = parts;
    simplified = simplified.replace(/\d+$/, '').trim(); // e.g. "本1" -> "本"
    traditional = traditional.replace(/\d+$/, '').trim();

    if (!simplified) continue;
    const dedupKey = `${simplified}__${level}`;
    if (seenWords.has(dedupKey)) continue;
    seenWords.add(dedupKey);

    // If single char, remember meaning
    if (simplified.length === 1 && !hanziMeanings.has(simplified)) {
      hanziMeanings.set(simplified, meaning);
    }

    // Generate accurate pinyin using PinyinPro
    const proTone = pinyin(simplified);
    const proNone = pinyin(simplified, { toneType: 'none' });
    const proNum = pinyin(simplified, { toneType: 'num' });
    const proInitials = pinyin(simplified, { pattern: 'first', toneType: 'none' });

    const cleanToneless = cleanPinyin(proNone);
    const cleanNum = cleanPinyin(proNum);
    const cleanInit = cleanPinyin(proInitials);

    // Also collect tone-stripped raw pinyin from TSV as backup
    const tsvClean = cleanPinyin(rawPinyin);

    items.push({
      id: nextId++,
      simplified,
      traditional,
      pinyin_display: proTone || rawPinyin,
      pinyin_clean: cleanToneless,
      pinyin_numbered: cleanNum,
      pinyin_initials: cleanInit,
      alt_pinyin: tsvClean !== cleanToneless ? tsvClean : '',
      polyphones: [],
      level,
      meaning: meaning.trim(),
      kind: 'Word'
    });
  }
}

console.log(`Processed ${items.length} HSK vocabulary words.`);

// Process Hanzi (Single Characters)
let hanziCount = 0;
const seenHanzi = new Set();

for (const { file, level } of hanziFiles) {
  const filePath = path.join(rawDir, file);
  const content = fs.readFileSync(filePath, 'utf-8');
  const chars = content.split(/\s+/).map(c => c.trim()).filter(Boolean);

  for (const char of chars) {
    if (seenHanzi.has(char)) continue;
    seenHanzi.add(char);

    // Query PinyinPro for all readings (polyphones)
    const allTones = pinyin(char, { multiple: true, type: 'array' });
    const allNones = pinyin(char, { multiple: true, toneType: 'none', type: 'array' });
    const allNums = pinyin(char, { multiple: true, toneType: 'num', type: 'array' });
    const primaryInit = cleanPinyin(pinyin(char, { pattern: 'first', toneType: 'none' }));

    const uniqueNones = Array.from(new Set(allNones.map(cleanPinyin)));
    const primaryTone = allTones[0] || '';
    const primaryNone = uniqueNones[0] || '';
    const primaryNum = cleanPinyin(allNums[0] || '');

    // Get meaning if previously encountered in single-char word
    const meaning = hanziMeanings.get(char) || '';

    items.push({
      id: nextId++,
      simplified: char,
      traditional: char, // Single hanzi standard form
      pinyin_display: allTones.join(', '),
      pinyin_clean: primaryNone,
      pinyin_numbered: primaryNum,
      pinyin_initials: primaryInit,
      alt_pinyin: uniqueNones.slice(1).join(' '),
      polyphones: allTones,
      level,
      meaning,
      kind: 'Hanzi'
    });
    hanziCount++;
  }
}

console.log(`Processed ${hanziCount} HSK Chinese characters.`);
console.log(`Total database entries: ${items.length}`);

// Write the compiled dataset to assets/hsk30_index.json
const outputFile = path.join(assetsDir, 'hsk30_index.json');
fs.writeFileSync(outputFile, JSON.stringify(items), 'utf-8');

const stats = fs.statSync(outputFile);
const sizeMb = (stats.size / (1024 * 1024)).toFixed(2);
console.log(`\nSuccessfully wrote ${items.length} items to ${outputFile} (${sizeMb} MB)`);
