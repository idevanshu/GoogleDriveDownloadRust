

<h1>Google Drive File Downloader</h1>

<h3>Overview</h3>
<p><strong>A powerful CLI tool developed in Rust for downloading files from Google Drive.</strong> This tool efficiently extracts file IDs, generates direct download URLs, and provides real-time progress updates. It requires Rust and Cargo to run.</p>

<h3>Requirements</h3>
<ul>
    <li><strong>Rust (version 1.56 or later)</strong></li>
    <li><strong>Cargo (Rust's package manager and build system)</strong></li>
</ul>

<h3>Use Case</h3>
<p><strong>This tool is particularly well-suited for scenarios where a command-line interface (CLI) is the primary mode of interaction, such as on a Virtual Private Server (VPS) or systems with no graphical user interface (GUI).</strong> It allows users to efficiently download files from Google Drive without requiring a browser or GUI-based tools. Automated progress tracking and file naming features ensure that the download process is streamlined and manageable even in minimalistic environments.</p>


<h3>Quick Start</h3>
<ol>
    <li><strong>Clone the repository:</strong></li>
    <pre><code>git clone https://github.com/idevanshu/GoogleDriveDownloadRust.git</code></pre>

  <li><strong>Navigate to the project directory:</strong></li>
  <pre><code>cd GoogleDriveDownloadRust</code></pre>

  <li><strong>Build the project:</strong></li>
  <pre><code>cargo build</code></pre>

  <li><strong>Start the download:</strong></li>
   <p>Replace <code>&lt;drive_link&gt;</code> with the actual Google Drive link you wish to download. Then run the following command:</p>
    <pre><code>cargo run &lt;drive_link&gt;</code>
</ol>

