use crate::interpreter::Value;
use crate::kamus::PetaTersusun;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
pub struct SenaraiNilai(Vec<Value>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PetaNilai(PetaTersusun<Value>);

impl PetaNilai {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<Vec<Value>> for SenaraiNilai {
    fn from(isi: Vec<Value>) -> Self {
        Self(isi)
    }
}

impl IntoIterator for SenaraiNilai {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;

    fn into_iter(mut self) -> Self::IntoIter {
        std::mem::take(&mut self.0).into_iter()
    }
}

impl Deref for SenaraiNilai {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SenaraiNilai {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for PetaNilai {
    type Target = PetaTersusun<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PetaNilai {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for SenaraiNilai {
    fn drop(&mut self) {
        if self.0.iter().any(ada_anak) {
            lepaskan(std::mem::take(&mut self.0));
        }
    }
}

impl Drop for PetaNilai {
    fn drop(&mut self) {
        if self.0.values().any(ada_anak) {
            lepaskan(self.0.ambil_nilai());
        }
    }
}

fn ada_anak(nilai: &Value) -> bool {
    matches!(
        nilai,
        Value::Senarai(_) | Value::Kamus(_) | Value::Objek(_) | Value::Induk(_)
    )
}

fn lepaskan(mut tugas: Vec<Value>) {
    while let Some(nilai) = tugas.pop() {
        match nilai {
            Value::Senarai(rc) => {
                if let Ok(sel) = Rc::try_unwrap(rc) {
                    tugas.append(&mut sel.into_inner().0);
                }
            }
            Value::Kamus(rc) => {
                if let Ok(sel) = Rc::try_unwrap(rc) {
                    tugas.append(&mut sel.into_inner().0.ambil_nilai());
                }
            }
            Value::Objek(rc) => {
                if let Ok(objek) = Rc::try_unwrap(rc) {
                    if let Ok(sel) = Rc::try_unwrap(objek.sifat) {
                        tugas.append(&mut sel.into_inner().0.ambil_nilai());
                    }
                }
            }
            Value::Induk(rc) => {
                if let Ok(induk) = Rc::try_unwrap(rc) {
                    if let Ok(sel) = Rc::try_unwrap(induk.instans_sifat) {
                        tugas.append(&mut sel.into_inner().0.ambil_nilai());
                    }
                }
            }
            _ => {}
        }
    }
}
