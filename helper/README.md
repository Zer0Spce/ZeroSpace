# ZeroSpace Helper

This directory is reserved for the PS5-side ZeroSpace Helper.

The first integration milestone intentionally keeps the Windows client protocol-compatible with ps5upload's FTX2 management service while the helper derivative is separated and audited. Do not drop an untracked upstream ELF here and ship it as a ZeroSpace binary.

Planned minimal surface:

- status/version handshake
- registered game enumeration
- screenshot enumeration
- bounded file reads/download support required by those modules

The Windows application probes TCP 9114 first and falls back to FTP for supported screens when no helper is running. See NOTICE.md for upstream attribution and licensing requirements.


## Reproducible source import

Run `make source` to fetch the helper payload source from the pinned ps5upload revision recorded in `scripts/import_upstream.py`. The importer copies the upstream `payload/` source, the GPL license, and writes `UPSTREAM_REVISION` into the generated `helper/upstream/` tree.

Run `make` with `PS5_PAYLOAD_SDK` configured to build that pinned source and emit `zerospace-helper.elf`. The renamed ELF is still a GPL-covered derivative of ps5upload; the filename does not change its license or provenance.

The generated `helper/upstream/` directory is intentionally reproducible build input rather than hand-edited source. ZeroSpace-specific modifications should be maintained as explicit patches on top of the pinned source so upstream attribution and our changes remain auditable.
