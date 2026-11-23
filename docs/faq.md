# Soalan Lazim (FAQ) Mengenai Bahasa MELAYU

Berikut merupakan jawapan kepada soalan yang kerap ditanya mengenai reka bentuk, pelaksanaan, dan masa hadapan projek **MELAYU**.

---

### 1. Mengapa membina bahasa pengaturcaraan berasaskan bahasa Melayu?
Objektif utama adalah meruntuhkan halangan bahasa untuk pemula dalam dunia sains komputer. Di rantau Nusantara (Indonesia, Malaysia, Brunei, Singapura), ramai pelajar menghadapi dua kesukaran serentak: memahami konsep algoritma dan memikirkan perbendaharaan kata Inggeris yang asing. MELAYU membolehkan fokus 100% diberikan kepada logik dan struktur pemikiran.

### 2. Apakah perbezaan perkataan mutabel `ada` dan pemalar `tetap`?
* `ada`: Mengisytiharkan pembolehubah yang boleh diubah nilainya kemudian (`ada skor = 10; skor = 20`).
* `tetap`: Mengisytiharkan pemalar malar yang dilindungi. Sebarang percubaan menulis semula nilai `tetap` akan ditolak oleh penganalisis semantik dengan mesej ralat yang jelas.

### 3. Mengapa blok ditutup dengan `selesai` dan bukannya tanda kurung kurawal `{}`?
Tanda kurung kurawal `{}` dan titik koma `;` sering kali menjadi punca ralat sintaks (*syntax error*) yang mengecewakan bagi pemula. Kata kunci `selesai` bersifat eksplisit, selaras dengan pertuturan harian, dan memudahkan pembacaan kod secara berstruktur (*structured readability*).

### 4. Adakah bahasa MELAYU selamat dijalankan?
Ya. Bermula daripada versi v0.1.1, jurubahasa MELAYU dilengkapi benteng keselamatan berbilang lapisan:
* Had rekursi bagi menyekat limpahan tindanan (*stack overflow protection*).
* Had pelaksanaan langkah lelaran bagi menyekat serangan Denial of Service (*infinite loop runaway limit*).
* Kotak pasir (*sandbox*) fail yang menyekat cubaan *path traversal* (`../`) ke luar direktori kerja selamat.

### 5. Bagaimana cara menulis OOP dalam MELAYU?
Bermula dari versi v0.3.0, anda boleh menggunakan struktur `kelas`, `bina` (pembina instans), `diri` (rujukan objek semasa), `warisi` (pewarisan daripada induk), dan `induk` (panggilan pembina/kaedah kelas induk).

### 6. Bagaimana cara menyumbang kepada projek ini?
Penyertaan sentiasa dialu-alukan! Anda boleh membuka *Issue* atau menghantar *Pull Request* di GitHub mengikut garis panduan dalam [CONTRIBUTING.md](../CONTRIBUTING.md).
