# ZeroSpace Helper

This directory is reserved for the PS5-side ZeroSpace Helper.

The first integration milestone intentionally keeps the Windows client protocol-compatible with ps5upload's FTX2 management service while the helper derivative is separated and audited. Do not drop an untracked upstream ELF here and ship it as a ZeroSpace binary.

Planned minimal surface:

- status/version handshake
- registered game enumeration
- screenshot enumeration
- bounded file reads/download support required by those modules

The Windows application probes TCP 9114 first and falls back to FTP for supported screens when no helper is running. See NOTICE.md for upstream attribution and licensing requirements.
