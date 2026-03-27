use std::ops::*;
use std::fmt::*;

enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz<T> { // enumi živijo na skladu
    Konstanta(T),
    Spremenljivka(String),
    Operacija(Box<Izraz<T>>, BinOperacija, Box<Izraz<T>>), // moramo dati Izraz na kopico, ker ne vemo, kakšne velikosti je
}

impl<T> Izraz<T> {
    fn konst(a: T) -> Self {
        Izraz::Konstanta(a)
    }

    fn spr(ime: &str) -> Self {
        Izraz::Spremenljivka(ime.to_string()) // ime se skopira in da na kopico, imamo kazalec na kopico?
    }

    fn op(left: Izraz<T>, op: BinOperacija, right: Izraz<T>) -> Self {
        Izraz::Operacija(Box::new(left), op, Box::new(right))
    }

    fn collect(&self) -> u32 { // vrne število konstant v izrazu
        match self {
            Izraz::Konstanta(_) => 1,
            Izraz::Spremenljivka(_) => 1,
            Izraz::Operacija(l, 
                _, 
                r) => l.collect() + r.collect()
        }
    }
}

// tu zgoraj še nismo rabili omejevati tipov, zato je dobro imeti dva impl

impl<T> Izraz<T> 
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone //????
{
    fn eval(&self) -> T {
        match self {
            Izraz::Konstanta(v) => v.clone(), // razlika med copy in clone? copy se dogaja na stacku; stvari, ki pa živijo na kopici pa se klonirajo (prekopiramo iz kopice nekam drugam na kopico in podamo kazalec, kam smo sklonirali)
            Izraz::Spremenljivka(_) => todo!(),
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
}

impl<T: Display> Izraz<T> {
    fn izpis(&self) -> String {
        match self {
            Izraz::Konstanta(v) => v.to_string(),
            Izraz::Spremenljivka(s) => s.to_string(),
            Izraz::Operacija(
                l, 
                bin_operacija, 
                r) => {
                    let li = l.izpis();
                    let ri = r.izpis();
                    match bin_operacija {
                        BinOperacija::Plus => format!("({} + {})", li, ri),
                        BinOperacija::Minus => format!("({} - {})", li, ri),
                        BinOperacija::Times => format!("({} * {})", li, ri),
                    }
                }
        }
    }
}

// Sedaj smo naredili izraze generične.

impl<T: Display> Display for Izraz<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { // <'_> ... življenjska doba
        write!(f, "Izraz: {}", self.izpis())
    }
}
