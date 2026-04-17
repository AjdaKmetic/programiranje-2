use super::Zaporedje;

pub struct AritmeticnoZaporedje<T> {
    ime: String,
    a0: T,
    d: T,
    index: u64
}

impl<T> AritmeticnoZaporedje<T> {
    pub fn new(ime: &str, a0: T, d: T) -> Self {
        AritmeticnoZaporedje {
            ime: ime.to_string(),
            a0,
            d,
            index: 0
        }
    }
}

// T: Clone - želimo, da T podpira clone
impl<T> Zaporedje<T> for AritmeticnoZaporedje<T> 
where T: Clone + std::ops::AddAssign + std::cmp::PartialEq<T>

{
    fn name(&self) -> &str {
        &self.ime
    }

    fn start(&self) -> T {
        self.a0.clone()
    }
    // če T znamo seštevati, ni nujno, da znamo seštevati referenco
    fn k_th(&self, k: u64) -> T {
        let mut result = self.a0.clone();
        // ne moremo uporabljati krat in +, zato gremo z zanko (ampak result += self.d; še vedno ni okej; rabimo dodati seštevanje k tipu T)
        for _ in 1..=k {
            result += self.d.clone();
        }
        result
    }
    // ker nikoli ne pridemo do konca seznama, če člena ne najdemu ne moremo reči, da člena ni.. -> če zaporedje ne vsebuje člena, bomo prišli so stack overflow
    fn contains(&self, value: &T) -> bool {
        let mut k = 0; // zanka dela kvadratično, to je neučinkovito
        loop {
            if self.k_th(k) == *value { // enačaj ne more biti uporabljen, zato moramo zahtevati std::cmp::PartialEq<&T>
                return true
            }
            k += 1;
        }
    }
}

impl<T: PartialEq> PartialEq for AritmeticnoZaporedje<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ime == other.ime && 
        self.a0 == other.a0 && 
        self.d == other.d && 
        self.index == other.index
    }
}

/* to naredi, ko napišemo PartialEq<T> in primerjamo T?
fn eq(&self, other: &T) -> bool {
        todo!()
    }
 */