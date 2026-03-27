// ko definiramo traite, moramo samo povedati, kaj vse mora imeti
mod aritmeticno;

use aritmeticno::*; // ko bo kdorkoli uporabljal naš modul zaporedja, bo lahko uporabljal aritmetično zaporedje

trait Zaporedje<T> {
    fn name(&self) -> &str;
    fn start(&self) -> T;
    fn k_th(&self, k: u64) -> T;
    fn contains(&self, value: &T) -> bool;
}

