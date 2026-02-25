<div align="center">

# Action Codex
### VERSION 1.3.3 - Oxide
Terminal-based code editor untuk workflow Rust dan proyek umum

</div>

## Ringkasan
Action Codex adalah editor terminal (TUI) yang fokus pada alur kerja cepat: buka file, edit, navigasi explorer, kelola dependency Cargo, dan format Rust tanpa keluar dari editor.

Fitur terbaru sampai `v1.3.3`:
- Explorer context menu: tambah file/folder, ubah nama file/folder, hapus file/folder.
- Popup klik kanan dengan indikator warna pada setiap aksi.
- Smart word wrap editor untuk baris panjang (dengan perilaku cursor yang tetap sinkron terhadap visual line).

Lihat histori lengkap pada [CHANGELOG.md](./CHANGELOG.md).

## Daftar Isi
- [Persyaratan](#persyaratan)
- [Build dan Menjalankan](#build-dan-menjalankan)
- [Argumen CLI](#argumen-cli)
- [Konsep Tampilan](#konsep-tampilan)
- [Shortcut Keyboard](#shortcut-keyboard)
- [Interaksi Mouse](#interaksi-mouse)
- [Explorer Context Menu](#explorer-context-menu)
- [Rust/Cargo Manager](#rustcargo-manager)
- [Smart Word Wrap](#smart-word-wrap)
- [Simpan File dan Signature Blob](#simpan-file-dan-signature-blob)
- [Troubleshooting](#troubleshooting)

## Persyaratan
Pastikan tool berikut tersedia di environment:
- `rustc` dan `cargo` (toolchain Rust)
- `rustfmt` (untuk format Rust dari dalam editor)
- `openssl` (dipakai saat pembuatan signature blob ketika simpan file)

Catatan:
- Aksi `cargo add` / `cargo remove` membutuhkan Cargo yang mendukung subcommand tersebut.
- Clipboard sistem menggunakan crate `arboard`; jika tidak tersedia backend clipboard OS, editor memakai fallback clipboard internal.

## Build dan Menjalankan
Menjalankan langsung mode development:
```bash
cargo run
```

Build release:
```bash
cargo build --release
```

Jalankan binary release:
```bash
./target/release/action-codex
```

## Argumen CLI
Buka satu atau beberapa file saat startup:
```bash
cargo run -- file.rs
cargo run -- src/main.rs Cargo.toml README.md
```

Perilaku:
- File yang ada akan dibuka pada tab terpisah.
- File yang belum ada akan diperlakukan sebagai tab file baru (akan dibuat saat disimpan).

## Konsep Tampilan
Layout utama terdiri dari:
- `Tabs`: daftar tab file aktif.
- `Explorer`: tree file/folder workspace saat ini.
- `Editor`: area editing utama.
- `Status Bar`: pesan status + posisi cursor.

Dialog/popup yang tersedia:
- Start menu (`Ctrl+M`)
- Help (`F1`)
- Save dialog (`Ctrl+S`)
- Search dialog (`Ctrl+F`)
- Editor context menu (klik kanan di editor)
- Explorer context menu (klik kanan di explorer)
- Rust/Cargo manager (`Ctrl+K`)

## Shortcut Keyboard
### Navigasi umum
- `Esc`: tutup dialog aktif, atau keluar aplikasi jika tidak ada dialog aktif.
- `Tab`: pindah fokus antar pane (Editor / Explorer / Tabs).
- `Ctrl+Tab`: next tab.
- `Shift+Tab`: previous tab.
- `Ctrl+B`: tampil/sembunyikan explorer.
- `Ctrl+T`: toggle tema gelap/terang.
- `F1`: buka/tutup bantuan.
- `Ctrl+M`: buka start menu.

### Tab
- `Ctrl+N`: buat tab baru.
- `Ctrl+W`: tutup tab aktif.
- Saat fokus `Tabs`:
- `Left` / `Right`: pindah tab aktif.
- `Enter`: kembali fokus ke editor.

### Editor
- `Ctrl+S`: buka dialog simpan.
- `Ctrl+F`: buka search keyword.
- `Ctrl+Shift+F`: format file Rust aktif.
- `Ctrl+A`: select all.
- `Ctrl+C`: copy selection atau baris aktif.
- `Ctrl+V`: paste.
- `Ctrl+X`: cut selection atau baris aktif.
- `Ctrl+Z`: undo.
- `Ctrl+Y` atau `Ctrl+Shift+Z`: redo.
- `Alt+Up` / `Alt+Down`: pindah blok/baris ke atas/bawah.
- `Shift+Arrow`: seleksi teks.

### Explorer
- Saat fokus `Explorer`:
- `Up` / `Down`: pindah selection.
- `Enter` pada folder: expand/collapse folder.
- `Enter` pada file: buka file di editor.

### Rust/Cargo Manager
- `Ctrl+K`: buka manager.
- Di manager mode menu: `Up/Down` pilih aksi, `Enter` jalankan, `Esc` tutup.

## Interaksi Mouse
- Klik tab untuk pindah tab aktif.
- Klik explorer untuk fokus/select item.
- Klik editor untuk fokus + set cursor.
- Drag mouse kiri di editor untuk seleksi.
- Scroll mouse di tab/explorer/editor untuk navigasi.
- Klik kanan di editor untuk context menu teks.
- Klik kanan di explorer untuk context menu file/folder.

## Explorer Context Menu
Saat klik kanan di explorer, menu menyesuaikan target:

### Jika target adalah file
- `Tambah Folder` (base: parent folder file)
- `Tambah File` (base: parent folder file)
- `Ubah Nama File`
- `Hapus File`

### Jika target adalah folder
- `Tambah Folder` (base: folder target)
- `Tambah File` (base: folder target)
- `Ubah Nama Folder`
- `Hapus Folder`

### Jika klik area kosong explorer
- Menu tampil dengan aksi pembuatan objek berbasis root explorer.

Validasi keamanan path:
- Path absolut tidak diizinkan.
- `..` (parent traversal) tidak diizinkan.
- Operasi rename/hapus hanya berlaku untuk objek di dalam root explorer.
- Root explorer tidak dapat dihapus/diubah nama.

## Rust/Cargo Manager
Aksi yang tersedia:
- `Format file Rust aktif`
- `Cari crate di crates.io`
- `Install crate (cargo add)`
- `Hapus crate (cargo remove)`
- `Tambah workspace member`
- `Hapus workspace member`

Catatan:
- Hasil command ditampilkan pada panel output manager.
- Untuk search crate, daftar hasil bisa dipilih lalu `Enter` untuk install/hapus.

## Smart Word Wrap
Mulai `v1.3.3`, editor menggunakan smart word wrap:
- Baris panjang dibungkus otomatis mengikuti lebar editor.
- Wrap diprioritaskan di batas whitespace (kata), fallback split aman jika perlu.
- Syntax highlight, selection highlight, dan pencarian tetap diterapkan pada hasil wrapping.
- Cursor visual diselaraskan ke line hasil wrap, sehingga posisi cursor tetap konsisten di tampilan.

## Simpan File dan Signature Blob
Saat menyimpan file (`Ctrl+S`):
- Jika parent directory belum ada, editor mencoba membuatnya otomatis.
- Editor membuat signature blob marker file tersembunyi (contoh: `.abc123...`) di direktori target.
- Kunci signing disimpan pada `.action-codex/ed25519_signing_key.pem`.

Jika `openssl` tidak tersedia, simpan file tetap berjalan tetapi pembuatan signature dapat gagal dan status bar akan menampilkan pesan error terkait signature.

## Troubleshooting
### `rustfmt` gagal
- Pastikan `rustfmt` tersedia:
```bash
rustfmt --version
```

### `cargo add` / `cargo remove` gagal
- Pastikan Cargo mendukung subcommand tersebut:
```bash
cargo add --help
cargo remove --help
```

### Signature gagal karena OpenSSL
- Pastikan `openssl` tersedia:
```bash
openssl version
```

### Clipboard sistem tidak berfungsi
- Editor otomatis fallback ke clipboard internal.
- Operasi copy/paste tetap bisa digunakan dalam sesi editor.

## Lisensi
Project ini menggunakan lisensi pada file [LICENSE](./LICENSE).

## SOCIAL
- [Telegram Group](https://t.me/xigmachat)
- [Telegram Channel](https://t.me/xigma98) 