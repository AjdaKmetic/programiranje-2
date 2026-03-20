enum Ocena {
    A,
    B,
    C
}

struct Ucenec {
    ime: String
}

type par = (u32, u32);

struct Par(u32, u32);

struct AritmeticnoZaporedje {
    a0: i64,
    d: i64,
    index: u64
}

/* Kaj znamo povedati o aritmetičnem zaporedju?
- začetni člen
- trenutni člen (indeks in vrednost)
- vsoto prvih nekaj členov
- naslednji člen
- prejšnji člen (razen, če je člen začetni člen)
*/

impl AritmeticnoZaporedje {
    fn new(a0: i64, d: i64) -> Self {
        AritmeticnoZaporedje {a0, d, index: 0}
    }

    fn next(&mut self) -> i64 {
        self.index += 1;
        self.a0 + (self.index as i64 - 1) * self.d

    }

    fn n_th(&self, n: u64) -> i64 { // referenca na self, sicer n_th prevzame lastništvo nad self; ko prvič pokličemo n_th in se ta izvede, na koncu pobriše self in ga ne moremo več klicati
        self.a0 + (n as i64) * self.d
    }

    fn reset(&mut self) {
        self.index = 0
    }

    fn current(&self) -> i64 {
        self.a0 + (self.index as i64) * self.d
    }

    fn sum(&mut self, n: u64) -> i64 {
        let tmp_index = self.index;
        let mut vsota = 0;
        for _ in 0..n { // če želimo, da je tudi n-ti korak vključen: 0..=n
            vsota += self.next()
        }
        self.index = tmp_index;
        vsota
    }

    fn vsota(&self, other: &Self) -> Self { // &Self, da si samo sposodi zaporedje in vrne novo zaporedje
        Self::new(self.a0 + other.a0, self.d + other.d)
    }

}

/* AST drevesa (2 + 3) * 1 
- koreni so operacije, listi so številke
*/
enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz { // enumi živijo na skladu
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>), // moramo dati Izraz na kopico, ker ne vemo, kakšne velikosti je
}

impl Izraz {
    fn konst(a: u32) -> Box<Izraz> {
        Box::new(Izraz::Konstanta(a))
    }
}

// oklepaji pridejo sami od sebe, jih ne rabimo (struktura drevesa določa oklepaje)

fn main() {
    let mut an = AritmeticnoZaporedje::new(1, 1);
    let mut bn = AritmeticnoZaporedje::new(5,4);
    let mut cn = an.vsota(&bn);
    println!("{}, {}", an.next(), an.next()); // AritmeticnoZaporedje::next(an)

    let izraz1 = Izraz::Operacija(
        Box::new(Izraz::Konstanta(1)),
        BinOperacija::Plus,
        Box::new(Izraz::Operacija(
            Box::new(Izraz::Konstanta(2)),
            BinOperacija::Times,
            Box::new(Izraz::Konstanta(2)),
        )),
    );
}

