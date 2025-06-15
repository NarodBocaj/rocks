class Rocks < Formula
  desc "Command line tool for scraping Yahoo Finance stock information"
  homepage "https://github.com/NarodBocaj/rocks"
  url "https://github.com/NarodBocaj/rocks/archive/refs/tags/v0.1.6.tar.gz"
  sha256 "df2b4f5b005e30552f76d8b6cedaaf1034bd9cd85af0ca618054b43e7e990e7a"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", "."
    
    # Install the CSV files to pkgshare
    pkgshare.install "filtered_data/equities.csv"
    pkgshare.install "filtered_data/etfs.csv"
    
    # Move the actual binary to libexec
    libexec.install "target/release/rocks"
    
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
  end

  test do
    system "#{bin}/rocks", "--version"
  end
end 