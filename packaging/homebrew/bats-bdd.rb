class BatsBdd < Formula
  desc "BDD library for Rust that integrates with BATS (Bash Automated Testing System)"
  homepage "https://github.com/Fguedes90/bats-bdd"
  version "VERSION"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Fguedes90/bats-bdd/releases/download/vVERSION/bats-bdd-macos-aarch64.tar.gz"
      sha256 "MACOS_ARM_SHA_PLACEHOLDER"
    else
      url "https://github.com/Fguedes90/bats-bdd/releases/download/vVERSION/bats-bdd-macos-x86_64.tar.gz"
      sha256 "MACOS_X86_SHA_PLACEHOLDER"
    end
  end

  on_linux do
    url "https://github.com/Fguedes90/bats-bdd/releases/download/vVERSION/bats-bdd-linux-x86_64.tar.gz"
    sha256 "LINUX_SHA_PLACEHOLDER"
  end

  def install
    bin.install "bats-bdd"
  end

  test do
    assert_match "bats-bdd", shell_output("#{bin}/bats-bdd --help")
  end
end
