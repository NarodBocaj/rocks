class Rocks < Formula
  desc "Command line tool for scraping Yahoo Finance stock information"
  homepage "https://github.com/NarodBocaj/rocks"
  url "https://github.com/NarodBocaj/rocks/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "fa47d86a879c933c8a5fa50c2f3a23fc5aaca13f6250653ceb73581d6d77ff9d"
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
      echo "About to execute: #{libexec}/rocks"
      if [ ! -f "#{libexec}/rocks" ]; then
        echo "Error: Binary not found at #{libexec}/rocks"
        exit 1
      fi
      exec "#{libexec}/rocks" "$@"
    EOS
    chmod 0755, bin/"rocks"

    # Move the actual binary to libexec
    libexec.install "target/release/rocks"
  end

  test do
    system "#{bin}/rocks", "--version"
  end
end 