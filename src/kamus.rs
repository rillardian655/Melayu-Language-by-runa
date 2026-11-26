use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct PetaTersusun<V> {
    entri: Vec<(String, V)>,
    indeks: HashMap<String, usize>,
}

impl<V> PetaTersusun<V> {
    pub fn new() -> Self {
        Self {
            entri: Vec::new(),
            indeks: HashMap::new(),
        }
    }

    pub fn with_capacity(kapasiti: usize) -> Self {
        Self {
            entri: Vec::with_capacity(kapasiti),
            indeks: HashMap::with_capacity(kapasiti),
        }
    }

    pub fn len(&self) -> usize {
        self.entri.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entri.is_empty()
    }

    pub fn get(&self, kunci: &str) -> Option<&V> {
        self.indeks.get(kunci).map(|&i| &self.entri[i].1)
    }

    pub fn get_mut(&mut self, kunci: &str) -> Option<&mut V> {
        match self.indeks.get(kunci) {
            Some(&i) => Some(&mut self.entri[i].1),
            None => None,
        }
    }

    pub fn contains_key(&self, kunci: &str) -> bool {
        self.indeks.contains_key(kunci)
    }

    pub fn insert(&mut self, kunci: String, nilai: V) -> Option<V> {
        if let Some(&i) = self.indeks.get(&kunci) {
            return Some(std::mem::replace(&mut self.entri[i].1, nilai));
        }
        self.indeks.insert(kunci.clone(), self.entri.len());
        self.entri.push((kunci, nilai));
        None
    }

    pub fn remove(&mut self, kunci: &str) -> Option<V> {
        let i = self.indeks.remove(kunci)?;
        let (_, nilai) = self.entri.remove(i);
        for (k, _) in &self.entri[i..] {
            if let Some(j) = self.indeks.get_mut(k) {
                *j -= 1;
            }
        }
        Some(nilai)
    }

    pub fn ambil_nilai(&mut self) -> Vec<V> {
        self.indeks.clear();
        std::mem::take(&mut self.entri).into_iter().map(|(_, v)| v).collect()
    }

    pub fn clear(&mut self) {
        self.entri.clear();
        self.indeks.clear();
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.entri.iter().map(|(k, _)| k)
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entri.iter().map(|(_, v)| v)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.entri.iter_mut().map(|(_, v)| v)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &V)> {
        self.entri.iter().map(|(k, v)| (k, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut V)> {
        self.entri.iter_mut().map(|(k, v)| (&*k, v))
    }
}

impl<V> Default for PetaTersusun<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: fmt::Debug> fmt::Debug for PetaTersusun<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<V: PartialEq> PartialEq for PetaTersusun<V> {
    fn eq(&self, lain: &Self) -> bool {
        self.len() == lain.len() && self.iter().all(|(k, v)| lain.get(k) == Some(v))
    }
}

impl<V> FromIterator<(String, V)> for PetaTersusun<V> {
    fn from_iter<I: IntoIterator<Item = (String, V)>>(iter: I) -> Self {
        let mut peta = Self::new();
        peta.extend(iter);
        peta
    }
}

impl<V> Extend<(String, V)> for PetaTersusun<V> {
    fn extend<I: IntoIterator<Item = (String, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<V> IntoIterator for PetaTersusun<V> {
    type Item = (String, V);
    type IntoIter = std::vec::IntoIter<(String, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entri.into_iter()
    }
}

impl<'a, V> IntoIterator for &'a PetaTersusun<V> {
    type Item = (&'a String, &'a V);
    type IntoIter = std::iter::Map<std::slice::Iter<'a, (String, V)>, fn(&'a (String, V)) -> (&'a String, &'a V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entri.iter().map(|(k, v)| (k, v))
    }
}
