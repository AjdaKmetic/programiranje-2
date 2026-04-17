use super::Zaporedje;

struct KonstantnoZaporedje<T> {
    ime: String,
    a0: T
}

impl<T> KonstantnoZaporedje<T> {
    fn new(ime: &str, a0: T) -> Self {
        KonstantnoZaporedje { 
            ime: ime.to_string(), 
            a0 
        }
    }
}

impl<T: Clone + std::cmp::PartialEq> Zaporedje<T> for KonstantnoZaporedje<T> {
    fn name(&self) -> &str {
        &self.ime           // ker je isto kot pri aritmetičnem, bi lahko implementirali kar pri mod.rs
    }

    fn start(&self) -> T {
        self.a0.clone()
    }

    fn k_th(&self, _k: u64) -> T {      // _k ... namerno neuporabljena spremenljivka
        self.a0.clone()
    }

    fn contains(&self, value: &T) -> bool {
        self.a0 == *value
    }
}