class Rocks < Formula
  desc "Command line tool for scraping Yahoo Finance stock information"
  homepage "https://github.com/NarodBocaj/rocks"
  url "https://github.com/NarodBocaj/rocks/archive/refs/tags/v0.1.7.tar.gz"
  sha256 "9067df175f98fd58e946c18d9314ae1d2f3c919124147044bc26f41d5add0f25"
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
      export ROCKS_DATA_DIR="#{pkgshare}"
      exec "#{libexec}/rocks" "$@"
    EOS
    chmod 0755, bin/"rocks"
  end

  test do
    system "#{bin}/rocks", "--version"
  end
end 