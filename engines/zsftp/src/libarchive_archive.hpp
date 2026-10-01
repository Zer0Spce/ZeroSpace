#pragma once

#include <cstddef>
#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <stdexcept>
#include <string>
#include <vector>

#include "rar_archive.hpp"

namespace rarftp {

enum class ArchiveFormat { Rar, Zip, SevenZip };

ArchiveFormat archive_format_from_path(const std::string& path);
const char* archive_format_name(ArchiveFormat format);

class LibArchiveError : public std::runtime_error {
 public:
  using std::runtime_error::runtime_error;
};

struct LibArchiveCallbacks {
  std::function<bool(const uint8_t* data, size_t size)> on_data;
};

class LibArchiveArchive {
 public:
  LibArchiveArchive(const std::string& path, ArchiveFormat format,
                    const std::optional<std::string>& password, LibArchiveCallbacks callbacks = {});
  ~LibArchiveArchive();
  LibArchiveArchive(const LibArchiveArchive&) = delete;
  LibArchiveArchive& operator=(const LibArchiveArchive&) = delete;

  bool next(ArchiveEntry& entry);
  void extract();
  void skip();
  bool aborted_by_callback() const;

  struct Impl;

 private:
  std::unique_ptr<Impl> impl_;
};

struct LibArchiveListing {
  std::vector<ArchiveEntry> entries;
  ArchiveFlags flags;
};

LibArchiveListing list_libarchive(const std::string& path, ArchiveFormat format,
                                  const std::optional<std::string>& password);
bool libarchive_error_is_password(const std::string& message);
std::string libarchive_version();

}  // namespace rarftp
