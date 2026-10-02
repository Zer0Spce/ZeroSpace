# ZeroSpace PS5 Helper

ZeroSpace uses a PS5-side helper for console-aware features such as registered game discovery and capture enumeration.

## Upstream

The helper integration is derived from / interoperates with the ps5upload payload by PhantomPtr:
https://github.com/phantomptr/ps5upload

ps5upload is licensed under GNU GPL version 3 (or later, as stated upstream). Any ZeroSpace helper derivative that incorporates GPL-covered ps5upload payload code must remain GPL-compatible, retain the applicable notices, and make corresponding source available when distributed.

## ZeroSpace policy

- The helper is a distinct PS5-side component; it is not the ZSFTP engine.
- ZeroSpace will identify modified helper builds as ZeroSpace Helper rather than representing them as upstream ps5upload releases.
- Upstream attribution and modification notices must remain with distributed helper source/binaries.
- Until the helper source and build pipeline are checked into this repository, ZeroSpace does not bundle or silently redistribute an upstream helper ELF.

## Runtime contract

- PS5 ELF loader: TCP 9021 by default.
- Helper management service: TCP 9114.
- FTX2 transfer service: TCP 9113 where enabled.
- The helper is expected to leave memory on reboot/rest mode and must then be loaded again.

This file is a project notice, not a replacement for the GPL license text that accompanies GPL-covered helper source.
