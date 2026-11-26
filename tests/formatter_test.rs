use std::fs;
use std::process::Command;

#[test]
fn test_cli_melayu_format_dan_semak() {
    let exe = env!("CARGO_BIN_EXE_melayu");
    let test_file = std::env::temp_dir().join(format!("melayu_fmt_test_{}.ml", std::process::id()));
    let messy = "jika betul\nada x=1\njika tidak\nada x=2\nselesai\ntunjuk x\n";
    fs::write(&test_file, messy).unwrap();

    // 1. --semak mesti gagal pada kod yang belum diformatkan
    let check_before = Command::new(exe)
        .args(["format", "--semak"])
        .arg(&test_file)
        .output()
        .unwrap();
    assert!(!check_before.status.success(), "Format --semak sepatutnya gagal sebelum diformatkan");

    // 2. Jalankan format
    let fmt_out = Command::new(exe)
        .arg("format")
        .arg(&test_file)
        .output()
        .unwrap();
    assert!(fmt_out.status.success(), "melayu format gagal: {:?}", fmt_out);

    let formatted = fs::read_to_string(&test_file).unwrap();
    assert_eq!(
        formatted,
        "jika betul\n    ada x=1\njika tidak\n    ada x=2\nselesai\ntunjuk x\n"
    );

    // 3. --semak selepas format mesti lulus
    let check_after = Command::new(exe)
        .args(["format", "--semak"])
        .arg(&test_file)
        .output()
        .unwrap();
    assert!(check_after.status.success(), "Format --semak sepatutnya lulus selepas diformatkan");

    let _ = fs::remove_file(&test_file);
}
