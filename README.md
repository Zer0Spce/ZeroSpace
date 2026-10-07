<h1 align="center">ZeroSpace</h1>

<p align="center">
  <strong>The all-in-one Windows toolkit for managing, transferring, and interacting with your PlayStation 5.</strong>
</p>

<p align="center">
  Fast transfers • FTP management • Games • Captures • Payloads • PS5 Helper
</p>

<hr>

<h2>🚀 About ZeroSpace</h2>

<p>
ZeroSpace combines fast archive-to-PS5 transfers, a dual-pane FTP file manager,
payload tools, console management, game browsing, captures, and PS5 Helper
integration into one clean Windows application.
</p>

<p>
Built around the <strong>ZSFTP</strong> transfer engine, ZeroSpace is designed
to reduce the number of separate utilities needed when working with a PS5.
</p>

<h2>✨ Features</h2>

<ul>
  <li>🚀 <strong>Direct Archive Transfer</strong> — Send RAR, 7Z, and ZIP archives directly to your PS5 without creating huge temporary extracted files.</li>

  <li>📂 <strong>FTP Manager</strong> — WinSCP-style dual-pane PC ↔ PS5 file management.</li>

  <li>🎮 <strong>Games Library</strong> — Browse registered PS5 games with artwork and game information through the PS5 Helper.</li>

  <li>📸 <strong>Captures</strong> — Browse screenshots and videos directly from the console, preview supported images, and download captures to your PC.</li>

  <li>🔌 <strong>PS5 Connection</strong> — Dedicated console connection screen with live Helper status and automatic detection.</li>

  <li>🛰️ <strong>ZeroSpace PS5 Helper</strong> — Enables richer console integration beyond standard FTP.</li>

  <li>🚀 <strong>Payload Sender</strong> — Send ELF, BIN, JavaScript, Lua, and JAR payloads with automatic common-port selection.</li>

  <li>📁 <strong>Local File Manager</strong> — Browse and manage local files without leaving ZeroSpace.</li>

  <li>⚡ <strong>Live Transfer Information</strong> — View progress, speed, ETA, current file, and transfer status.</li>

  <li>🔐 <strong>Encrypted Archive Support</strong> — Password prompts for protected archives.</li>

  <li>🔄 <strong>Shared Connection Settings</strong> — PS5 connection information is shared between supported ZeroSpace modules.</li>

  <li>🌙 <strong>Light &amp; Dark Modes</strong></li>

  <li>🪟 <strong>Portable Windows Build</strong> — No installer required.</li>
</ul>

<hr>

<h2>⚡ ZSFTP — No Giant Temporary Extraction</h2>

<p>
One of ZeroSpace's core features is <strong>ZSFTP</strong>.
</p>

<p>
Instead of extracting a huge archive to your PC first and then uploading the
extracted files separately, ZeroSpace can process the archive as part of the
transfer workflow.
</p>

<blockquote>
  <strong>A 200 GB archive doesn't require another 200 GB of temporary free
  space just to transfer it.</strong>
</blockquote>

<p>
This is especially useful when working with large archives on PCs with limited
free storage.
</p>

<hr>

<h2>🔌 Two Ways to Work With Your PS5</h2>

<h3>FTP / ZSFTP</h3>

<p>
Handles standard filesystem operations and direct archive transfers.
</p>

<h3>ZeroSpace PS5 Helper</h3>

<p>
Provides richer console functionality used by features such as Games and
Captures.
</p>

<p>
The two systems are intentionally separated. Basic FTP and ZSFTP transfers
remain independent from the PS5 Helper.
</p>

<hr>

<h2>🎮 Games</h2>

<p>
When the PS5 Helper is available, ZeroSpace can automatically retrieve your
registered games and present them in an artwork-focused library.
</p>

<p>
The Games workspace includes game names, title IDs, artwork where available,
search, and automatic console refresh behavior.
</p>

<hr>

<h2>📸 Captures</h2>

<p>
Browse screenshots and videos stored on your PS5 directly from ZeroSpace.
</p>

<p>Supported functionality includes:</p>

<ul>
  <li>Screenshot thumbnails</li>
  <li>Image previews</li>
  <li>Video detection</li>
  <li>Individual downloads</li>
  <li>Bulk capture downloads</li>
  <li>Filtering and sorting</li>
  <li>Multi-selection</li>
</ul>

<hr>

<h2>📂 FTP Manager</h2>

<p>
ZeroSpace includes a dual-pane file manager designed for moving files between
your PC and PS5.
</p>

<p>
Navigate local and remote directories, upload and download files, create
folders, rename items, delete files, and manage transfers without leaving
ZeroSpace.
</p>

<hr>

<h2>🚀 Payloads</h2>

<p>
Send common PS5 payload formats directly from ZeroSpace.
</p>

<ul>
  <li><strong>ELF / BIN</strong> — Port 9021</li>
  <li><strong>JavaScript</strong> — Port 50000</li>
  <li><strong>Lua</strong> — Port 9026</li>
  <li><strong>JAR</strong> — Port 9025</li>
</ul>

<p>
ZeroSpace automatically selects the common port based on the payload type while
still allowing manual configuration.
</p>

<hr>

<h2>🧪 Connection Diagnostics</h2>

<p>
ZeroSpace can independently test the PS5 Helper and FTP connection.
</p>

<p>For example:</p>

<pre>
Helper Online · FTP Online
Helper Online · FTP Offline
Helper Offline · FTP Online
Helper Offline · FTP Offline
</pre>

<p>
This makes it much easier to determine which part of the PS5 connection needs
attention.
</p>

<hr>

<h2>🖥️ Platform</h2>

<p>
<strong>Windows x64</strong>
</p>

<p>
ZeroSpace is currently focused entirely on Windows. macOS and Linux builds are
not currently provided.
</p>

<h3>Portable Build</h3>

<p>No installer is required.</p>

<p>Keep the following files together:</p>

<pre>
zerospace.exe
zsftpcore.dll
</pre>

<hr>

<h2>⚠️ Project Status</h2>

<p>
ZeroSpace is under active development.
</p>

<p>
Some modules are still being expanded, and functionality requiring the PS5
Helper may depend on the environment running on your console.
</p>

<p>
Expect additional PS5 management features and workflow improvements in future
versions.
</p>

<hr>

<p align="center">
  <strong>ZeroSpace</strong><br>
  One workspace for your PS5.
</p>
