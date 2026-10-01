# ZeroSpace architecture

ZeroSpace is a modular Windows desktop application. The existing ZSFTP transfer engine remains the focused archive-to-FTP component rather than being rewritten into unrelated features.

## Modules

- **Transfer** — ZSFTP-backed RAR/ZIP/7z to FTP streaming. Default module.
- **FTP Manager** — dual-pane PC to PS5 file manager.
- **Files** — PS5 filesystem workflows.
- **Games** — installed game management.
- **Packages** — package workflows.
- **Payloads** — payload management and sending.
- **Captures** — screenshot/video management.
- **Power** — console power actions.
- **System** — console/system information.
- **Tools** — utilities.
- **Settings** — shared connection and app preferences.

## ZSFTP boundary

The desktop shell talks to ZSFTP through a narrow adapter. The ZSFTP 1.2 source lives in `engines/zsftp/`. ZeroSpace's Rust backend links to the engine's existing `zsftpcore` C API and owns the transfer job lifecycle directly. The legacy standalone ZSFTP GUI is intentionally not carried forward.

This keeps archive extraction and FTP streaming isolated from future modules and reduces the chance of regressions in the proven transfer path.

## CI policy

Do not spend Actions minutes on every small edit. CI, when added, should be Windows x64 only, cancel superseded runs, cache dependencies, and reserve packaging for releases or manual runs.