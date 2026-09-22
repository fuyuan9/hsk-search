# typed: false
# frozen_string_literal: true

class Hsk < Formula
  desc "High-precision HSK 3.0 Chinese Hanzi & Word search TUI client"
  homepage "https://github.com/fuyuan9/hsk-search"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/fuyuan9/hsk-search/releases/download/v#{version}/hsk-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "PENDING_RELEASE_SHA256"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/fuyuan9/hsk-search/releases/download/v#{version}/hsk-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PENDING_RELEASE_SHA256"
    else
      url "https://github.com/fuyuan9/hsk-search/releases/download/v#{version}/hsk-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PENDING_RELEASE_SHA256"
    end
  end

  def install
    bin.install "hsk"
  end

  test do
    assert_match "hsk #{version}", shell_output("#{bin}/hsk --version")
  end
end
