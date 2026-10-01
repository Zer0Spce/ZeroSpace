#include "libarchive_archive.hpp"

#include <algorithm>
#include <cctype>
#include <cstring>
#include <vector>

#ifdef _WIN32
#include <windows.h>
#endif

#include <archive.h>
#include <archive_entry.h>

namespace rarftp {
namespace {

std::string lower(std::string s) {
  std::transform(s.begin(), s.end(), s.begin(), [](unsigned char c) { return static_cast<char>(std::tolower(c)); });
  return s;
}

std::string error_text(struct archive* a, const std::string& prefix) {
  const char* detail = archive_error_string(a);
  return detail && *detail ? prefix + ": " + detail : prefix;
}

void check(int rc, struct archive* a, const std::string& what) {
  if (rc < ARCHIVE_WARN) throw LibArchiveError(error_text(a, what));
}

#ifdef _WIN32
std::wstring utf8_to_wide(const std::string& s) {
  if (s.empty()) return {};
  const int size = MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, s.data(),
                                       static_cast<int>(s.size()), nullptr, 0);
  if (size <= 0) throw LibArchiveError("archive path is not valid UTF-8");
  std::wstring out(static_cast<size_t>(size), L'\\0');
  if (MultiByteToWideChar(CP_UTF8, MB_ERR_INVALID_CHARS, s.data(), static_cast<int>(s.size()),
                          out.data(), size) != size) {
    throw LibArchiveError("cannot convert archive path to Windows Unicode");
  }
  return out;
}
#endif

void enable_format(struct archive* a, ArchiveFormat format) {
  int rc = ARCHIVE_FATAL;
  switch (format) {
    case ArchiveFormat::Zip: rc = archive_read_support_format_zip(a); break;
    case ArchiveFormat::SevenZip: rc = archive_read_support_format_7zip(a); break;
    case ArchiveFormat::Rar: throw LibArchiveError("internal error: RAR must use UnRAR");
  }
  check(rc, a, "cannot enable archive format");
  check(archive_read_support_filter_all(a), a, "cannot enable archive filters");
}

ArchiveEntry make_entry(struct archive_entry* raw) {
  ArchiveEntry e;
  const char* name = archive_entry_pathname_utf8(raw);
  if (!name) name = archive_entry_pathname(raw);
  e.name = name ? name : "";
  e.size = archive_entry_size_is_set(raw) && archive_entry_size(raw) > 0
               ? static_cast<uint64_t>(archive_entry_size(raw)) : 0;
  e.mtime = archive_entry_mtime_is_set(raw) ? static_cast<int64_t>(archive_entry_mtime(raw)) : 0;
  e.encrypted = archive_entry_is_data_encrypted(raw) == 1 || archive_entry_is_metadata_encrypted(raw) == 1;

  if (archive_entry_hardlink(raw)) e.kind = EntryKind::Hardlink;
  else if (archive_entry_symlink(raw)) e.kind = EntryKind::Symlink;
  else {
    const auto type = archive_entry_filetype(raw);
    if (type == AE_IFDIR) e.kind = EntryKind::Directory;
    else if (type == AE_IFREG || type == 0) e.kind = EntryKind::File;
    else e.kind = EntryKind::Symlink;
  }
  return e;
}

}  // namespace

struct LibArchiveArchive::Impl {
  struct archive* handle = nullptr;
  LibArchiveCallbacks callbacks;
  bool aborted = false;
  bool current = false;

  ~Impl() { if (handle) archive_read_free(handle); }
};

ArchiveFormat archive_format_from_path(const std::string& path) {
  const std::string p = lower(path);
  if (p.size() >= 4 && p.compare(p.size() - 4, 4, ".zip") == 0) return ArchiveFormat::Zip;
  if (p.size() >= 3 && p.compare(p.size() - 3, 3, ".7z") == 0) return ArchiveFormat::SevenZip;
  return ArchiveFormat::Rar;
}

const char* archive_format_name(ArchiveFormat format) {
  switch (format) {
    case ArchiveFormat::Rar: return "RAR";
    case ArchiveFormat::Zip: return "ZIP";
    case ArchiveFormat::SevenZip: return "7z";
  }
  return "archive";
}

LibArchiveArchive::LibArchiveArchive(const std::string& path, ArchiveFormat format,
                                     const std::optional<std::string>& password,
                                     LibArchiveCallbacks callbacks)
    : impl_(std::make_unique<Impl>()) {
  impl_->callbacks = std::move(callbacks);
  impl_->handle = archive_read_new();
  if (!impl_->handle) throw LibArchiveError("cannot allocate archive reader");
  enable_format(impl_->handle, format);
  if (password) check(archive_read_add_passphrase(impl_->handle, password->c_str()), impl_->handle,
                      "cannot set archive password");
#ifdef _WIN32
  const std::wstring wide_path = utf8_to_wide(path);
  check(archive_read_open_filename_w(impl_->handle, wide_path.c_str(), 1024 * 1024), impl_->handle,
        "cannot open archive");
#else
  check(archive_read_open_filename(impl_->handle, path.c_str(), 1024 * 1024), impl_->handle,
        "cannot open archive");
#endif
}

LibArchiveArchive::~LibArchiveArchive() = default;

bool LibArchiveArchive::next(ArchiveEntry& entry) {
  struct archive_entry* raw = nullptr;
  const int rc = archive_read_next_header(impl_->handle, &raw);
  if (rc == ARCHIVE_EOF) { impl_->current = false; return false; }
  check(rc, impl_->handle, "cannot read archive header");
  impl_->current = true;
  entry = make_entry(raw);
  return true;
}

void LibArchiveArchive::extract() {
  if (!impl_->current) throw LibArchiveError("internal error: no current archive entry");
  const void* data = nullptr;
  size_t size = 0;
  la_int64_t offset = 0;
  while (true) {
    const int rc = archive_read_data_block(impl_->handle, &data, &size, &offset);
    if (rc == ARCHIVE_EOF) break;
    check(rc, impl_->handle, "cannot decompress archive entry");
    if (size && impl_->callbacks.on_data &&
        !impl_->callbacks.on_data(static_cast<const uint8_t*>(data), size)) {
      impl_->aborted = true;
      throw LibArchiveError("archive extraction cancelled");
    }
  }
  impl_->current = false;
}

void LibArchiveArchive::skip() {
  if (!impl_->current) return;
  check(archive_read_data_skip(impl_->handle), impl_->handle, "cannot skip archive entry");
  impl_->current = false;
}

bool LibArchiveArchive::aborted_by_callback() const { return impl_->aborted; }

LibArchiveListing list_libarchive(const std::string& path, ArchiveFormat format,
                                  const std::optional<std::string>& password) {
  LibArchiveListing out;
  // 7z can group files into shared compression folders. Treat it conservatively
  // as solid so skipped entries are decoded when necessary.
  out.flags.solid = format == ArchiveFormat::SevenZip;
  LibArchiveArchive archive(path, format, password);
  ArchiveEntry entry;
  while (archive.next(entry)) {
    if (format == ArchiveFormat::SevenZip && entry.encrypted) {
      throw LibArchiveError("7z encryption is not supported by the current backend");
    }
    out.entries.push_back(entry);
    archive.skip();
  }
  return out;
}

bool libarchive_error_is_password(const std::string& message) {
  const std::string m = lower(message);
  return m.find("passphrase") != std::string::npos || m.find("password") != std::string::npos ||
         m.find("encrypted") != std::string::npos;
}

std::string libarchive_version() {
  const char* v = archive_version_string();
  return v ? v : "libarchive";
}

}  // namespace rarftp
