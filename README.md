# ZeroSpace

**The all-in-one Windows toolkit for PS5 file management and direct archive-to-FTP transfers.**

ZeroSpace grows the proven ZSFTP transfer workflow into a modular desktop application. Transfer remains the core feature; additional tools live in separate modules so they do not complicate the archive streaming engine.

## Foundation

- Windows-first Tauri 2 + React + TypeScript desktop shell
- Transfer tab designed around the existing ZSFTP engine
- Sidebar shells for FTP Manager, Files, Games, Packages, Payloads, Captures, Power, System, Tools and Settings
- No ps5upload Stream & Install clone
- No GPL ps5upload source copied into this repository
- ZSFTP 1.2 source integrated as the native Transfer engine
- CI intentionally omitted during early iteration to avoid wasting GitHub Actions minutes

## Transfer

The Transfer module is for RAR, ZIP and 7z archive contents sent directly to a PS5 FTP server without first creating a second fully extracted copy on the PC. The ZSFTP source is now integrated under `engines/zsftp/`. ZeroSpace links directly to its C API (`zsftpcore`) so Transfer can start, poll, cancel and close native transfer jobs without launching a separate CLI process.

## Development

```bash
npm install
npm run tauri dev
```

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the module layout and CI policy.

## Credits

ZeroSpace builds on ZSFTP / PS5 ZeroSpace FTP, which is derived from the original `rarftp` project by Rui Nelson. Applicable license and attribution notices will be preserved as the engine is imported.

## PS5 Helper

ZeroSpace's richer console modules use an on-console helper over the FTX2 management protocol on TCP 9114. During the current integration stage, the helper is based on the GPLv3 ps5upload payload by PhantomPtr and is treated as a separately attributed GPL component. The Windows UI and ZSFTP transfer engine remain separate modules.

The helper is loaded through the PS5 ELF loader (normally TCP 9021) and remains resident until the console reboots or enters rest mode. Games and Captures prefer the helper when it is available and fall back to FTP where supported.

Upstream helper source and license: phantomptr/ps5upload (GPL-3.0). ZeroSpace will preserve the upstream copyright, license, source availability and modification notices for any distributed derivative helper.
