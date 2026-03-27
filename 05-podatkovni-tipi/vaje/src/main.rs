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
    fn konst(a: u32) -> Self {
        Izraz::Konstanta(a)
    }

    fn op(left: Izraz, op: BinOperacija, right: Izraz) -> Self {
        Izraz::Operacija(Box::new(left), op, Box::new(right))
    }

    fn eval(&self) -> u32 {
        match self {
            Izraz::Konstanta(v) => *v,
            Izraz::Operacija(
                l,
                bin_operacija, 
                r) => {
                    let lv = l.eval();
                    let rv = r.eval();
                    match bin_operacija {
                        BinOperacija::Plus => lv + rv,
                        BinOperacija::Minus => lv - rv,
                        BinOperacija::Times => lv * rv,
                    }
                }
        }
    }

    fn collect(&self) -> u32 { // vrne število konstant v izrazu
        match self {
            Izraz::Konstanta(_) => 1,
            Izraz::Operacija(l, 
                _, 
                r) => {
                    let lc = l.collect(); // lc ... left collect
                    let rc = r.collect();
                    lc + rc
                }
        }
    }

    fn izpis(&self) -> String {
        match self {
            Izraz::Konstanta(v) => v.to_string(),
            Izraz::Operacija(
                l, 
                bin_operacija, 
                r) => {
                    let li = l.izpis();
                    let ri = r.izpis();
                    match bin_operacija {
                        BinOperacija::Plus => format!("({} + {})", li, ri),
                        // li + &String::from(" + ") + &ri ... li tu spreminjamo, zato si moramo sposoditi
                        BinOperacija::Minus => format!("({} - {})", li, ri),
                        BinOperacija::Times => format!("({} * {})", li, ri),
                    }
                }
        }
    }

/* lepši zapis izpisa:
    fn izpis(&self) -> String {
        match self {
            Izraz::Konstanta(v) => v.to_string(),
            Izraz::Operacija(
                l, 
                bin_operacija, 
                r) => {
                    let li = l.izpis();
                    let ri = r.izpis();
                    let opi = match bin_operacija {
                        BinOperacija::Plus => "+",
                        BinOperacija::Minus => "-",
                        BinOperacija::Times => "*",
                    };
                    format!("({} {} {})", li, opi, ri)
                }
        }
*/
    
}

// oklepaji pridejo sami od sebe, jih ne rabimo (struktura drevesa določa oklepaje)

fn main() {
    let konst1 = Izraz::Konstanta(3);
    let konst2 = Izraz::Konstanta(4);
    let vsota = Izraz::Operacija(
        Box::new(konst1),
        BinOperacija::Plus,
        Box::new(konst2));
    // (1 + 2) * 3
    let izraz = Izraz::op(
        Izraz::op(
            Izraz::konst(1),
            BinOperacija::Plus,
            Izraz::konst(2)),
        BinOperacija::Times,
        Izraz::konst(3));
    println!("{}", izraz.izpis());
}

// &str - živi na stacku in ima točno določeno dolžino

/* kaj smo deli v main: 
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
*/