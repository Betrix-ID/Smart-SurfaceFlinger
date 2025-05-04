# Date : 25 - 04 - 2025          
      [ Smart SurfaceFlinger - Nonroot ]
                   Author 
     Telegram @UnixeID | Github Betrix-ID
                version : 1.0.1
         ////////////////\\\\\\\\\\\\\\\\
            install with > Brevent
            
### ` Disclaimer `
Script ini dibuat untuk kebutuhan **pengaturan dan tuning performa grafis Android** melalui SurfaceFlinger. Tujuan utamanya adalah memberikan kontrol terhadap *duration timing* dan *refresh rate* sistem, yang berdampak langsung pada responsivitas tampilan.

Penggunaan script ini **ditujukan untuk pengguna berpengalaman**. Root access **kemungkinan besar dibutuhkan**, dan segala risiko ditanggung sendiri.

---

### ` Fungsi Utama `
Script ini menyesuaikan berbagai properti SurfaceFlinger secara otomatis maupun manual untuk mencapai **frekuensi refresh** yang diinginkan, seperti:

- `surface_flinger120()` — atur ke 120Hz (maksimal kelancaran visual).
- `surface_flinger90()` — atur ke 90Hz (seimbang performa dan baterai).
- `surface_flinger60()` — atur ke 60Hz (stabil dan hemat daya).
- `monitor_auto()` — ambil durasi langsung dari log SurfaceFlinger lalu sesuaikan secara otomatis.
- `kill()` — reset semua pengaturan ke default.

---

### ` Mekanisme Kerja `
Script menghitung nilai **duration (ns)** berdasarkan target Hz (misal: 1_000_000_000 / 120 untuk 120Hz), lalu menyebarkannya ke berbagai properti Android melalui `setprop`:

- `debug.sf.phaseoffset_app`
- `debug.sf.vsync_period`
- `debug.sf.duration_app`, dll.

Setelah itu, script akan menampilkan notifikasi (via `cmd notification`) untuk memberi tahu pengguna bahwa perubahan telah diterapkan.

---

### ` Efek Positif `
- Tampilan lebih **smooth & responsif**
- Mengurangi **lag grafis** di aplikasi berat
- Menyesuaikan FPS target dengan **akurasi tinggi**
- Potensi peningkatan **efisiensi daya**
- Kontrol penuh terhadap timing SurfaceFlinger

> Hasil bisa bervariasi tergantung device, ROM, dan patch keamanan.  
> Direkomendasikan untuk perangkat rooted dan paham risiko tuning grafis.

---

### ` Rekomendasi Penggunaan `
- Cocok untuk **pengguna advanced** yang ingin mengoptimalkan refresh rate.
- Sangat ideal untuk:  
  `> Gaming performance`  
  `> UI smoothness`  
  `> Battery vs Performance tuning`

### Note :
    $ shell script ini tidak meyebabkan Aplikasi mana pun Crash dan lag dikerenakan sudah saya susun seteliti mungkin agar tidak bedampak pada aplikasi manapun dill..
                        
### Inspired Script :
    $ @RiProG ( Muhammad Rizki )

### Penanggung Jawab 100% By :
- @UnixeID | Telegram
- Betrix-ID   | Github

### Channel Telegram and Github :
- https://t.me/Yeye_PID
- https://github.com/Betrix-ID

### install :
Ekstrak dulu ' zip ' Ke internal
Lalu Jalankan Perintah down Here :
sh /sdcard/int/run.sh --help

### Uninstall : Rebot youre phone ,matikan ulang hp kalian dill*

### Disclemoer Efek :
    $ Maaf jika script nya tidak begitu Berefek pada device kalian kerena tiap Device, Kernel And Versi Andorid Berbeda-beda Susah membuat script nya bekerja 100% dill..

### ⚠️ Especially for those who want to use my script in your Module, please contact me & add credit @UnixeID
