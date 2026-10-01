# ZeroSpace

**The all-in-one Windows toolkit for PS5 file management and direct archive-to-FTP transfers.**

ZeroSpace grows the proven ZSFTP transfer workflow into a modular desktop application. Transfer remains the core feature; additional tools live in separate modules so they do not complicate the archive streaming engine.

## Foundation

- Windows-first Tauri 2 + React + TypeScript desktop shell
- Transfer tab designed around the existing ZSFTP engine
- Sidebar shells for FTP Manager, Files, Games, Packages, Payloads, Captures, Power, System, Tools and Settings
- No ps5upload Stream & Install clone
- No GPL ps5upload source copied into this repository
- CI intentionally omitted during early iteration to avoid wasting GitHub Actions minutes

## Transfer

The Transfer module is for RAR, ZIP and 7z archive contents sent directly to a PS5 FTP server without first creating a second fully extracted copy on the PC. The current foundation includes the desktop-to-ZSFTP adapter; the existing ZSFTP engine itself still needs to be imported/bundled from the original PS5 ZeroSpace FTP repository.

## Development

```bash
npm install
npm run tauri dev
```

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the module layout and CI policy.

## Credits

ZeroSpace builds on ZSFTP / PS5 ZeroSpace FTP, which is derived from the original `rarftp` project by Rui Nelson. Applicable license and attribution notices will be preserved as the engine is imported.