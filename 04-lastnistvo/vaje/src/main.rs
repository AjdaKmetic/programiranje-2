use std::time::{Duration, Instant};

fn time_it<F: FnOnce() -> R, R>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

fn on_stack() {
    // Narišite shemo spreminjanja sklada in kopice
    // Za vsako vrstico napiši, kolikokrat se v pomnilniku pojavi 13?
    let mut a = [13; 100]; // fiksne dolžine -> na stack se zapiše 13 stokrat -> sklad ima 13: 100 (če bi imeli Box::new(), bi bil array na kopici)
    let mut b = a; // vrednost iz a se skopira na b -> sklad ima 13: 100 (od prej) + 100
    let q = String::from("13"); // string je spremenljive dolžine -> "13" gre na kopico -> sklad - 13: 200 | kopica - 13: 0 (ker je "13" niz in je na kopici zapisan z asci kodo)
    // let r = "abcd" - to ni string, to je rezina, ki ni spremenljive dolžine, zato je zapisana na skladu
    println!("{}", q);
    let r = q; // r prevzame lastništvo
    let p = &r; // p si izposodi, kar je v r; dokler p obstaja, ima sposojen r?
    a[0] = 1; // spremeni prvi el. v a (tj. iz 13) v 1, b-ja NE spremeni -> sklad - 13: 99 + 100
    {
        let c = &b; // c si izposodi b, zunaj {} c ne bo več obstajal
        // ali lahko naredimo naslednje: c[0] = 80; - da, lahko, ampak c more biti potem: let mut c = &mut b; s tem se spremenita tako prvi el. b kot prvi el. c (obviously, ker je c referenca)
        println!("{}", c[0]); // izpiše se 13
        println!("{}", b[0]); // izpiše se 13, tukaj ne gre za izposojo lastništva, saj je b cel na stacku!!!
    }
    println!("{}", b[0]); // izpiše 13
    println!("{}", a[0]); // izpiše 1
    println!("{}", p); // izpiše "13"
    println!("{}", r); // izpiše "13"
    // println!("{}", q); // Razloži, zakaj to ne deluje. Ne deluje, saj je r prevzel lastništvo nad q (q je izgubil lastništvo).
}

/// Napišite funkcijo `swap`, ki zamenja vrednosti dveh celoštevilskih spremenljivk.

fn swap(a: &mut i32, b: &mut i32) {
    let c = *a;
    *a = *b;
    *b = c;
}

fn test_swap() {
    // V spremenljivko `a` shranite vrednost 13, v spremenljivko `b` pa vrednost 42.
    let mut a = 13;
    let mut b = 42;
    
    println!("a: {}, b: {}", a, b);
    // Izpiše `a: 13, b: 42`.

    // Naredite swap s pomočjo pomožne funkcije `swap`.
    swap(&mut a, &mut b);
    // swap(a, b);
/*  {
        let mut a = 13;
        let mut b = 42;
        let c = a;
        a = b;
        b = c;
    } */
    println!("a: {}, b: {}", a, b); // tedva a in b nista več kul, če narediš swap(a, b)
    // Izpiše `a: 42, b: 13`.
}

/// Popravite zakomentiran del spodnje funkcije, da bo deloval
fn str_own() {
    let x = String::from("Hello world");
    let y = &x; 
    // let y = x.clone();
    println!("{}, {}", x, y);
}

/// Popravite brez uporabe funkcije `clone`
/// Namig: sklad in kopiranje na skladu - kodo lahko spremenite
fn str_own2() {
    let x = (1, 2, (), String::from("Hello world"));
    let y = &x;
    println!("{:?}, {:?}", x, y);
}

/// Popravite spodnji dve funkciji, da bosta delovali

fn wrong() {
    let s = String::from("Hello World");
    print_str(&s); 
    /*  = {
        println!()
    } */ 
   // brez &s ne bi delalo, saj, ko pridem ven iz scopa nimam več s?
    println!("{}", s);
}

fn print_str(s: &String) {
    println!("{}", s)
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala
fn fn1() {
    let s = String::from("Hello ");

    let mut s1 = s; // vseeno ali je s mutable, saj je prevzel s1 lastništvo

    s1.push_str("World!");

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------
/// Popravite spodnjo funkcijo, da bo delovala

fn fn2() {
    let x = Box::new(5);

    // Popravite zgolj tukaj vmes
    let mut y = x.clone();
    // 
    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------

fn fn3() {
    let t = (
        String::from("hello"),
        String::from("world"),
        String::from("!"),
    );

    let _s = t.1; // _s prevzame lastništvo nad t.1

    // Izpišite čim večji del t-ja.
    println!("{}, {}", t.0, t.2); // ne bo šlo println!("{}, t")
}

/// ------------------------------------------------------------------------------------------------

fn fn4() {
    let x = 5;
    println!("{:p}", &x) // pointer na referenco
    // Izpišite naslov (mesto v pomnilniku) spremenljivke x
}

/// ------------------------------------------------------------------------------------------------

fn fn5() {
    let x = 13;
    let y = &x;

    // Popravite spodnjo vrstico, da bo bo enakost držala
    assert_eq!(13, *y);
}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper` se mora poklicati čim bolj učinkovito.
fn fn6() {
    let mut s = String::from("hello, ");

    helper(&s);

    println!("Success!");
}

// Te funkcije ne spreminjajte
fn helper(s: &String) {}

/// ------------------------------------------------------------------------------------------------

/// Popravite spodnjo funkcijo, funkcija `helper2` se mora poklicati čim bolj učinkovito.
fn fn7() {
    let mut s = String::from("hello, ");

    helper2(&mut s);

    println!("Success!");
}
// Te funkcije ne spreminjajte
fn helper2(s: &mut String) {
    s.push_str("world")
}

/// ------------------------------------------------------------------------------------------------

/// Pojasnite, zakaj spodnja koda ne deluje
fn fn8() {
    let mut s = String::from("hello, ");

    let p = &mut s;

    p.push_str("world");

    println!("Success! {}", p);
    // println!("Success! {}", s); // ne moremo uporabiti &mut s, ker smo ga že enkrat si sposodili
    p.push_str("!");
}

// lahko pa naredimo tako:
/* 
fn fn11() {
    let mut s = String::from("hello, ");

    { 
        let p = &mut s;

        p.push_str("world");

        println!("Success! {}", p);
    }
    
    println!("Success! {}", s); // ne moremo uporabiti &mut s, ker smo ga že enkrat si sposodili
    // p.push_str("!"); // ta sedaj ne obstaja
}
*/
/// ------------------------------------------------------------------------------------------------
/// Pojasnite, zakaj spodnja koda ne deluje in jo popravite
/// Pojasnite tudi zakaj je popravek ok

fn fn9() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s; // ni kul, da je dvakrat &mut s
    // primer: r1 in r2 kažeta na isto mesto, r2 spremenimo tako, da dodamo nek el., ampak kaj pa če je ta stvar sedaj prevelika in jo mora prestaviti? potem r1 več ne kaže na pravo stvar, ker se je pričakovana stvar prestavila v kopici

    println!("{}, {}", r1, r2);

    println!("Success!");
}

/// ------------------------------------------------------------------------------------------------
fn fn10() {
    // Popravite spodnjo vrstico
    let mut s = String::from("hello, ");

    helper3(&mut s);

    println!("Success!");
}

fn helper3(s: &mut String) {}

/// ------------------------------------------------------------------------------------------------

fn main() {
    test_swap()
}
