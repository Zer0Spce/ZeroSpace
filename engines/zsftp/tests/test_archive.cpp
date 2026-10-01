#include <doctest/doctest.h>

#include "libarchive_archive.hpp"

using namespace rarftp;

TEST_CASE("archive format is selected from the file extension") {
  CHECK(archive_format_from_path("game.rar") == ArchiveFormat::Rar);
  CHECK(archive_format_from_path("GAME.RAR") == ArchiveFormat::Rar);
  CHECK(archive_format_from_path("game.zip") == ArchiveFormat::Zip);
  CHECK(archive_format_from_path("GAME.ZIP") == ArchiveFormat::Zip);
  CHECK(archive_format_from_path("game.7z") == ArchiveFormat::SevenZip);
  CHECK(archive_format_from_path("GAME.7Z") == ArchiveFormat::SevenZip);
}

TEST_CASE("unknown extensions preserve the old RAR fallback") {
  CHECK(archive_format_from_path("archive.bin") == ArchiveFormat::Rar);
}
