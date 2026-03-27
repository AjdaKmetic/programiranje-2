use super::Zaporedje;

struct AritmeticnoZaporedje<T> {
    ime: String,
    a0: T,
    d: T,
    index: u64
}

impl<T> AritmeticnoZaporedje<T> {
    fn new(ime: &str, a0: T, d: T) -> Self {
        AritmeticnoZaporedje {
            ime: ime.to_string(),
            a0,
            d,
            index: 0
        }
    }
}

impl<T> Zaporedje<T> for AritmeticnoZaporedje<T> {
    fn name(&self) -> &str {
        todo!()
    }

    fn start(&self) -> T {
        todo!()
    }

    fn k_th(&self, k: u64) -> T {
        todo!()
    }

    fn contains(&self, value: &T) -> bool {
        todo!()
    }
}