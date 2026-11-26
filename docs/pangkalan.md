# Panduan Rujukan Pangkalan Data Terbenam (`ambil "pangkalan"`)

Modul **`pangkalan`** menyediakan storan jadual dan dokumen JSON berstruktur yang patuh sepenuhnya kepada sandbox keselamatan projek Bahasa Melayu.

## 1. Operasi CRUD Asas

```melayu
ambil "pangkalan"

ada db = Pangkalan("data/kedai.json")
db.ciptaJadual("produk")

# Sisip rekod (auto-jana _id)
db.sisip("produk", {"nama": "Kopi Gayo", "kategori": "Minuman", "harga": 25000})
db.sisip("produk", {"nama": "Teh Tarik", "kategori": "Minuman", "harga": 12000})

# Carian
ada semua = db.cariSemua("produk")
ada minuman = db.cari("produk", {"kategori": "Minuman"})
ada kopi = db.cariSatu("produk", {"nama": "Kopi Gayo"})

# Kemas kini & Padam
db.kemaskini("produk", {"nama": "Kopi Gayo"}, {"harga": 28000})
db.padam("produk", {"nama": "Teh Tarik"})

tunjuk "Baki produk:", db.bilang("produk")
```

## 2. Pengurusan Jadual

* `db.senaraiJadual()`: Mengembalikan senarai nama jadual yang wujud.
* `db.padamJadual(nama)`: Memadam keseluruhan jadual dan mengemaskini fail cakera.
* Setiap operasi tulis (`ciptaJadual`, `sisip`, `kemaskini`, `padam`, `padamJadual`) disimpan ke cakera secara automatik.
