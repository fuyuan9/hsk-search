#!/usr/bin/env node

/**
 * Helper to generate or update Formula/hsk.rb for fuyuan9/homebrew-tap
 * Usage: node scripts/generate_homebrew_formula.mjs [version]
 */

import { execSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

const version = process.argv[2] || '0.1.0';
const cleanVersion = version.replace(/^v/, '');

function getReleaseDigests(ver) {
  try {
    const raw = execSync(`gh release view v${ver} --json assets`, { encoding: 'utf-8' });
    const data = JSON.parse(raw);
    const map = {};
    for (const asset of data.assets || []) {
      if (asset.digest && asset.digest.startsWith('sha256:')) {
        map[asset.name] = asset.digest.replace('sha256:', '');
      }
    }
    return map;
  } catch (err) {
    console.error('Warning: failed to query gh release assets:', err.message);
    return {};
  }
}

async function main() {
  const digests = getReleaseDigests(cleanVersion);

  const macArmSha = digests[`hsk-v${cleanVersion}-aarch64-apple-darwin.tar.gz`] || "PENDING_RELEASE_SHA256";
  const linuxX64Sha = digests[`hsk-v${cleanVersion}-x86_64-unknown-linux-gnu.tar.gz`] || "PENDING_RELEASE_SHA256";
  const linuxArmSha = digests[`hsk-v${cleanVersion}-aarch64-unknown-linux-gnu.tar.gz`] || "PENDING_RELEASE_SHA256";

  const formulaContent = `# typed: false
# frozen_string_literal: true

class Hsk < Formula
  desc "High-precision HSK 3.0 Chinese Hanzi & Word search TUI client"
  homepage "https://github.com/fuyuan9/hsk-search"
  version "${cleanVersion}"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/fuyuan9/hsk-search/releases/download/v#{version}/hsk-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "${macArmSha}"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/fuyuan9/hsk-search/releases/download/v#{version}/hsk-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "${linuxArmSha}"
    else
      url "https://github.com/fuyuan9/hsk-search/releases/download/v#{version}/hsk-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "${linuxX64Sha}"
    end
  end

  def install
    bin.install "hsk"
  end

  test do
    assert_match "hsk #{version}", shell_output("#{bin}/hsk --version")
  end
end
`;

  // Output locally in hsk-search
  const outDir = path.resolve('Formula');
  if (!fs.existsSync(outDir)) {
    fs.mkdirSync(outDir, { recursive: true });
  }
  const outFile = path.join(outDir, 'hsk.rb');
  fs.writeFileSync(outFile, formulaContent, 'utf-8');
  console.log(`Updated ${outFile}`);

  // Also update /Users/fuyuan/Desktop/homebrew-tap if present
  const tapDir = path.resolve('/Users/fuyuan/Desktop/homebrew-tap');
  if (fs.existsSync(tapDir)) {
    const tapFormulaDir = path.join(tapDir, 'Formula');
    if (!fs.existsSync(tapFormulaDir)) {
      fs.mkdirSync(tapFormulaDir, { recursive: true });
    }
    const tapOutFile = path.join(tapFormulaDir, 'hsk.rb');
    fs.writeFileSync(tapOutFile, formulaContent, 'utf-8');
    console.log(`Updated ${tapOutFile}`);
  }
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
