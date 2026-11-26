use std::cell::Cell;

thread_local! {
    static SEMPADAN: Cell<usize> = const { Cell::new(0) };
}

#[inline(never)]
fn alamat_semasa() -> usize {
    let penanda = 0u8;
    std::hint::black_box(&penanda) as *const u8 as usize
}

pub fn tetapkan_saiz(saiz: usize) {
    let simpanan = saiz / 8;
    let sempadan = alamat_semasa().saturating_sub(saiz - simpanan);
    SEMPADAN.with(|s| s.set(sempadan));
}

pub fn hampir_penuh() -> bool {
    let sempadan = SEMPADAN.with(|s| s.get());
    sempadan != 0 && alamat_semasa() < sempadan
}
