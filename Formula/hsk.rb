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
      sha256 "b27f06693b2fa1a1b71a3505cb361579c2f029b2d06c67fa406b108b9c7dc9ed"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/fuyuan9/hsk-search/releases/download/v#{version}/hsk-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "cd0b3747b241a5c1455891a7f36595149fd416a7b1964b0beb32d4019bd79d53"
    else
      url "https://github.com/fuyuan9/hsk-search/releases/download/v#{version}/hsk-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "2b273e64d6318b5839ec16afffc41bdb49c0459286779d44bfe51b341999fbfa"
    end
  end

  def install
    bin.install "hsk"
  end

  test do
    assert_match "hsk #{version}", shell_output("#{bin}/hsk --version")
  end
end
