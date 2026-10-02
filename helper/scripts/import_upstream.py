#!/usr/bin/env python3
"""Fetch the GPL-covered ZeroSpace Helper source from the pinned ps5upload revision.

This intentionally imports SOURCE, never a prebuilt ps5upload ELF. The resulting
helper/upstream tree remains GPL-covered and must be distributed with its license
and modification notices.
"""
from pathlib import Path
import io, os, shutil, tarfile, urllib.request

UPSTREAM_REPO = "phantomptr/ps5upload"
UPSTREAM_REV = "a364f7a473bfd6aa5582e7474bfce54fb88a0f5a"
URL = f"https://github.com/{UPSTREAM_REPO}/archive/{UPSTREAM_REV}.tar.gz"
ROOT = Path(__file__).resolve().parents[1]
DEST = ROOT / "upstream"
TMP = ROOT / ".upstream-import"

def main():
    if TMP.exists(): shutil.rmtree(TMP)
    if DEST.exists(): shutil.rmtree(DEST)
    TMP.mkdir(parents=True)
    print(f"Fetching {UPSTREAM_REPO}@{UPSTREAM_REV}...")
    with urllib.request.urlopen(URL, timeout=60) as r:
        data = r.read()
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as tf:
        tf.extractall(TMP, filter="data")
    src = next(TMP.iterdir())
    shutil.copytree(src / "payload", DEST / "payload")
    shutil.copy2(src / "LICENSE", DEST / "LICENSE")
    (DEST / "UPSTREAM_REVISION").write_text(UPSTREAM_REV + "\n", encoding="utf-8")
    shutil.rmtree(TMP)
    print(f"Imported helper source into {DEST}")

if __name__ == "__main__":
    main()
