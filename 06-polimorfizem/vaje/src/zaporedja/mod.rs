// ko definiramo traite, moramo samo povedati, kaj vse mora imeti
mod aritmeticno;
mod konstantno;
mod zamaknjeno;

pub use aritmeticno::*;
use konstantno::*;
use zamaknjeno::*;

// ko bo kdorkoli uporabljal naš modul zaporedja, bo lahko uporabljal aritmetično zaporedje

pub trait Zaporedje<T> {
    fn name(&self) -> &str;
    fn start(&self) -> T;
    fn k_th(&self, k: u64) -> T;
    fn contains(&self, value: &T) -> bool;
}

