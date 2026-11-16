# MELAYU Language Support untuk VS Code

Ekstensi rasmi Visual Studio Code bagi bahasa pengaturcaraan **MELAYU** (`.ml`).

## Ciri-ciri Utama

- **Pewarnaan Sintaks Penuh (Syntax Highlighting)**: Menyokong semua kata kunci bahasa Melayu (`ada`, `tetap`, `jika`, `selesai`, `kerja`, `ambil`), jenis data, rentetan, nombor, dan operator.
- **Koleksi Snippets Pintar**: Menaip pantas untuk blok `kerja`, `jika`, `selagi`, `untuk`, `cuba...tangkap`, dan `ambil`.
- **Konfigurasi Bahasa**: Autolengkap tanda kurung `[]`, `{}`, `""` dan sokongan lipatan kod (*code folding*).
- **Peraturan Indentasi Pintar**: Indentasi automatik selepas kata kunci blok dan pengurangan indentasi pada `selesai`.

## Cara Pemasangan Tempatan

### Pilihan 1: Salin ke Direktori Ekstensi VS Code

Salin folder `editors/vscode` ke dalam direktori ekstensi editor anda:

```bash
# Untuk Linux & macOS:
cp -r editors/vscode ~/.vscode/extensions/melayu-language-support

# Untuk VSCodium / Cursor:
cp -r editors/vscode ~/.cursor/extensions/melayu-language-support
# atau
cp -r editors/vscode ~/.vscode-oss/extensions/melayu-language-support
```

Selepas menyalin, mulakan semula (restart) atau muat semula tetingkap VS Code (`Developer: Reload Window`).

### Pilihan 2: Bina Pakej .vsix

Jika anda mempunyai `npm` atau `npx`:

```bash
cd editors/vscode
npx @vscode/vsce package
```

Fail `melayu-language-support-0.1.2.vsix` akan dijana dan boleh dipasang melalui menu VS Code:
`Extensions -> ... (Views and More Actions) -> Install from VSIX...`.
