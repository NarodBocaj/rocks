class Rocks < Formula
  desc "Command line tool for scraping Yahoo Finance stock information"
  homepage "https://github.com/NarodBocaj/rocks"
  url "https://github.com/NarodBocaj/rocks/archive/refs/tags/v0.1.4.tar.gz"
  sha256 "f7925689f807f8304bcb5539f5ac4423de7c936c6d883fb10b1d3db7a261d777"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", "."
    
    # Install the CSV files to pkgshare
    pkgshare.install "filtered_data/equities.csv"
    pkgshare.install "filtered_data/etfs.csv"
    
    # Create a wrapper script that sets the correct path for the CSV files
    (bin/"rocks").write <<~EOS
      #!/bin/bash
      set -e  # Exit on error
      echo "Wrapper script starting..."
      echo "Setting ROCKS_DATA_DIR to #{pkgshare}"
      export ROCKS_DATA_DIR="#{pkgshare}"
      echo "ROCKS_DATA_DIR is now: $ROCKS_DATA_DIR"
      echo "About to execute: #{bin}/rocks"
      if [ ! -f "#{bin}/rocks" ]; then
        echo "Error: Binary not found at #{bin}/rocks"
        exit 1
      fi
      exec "#{bin}/rocks" "$@"
    EOS
    chmod 0755, bin/"rocks"
  end

  test do
    system "#{bin}/rocks", "--version"
  end
end 