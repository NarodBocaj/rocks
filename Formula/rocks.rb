class Rocks < Formula
  desc "Command line tool for scraping Yahoo Finance stock information"
  homepage "https://github.com/NarodBocaj/rocks"
  url "https://github.com/NarodBocaj/rocks/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "" # You'll need to fill this in after creating the release
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", "."
    
    # Install the CSV files
    pkgshare.install "filtered_data/equities.csv"
    pkgshare.install "filtered_data/etfs.csv"
    
    # Create a wrapper script that sets the correct path for the CSV files
    (bin/"rocks").write <<~EOS
      #!/bin/bash
      exec "#{bin}/rocks" "$@"
    EOS
  end

  test do
    system "#{bin}/rocks", "--version"
  end
end 