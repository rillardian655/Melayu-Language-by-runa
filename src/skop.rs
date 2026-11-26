use crate::interpreter::Variable;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::rc::Rc;

const HAD_LINEAR: usize = 8;

#[derive(Debug, Clone, Default)]
pub struct HasherCepat(u64);

impl Hasher for HasherCepat {
    fn finish(&self) -> u64 {
        self.0 ^ (self.0 >> 29)
    }

    fn write(&mut self, bait: &[u8]) {
        for blok in bait.chunks(8) {
            let mut n = [0u8; 8];
            n[..blok.len()].copy_from_slice(blok);
            self.0 = (self.0.rotate_left(5) ^ u64::from_le_bytes(n)).wrapping_mul(0x517c_c1b7_2722_0a95);
        }
    }
}

type Indeks = HashMap<Rc<str>, usize, BuildHasherDefault<HasherCepat>>;

#[derive(Debug, Clone, Default)]
pub struct Skop {
    entri: Vec<(Rc<str>, Variable)>,
    indeks: Option<Box<Indeks>>,
}

impl Skop {
    fn cari(&self, nama: &str) -> Option<usize> {
        match &self.indeks {
            Some(indeks) => indeks.get(nama).copied(),
            None => self.entri.iter().position(|(n, _)| &**n == nama),
        }
    }

    pub fn get(&self, nama: &str) -> Option<&Variable> {
        self.cari(nama).map(|i| &self.entri[i].1)
    }

    pub fn get_mut(&mut self, nama: &str) -> Option<&mut Variable> {
        self.cari(nama).map(|i| &mut self.entri[i].1)
    }

    pub fn insert(&mut self, nama: &str, pembolehubah: Variable) {
        match self.cari(nama) {
            Some(i) => self.entri[i].1 = pembolehubah,
            None => self.tambah(Rc::from(nama), pembolehubah),
        }
    }

    pub fn insert_rc(&mut self, nama: &Rc<str>, pembolehubah: Variable) {
        match self.cari(nama) {
            Some(i) => self.entri[i].1 = pembolehubah,
            None => self.tambah(Rc::clone(nama), pembolehubah),
        }
    }

    fn tambah(&mut self, nama: Rc<str>, pembolehubah: Variable) {
        if let Some(indeks) = &mut self.indeks {
            indeks.insert(Rc::clone(&nama), self.entri.len());
        }
        self.entri.push((nama, pembolehubah));
        if self.indeks.is_none() && self.entri.len() > HAD_LINEAR {
            self.indeks = Some(Box::new(
                self.entri
                    .iter()
                    .enumerate()
                    .map(|(i, (n, _))| (Rc::clone(n), i))
                    .collect(),
            ));
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Variable)> {
        self.entri.iter().map(|(n, v)| (&**n, v))
    }
}
