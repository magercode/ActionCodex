# CHANGELOG HISTORY

v1.0.0
- masih dalam versi uji coba
- masih menggunakan c# (performa lambat)
- sumber kode terbuka bebas untuk komunitas bisa cek di githubnya magercode 

v1.1.0 (Oxide)
- Versi minor dengan perubahan besar dimana semua nya dibuat ulang dengan rust + dukungan syntax highlight untuk Rust
- dukungan untuk perangkat cross platform 
- penambahan fitur 
1. Tree Explorer (sidebar)

v1.2.0 (Masih edisi Oxide)
- Pembaruan syntax highlighting yang canggih mirip dengan vs code
- Penambahan fitur shortcut untuk copy dan paste dan juga undo redo dan cut
- Fitur block & select dan juga menu dialog klik kanan pada editor

v1.3.0 (Oxide)
- Penambahan fitur paket manager yang interaktif
- Penambahan fitur formatter pintar
- Penambahan fitur workspace manager (masih dalam uji coba)

v1.3.1 (Oxide)
- Penambahan context menu klik kanan di Explorer
- Penambahan fitur tambah folder baru dari Explorer
- Penambahan fitur tambah file baru dari Explorer (termasuk lewat klik kanan pada folder)
- Folder pada Explorer sekarang bisa diklik kanan dengan opsi hapus folder
- Patch bug: klik area kosong Explorer tidak lagi otomatis memilih item terakhir
- Patch bug: state expand/collapse Explorer lebih stabil setelah create/delete folder
- Patch bug: simpan file sekarang otomatis membuat direktori parent jika belum ada

v1.3.2 (Oxide)
- Popup klik kanan sekarang memiliki indikator warna latar pada tombol pilihan aksi
- Explorer object (file/folder) sekarang punya opsi ubah nama
- Explorer object (file/folder) sekarang punya opsi hapus
- Patch bug layout editor untuk teks panjang dengan pemotongan aman + indikator '~' (show more)

v1.3.3 (Oxide)
- Penambahan fitur smart word wrap pada editor untuk baris teks panjang
- Cursor editor kini mengikuti visual line hasil wrapping agar navigasi tetap konsisten
- Patch stabilitas render teks panjang untuk mencegah kerusakan layout/panic
