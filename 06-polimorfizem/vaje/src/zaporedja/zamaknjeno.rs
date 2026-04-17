use super::Zaporedje;

/* 
struct ZamaknjenoZaporedje<'a, T> { // zakaj z Z? ker Z predstavlja zaporedje
    zaporedje: &'a dyn Zaporedje<T>,
    n: u64
}
*/

// dyn ... dynamic dispatch

struct ZamaknjenoZaporedje<Z> {
    zaporedje: Z,
    n: u64 // za koliko je zaporedje zamaknjeno
}

impl<Z> ZamaknjenoZaporedje<Z> {
    fn new(zaporedje: Z, n: u64) -> Self {
        ZamaknjenoZaporedje { 
            zaporedje, 
            n
        }
    }
}

impl<T: PartialEq, Z: Zaporedje<T>> Zaporedje<T> for ZamaknjenoZaporedje<Z> {    // ZamaknjenoZaporedje je samo okvir za naše zaporedje Z, zato tu pri Zaporedju pišemo T
    fn name(&self) -> &str {
        self.zaporedje.name() // vprašanje: zakaj tu ne moremo pisati ime()?
    }

    fn start(&self) -> T {
        self.zaporedje.k_th(self.n)
    }

    fn k_th(&self, k: u64) -> T { // zanima nas k-ti člen zamaknjenega zaporedja
        self.zaporedje.k_th(self.n + k)
    }

    fn contains(&self, value: &T) -> bool {
        let mut k = 0;
        loop {
            if self.k_th(k) == *value {
                return true;
            }
            k += 1;
        }
    } 

}

// da bo ZamaknjenoZaporedje zaporedje, mora biti Z zaporedje: Z: Zaporedje<T>

// Izpit!!!
// 1. naloga: string, številke, stack
// 2. naloga: implementacija: kompletna implementacija, enum -> zanjga definiramo metode, zadostimo traitom (to kot smo mel zgorej: T: PartialEq)
// 3. naloga: življenska doba (Rust zajoka, mi samo popravimo kot reče Rust)

// dovoljeno vse, razen umetne inteligence in pogovora