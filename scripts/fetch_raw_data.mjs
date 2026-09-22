import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import https from 'https';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const rawDir = path.join(__dirname, '..', 'assets', 'raw');
fs.mkdirSync(rawDir, { recursive: true });

const BASE_URL = 'https://raw.githubusercontent.com/krmanik/HSK-3.0/main';

const filesToFetch = [
  // HSK 1 to 7-9 Words (TSV)
  {
    url: `${BASE_URL}/Scripts%20and%20data/tsv/HSK%201.tsv`,
    dest: 'hsk1_words.tsv'
  },
  {
    url: `${BASE_URL}/Scripts%20and%20data/tsv/HSK%202.tsv`,
    dest: 'hsk2_words.tsv'
  },
  {
    url: `${BASE_URL}/Scripts%20and%20data/tsv/HSK%203.tsv`,
    dest: 'hsk3_words.tsv'
  },
  {
    url: `${BASE_URL}/Scripts%20and%20data/tsv/HSK%204.tsv`,
    dest: 'hsk4_words.tsv'
  },
  {
    url: `${BASE_URL}/Scripts%20and%20data/tsv/HSK%205.tsv`,
    dest: 'hsk5_words.tsv'
  },
  {
    url: `${BASE_URL}/Scripts%20and%20data/tsv/HSK%206.tsv`,
    dest: 'hsk6_words.tsv'
  },
  {
    url: `${BASE_URL}/Scripts%20and%20data/tsv/HSK%207-9.tsv`,
    dest: 'hsk7-9_words.tsv'
  },
  // HSK 1 to 7-9 Hanzi (TXT)
  {
    url: `${BASE_URL}/New%20HSK%20(2021)/HSK%20Hanzi/HSK%201.txt`,
    dest: 'hsk1_hanzi.txt'
  },
  {
    url: `${BASE_URL}/New%20HSK%20(2021)/HSK%20Hanzi/HSK%202.txt`,
    dest: 'hsk2_hanzi.txt'
  },
  {
    url: `${BASE_URL}/New%20HSK%20(2021)/HSK%20Hanzi/HSK%203.txt`,
    dest: 'hsk3_hanzi.txt'
  },
  {
    url: `${BASE_URL}/New%20HSK%20(2021)/HSK%20Hanzi/HSK%204.txt`,
    dest: 'hsk4_hanzi.txt'
  },
  {
    url: `${BASE_URL}/New%20HSK%20(2021)/HSK%20Hanzi/HSK%205.txt`,
    dest: 'hsk5_hanzi.txt'
  },
  {
    url: `${BASE_URL}/New%20HSK%20(2021)/HSK%20Hanzi/HSK%206.txt`,
    dest: 'hsk6_hanzi.txt'
  },
  {
    url: `${BASE_URL}/New%20HSK%20(2021)/HSK%20Hanzi/HSK%207-9.txt`,
    dest: 'hsk7-9_hanzi.txt'
  }
];

function download(url) {
  return new Promise((resolve, reject) => {
    https.get(url, (res) => {
      if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
        return download(res.headers.location).then(resolve).catch(reject);
      }
      if (res.statusCode !== 200) {
        return reject(new Error(`Failed to download ${url}: status ${res.statusCode}`));
      }
      const chunks = [];
      res.on('data', chunk => chunks.push(chunk));
      res.on('end', () => resolve(Buffer.concat(chunks)));
      res.on('error', reject);
    }).on('error', reject);
  });
}

async function main() {
  console.log('Fetching raw HSK 3.0 data files...');
  const checksumEntries = [];

  for (const item of filesToFetch) {
    const destPath = path.join(rawDir, item.dest);
    console.log(`Downloading ${item.dest}...`);
    const buffer = await download(item.url);
    fs.writeFileSync(destPath, buffer);
    const hash = crypto.createHash('sha256').update(buffer).digest('hex');
    checksumEntries.push(`${hash}  raw/${item.dest}`);
    console.log(`  Saved ${item.dest} (SHA256: ${hash})`);
  }

  const checksumFilePath = path.join(__dirname, '..', 'assets', 'checksums.sha256');
  fs.writeFileSync(checksumFilePath, checksumEntries.join('\n') + '\n', 'utf-8');
  console.log(`\nChecksums written to assets/checksums.sha256`);
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
