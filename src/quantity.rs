use std::{ops};
use unicode_segmentation::UnicodeSegmentation;


// SI unit
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub mole: i8,
    pub metre: i8,
    pub second: i8,
    pub kilogram: i8,
    pub kelvin: i8,
    pub ampere: i8,
    pub candela: i8,
}
impl Unit {
    pub const fn unitless() -> Unit {
        Unit { mole: 0, metre: 0, second: 0, kilogram: 0, kelvin: 0, ampere: 0, candela: 0 }
    }
    pub fn is_unitless(&self) -> bool {
        *self == Unit { mole: 0, metre: 0, second: 0, kilogram: 0, kelvin: 0, ampere: 0, candela: 0 }
    }
    pub fn parse_single_unit(text: &str) -> (Unit, f64, f64) {
        let chars = text.graphemes(true).collect::<Vec<&str>>();
        let mut unit = Unit::unitless();
        let mut factor;
        let mut shift: f64 = 0.0;

        // find the end of the stringy part
        let mut sepid = 0;
        for i in 0..chars.len() {
            if "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ°µμ".find(chars[i]).is_some() {
                sepid = i;
            }else{
                break;
            }
        }

        let unit_str = &chars[0..=sepid];
        let mut skip = 1;

        match unit_str[0] {
            "Q"  => {factor = 1e30;}
            "R"  => {factor = 1e27;}
            "Y"  => {factor = 1e24;}
            "Z"  => {factor = 1e21;}
            "E"  => {factor = 1e18;}
            "P"  => {factor = 1e15;}
            "T"  => {factor = 1e12;}
            "G"  => {factor = 1e9;}
            "M"  => {factor = 1e6;}
            "k"  => {factor = 1e3;}
            "h"  => {factor = 1e2;}
            "d"  => {
                if chars.len() >= 2 {
                    if chars[0] == "d" && chars[1] == "a" {
                        skip = 2;
                        factor = 10.0; // da
                    }else{
                        factor = 0.1; // d
                    }
                }else{
                    panic!("Unknown symbol 'd'");
                }
            }
            "c"  => {factor = 1.0/1e2}
            "m"  => {
                if chars.len() >= 2 {
                    if (chars[0] == "m" && chars[1] == "u") || (chars[0] == "m" && chars[1] == "i") {
                        skip = 2;
                        factor = 1.0/1e6; // mu == mi == µ
                    }else{
                        factor = 1.0/1e3; // m
                    }
                }else{
                    factor = 1.0/1e3; // m
                }
            }
            "µ" | "μ" => {factor = 1.0/1e6;} // warning: those are two different characters
            "n"  => {factor = 1.0/1e9;}
            "p"  => {factor = 1.0/1e12;}
            "f"  => {factor = 1.0/1e15;}
            "a"  => {factor = 1.0/1e18;}
            "z"  => {factor = 1.0/1e21;}
            "y"  => {factor = 1.0/1e24;}
            "r"  => {factor = 1.0/1e27;}
            "q"  => {factor = 1.0/1e30;}
            _    => {
                skip = 0; 
                factor = 1.0;
            }
        }; 

        // SPECIAL CASES
        let joined_unit_str = unit_str.join("");
        if joined_unit_str == "m" {
            // should be the meter unit not the 'milli'
            factor = 1.0; 
            skip = 0;
        }
        if joined_unit_str == "mol" {
            // should be the mole unit not the 'milli'
            factor = 1.0;
            skip = 0;
        }
        if joined_unit_str == "cd" {
            // should be the candela unit not the 'centi'
            factor = 1.0;
            skip = 0;
        }
        if joined_unit_str == "rad" {
            // should be the rad unit not the 'r'
            factor = 1.0;
            skip = 0;
        }
        if joined_unit_str == "°C" {
            // centigrade degrees
            shift = -273.15;
            factor = 1.0;
            skip = 0;
        }
        if joined_unit_str == "°" || joined_unit_str == "deg" {
            // degrees
            factor = std::f64::consts::PI / 180.0;
            skip = 0;
        }
        if joined_unit_str == "%" {
            // percentage
            factor = 0.01;
            skip = 0;
        }
        if joined_unit_str == "pi" || joined_unit_str == "π" {
            // π
            factor = std::f64::consts::PI;
            skip = 0;
        }

        if unit_str.len() > skip {
            match &unit_str[skip..].join("")[..] {
                // SI base units
                "m" => { unit.metre = 1; }
                "s" => { unit.second = 1; }
                "g" => { unit.kilogram = 1; factor = factor / 1000.0; }
                "K" => { unit.kelvin = 1; }
                "cd" => { unit.candela = 1; }
                "mol" => { unit.mole = 1; }
                "A" => { unit.ampere = 1; }

                // scales
                "°C" => { unit.kelvin = 1; }
                
                // unitless
                "°" | "deg" | "%" | "pi" | "π"=> { }

                // not SI
                "L" => { unit.metre = 3; factor = factor / 1000.0; }
                "eV" => { factor *= 1.602176565e-19; unit.kilogram = 1; unit.metre = 2; unit.second = -2; }

                // derived units
                "Hz" => { unit.second = -1; }
                "N" => { unit.kilogram = 1; unit.metre = 1; unit.second = -2; }
                "Pa" => { unit.kilogram = 1; unit.metre = -1; unit.second = -2; }
                "J" => { unit.kilogram = 1; unit.metre = 2; unit.second = -2; }
                "W" => { unit.kilogram = 1; unit.metre = 2; unit.second = -3; }
                "C" => { unit.second = 1; unit.ampere = 1; }
                "V" => { unit.kilogram = 1; unit.metre = 2; unit.second = -3; unit.ampere = -1; }
                "F" => { unit.kilogram = -1; unit.metre = -2; unit.second = 4; unit.ampere = 2; }
                "ohm" | "Ω" => { unit.kilogram = 1; unit.metre = 2; unit.second = -3; unit.ampere = -2; }
                "S" => { unit.kilogram = -1; unit.metre = -2; unit.second = 3; unit.ampere = 2; }
                "Wb" => { unit.kilogram = 1; unit.metre = 2; unit.second = -2; unit.ampere = -1; }
                "Tesla" => { unit.kilogram = 1; unit.second = -2; unit.ampere = -1; }
                "H" => { unit.kilogram = 1; unit.metre = 2; unit.second = -2; unit.ampere = -2; }
                "lm" => { unit.candela = 1; }
                "lx" => { unit.candela = 1; unit.metre = -2; }
                "rad" | "sr" => { }
                _ => {
                    panic!("Unknown unit expression '{}' due to unknown unit '{}'", text, unit_str[skip..].join("") );
                }
            }
        }

        if chars.len() - 1 >= sepid + 1 { 
            let exponent_str = &chars[sepid+1..].join("");
            let exponent: Result<i8, _> = exponent_str.parse();
            match exponent {
                Result::Ok(exp) => {
                    unit.metre *= exp;
                    unit.second *= exp;
                    unit.kilogram *= exp;
                    unit.kelvin *= exp;
                    unit.candela *= exp;
                    unit.mole *= exp;
                    unit.ampere *= exp;
                    factor = factor.powi(exp as i32);
                }
                Result::Err(e) => {
                    panic!("Unknown unit expression '{}' due to unknown exponent '{}'. Parsing error: '{}'", text, exponent_str, e);
                }
            }    
        }

        (unit, factor, shift)
    }

    pub fn parse_unit_block(text: &str) -> (Unit, f64, f64) {
        let slash_split: Vec<&str> = text.split('/').collect();
        let prod: &str;
        let mut div= "";
        match slash_split.len() {
            1 => {
                prod = slash_split[0];
            }
            2 => {
                prod = slash_split[0];
                div = slash_split[1];
            }
            _ => {
                panic!("Couldn't parse the unit block '{}' because more than one '/' where found", text);
            }
        }

        let mut unit: Unit = Unit::unitless();
        let mut factor: f64 = 1.0;
        let mut shift = 0.0;

        let mut units_counter = 0;

        for x in prod.split('.').map(|t| {
            if t == "" { return (Unit::unitless(), 1.0, 0.0); }
            units_counter += 1;
            crate::quantity::Unit::parse_single_unit(t)
        }) {
            unit = unit * x.0;
            factor *= x.1;
            shift += x.2;
        }
        for x in div.split('.').map(|t| {
            if t == "" { return (Unit::unitless(), 1.0, 0.0); }
            units_counter += 1;
            crate::quantity::Unit::parse_single_unit(t)
        }) {
            unit = unit / x.0;
            factor /= x.1;
            shift += x.2;
        }

        if shift != 0.0 && units_counter > 1 {
            panic!("Shifted units cannot be composed with other units: '{text}'");
        }

        (unit, factor, shift)
    }

    pub fn powi(&self, i: i8) -> Unit {
        Unit {
            metre: self.metre * i,
            second: self.second * i,
            kilogram: self.kilogram * i,
            kelvin: self.kelvin * i,
            candela: self.candela * i,
            mole: self.mole * i,
            ampere: self.ampere * i,
        }
    }

    pub fn taxi_norm(&self) -> i8 {
        self.metre.abs() + self.second.abs() + self.kilogram.abs() + self.kelvin.abs() + 
        self.candela.abs() + self.mole.abs() + self.ampere.abs()
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
struct ComposedUnit {
    pub mole: i8,
    pub metre: i8,
    pub second: i8,
    pub kilogram: i8,
    pub kelvin: i8,
    pub ampere: i8,
    pub candela: i8,
    pub N: i8,
    pub Pa: i8,
    pub J: i8,
    pub W: i8,
    pub C: i8,
    pub V: i8,
    pub F: i8,
    pub ohm: i8,
    pub S: i8,
    pub Wb: i8,
    pub Tesla: i8,
    pub H: i8,
    pub lx: i8,
}

impl Unit {
    fn to_composed_unit(&self) -> ComposedUnit {
        let derived_units = [
            ("N"  , Unit {kilogram: 1, metre: 1, second:-2, mole: 0, kelvin: 0, ampere: 0, candela: 0}),
            ("Pa" , Unit {kilogram: 1, metre:-1, second:-2, mole: 0, kelvin: 0, ampere: 0, candela: 0}),
            ("J"  , Unit {kilogram: 1, metre: 2, second:-2, mole: 0, kelvin: 0, ampere: 0, candela: 0}),
            ("W"  , Unit {kilogram: 1, metre: 2, second:-3, mole: 0, kelvin: 0, ampere: 0, candela: 0}),
            ("C"  , Unit {kilogram: 0, metre: 0, second: 1, mole: 0, kelvin: 0, ampere: 1, candela: 0}),
            ("V"  , Unit {kilogram: 1, metre: 2, second:-3, mole: 0, kelvin: 0, ampere:-1, candela: 0}),
            ("F"  , Unit {kilogram:-1, metre:-2, second: 4, mole: 0, kelvin: 0, ampere: 2, candela: 0}),
            ("ohm", Unit {kilogram: 1, metre: 2, second:-3, mole: 0, kelvin: 0, ampere:-2, candela: 0}),
            ("S"  , Unit {kilogram:-1, metre:-2, second: 3, mole: 0, kelvin: 0, ampere: 2, candela: 0}),
            ("Wb" , Unit {kilogram: 1, metre: 2, second:-2, mole: 0, kelvin: 0, ampere:-1, candela: 0}),
            ("Tesla", Unit {kilogram: 1, metre: 0, second:-2, mole: 0, kelvin: 0, ampere:-1, candela: 0}),
            ("H"  , Unit {kilogram: 1, metre: 2, second:-2, mole: 0, kelvin: 0, ampere:-2, candela: 0}),
            ("lx" , Unit {kilogram: 0, metre:-2, second: 0, mole: 0, kelvin: 0, ampere: 0, candela: 1}),
        //  ("Hz" , Unit {kilogram: 0, metre: 0, second:-1, mole: 0, kelvin: 0, ampere: 0, candela: 0}),

            ("kg" , Unit {kilogram: 1, metre: 0, second: 0, mole: 0, kelvin: 0, ampere: 0, candela: 0}),
            ("m"  , Unit {kilogram: 0, metre: 1, second: 0, mole: 0, kelvin: 0, ampere: 0, candela: 0}),
            ("s"  , Unit {kilogram: 0, metre: 0, second: 1, mole: 0, kelvin: 0, ampere: 0, candela: 0}),
            ("mol", Unit {kilogram: 0, metre: 0, second: 0, mole: 1, kelvin: 0, ampere: 0, candela: 0}),
            ("K"  , Unit {kilogram: 0, metre: 0, second: 0, mole: 0, kelvin: 1, ampere: 0, candela: 0}),
            ("A"  , Unit {kilogram: 0, metre: 0, second: 0, mole: 0, kelvin: 0, ampere: 1, candela: 0}),
            ("cd" , Unit {kilogram: 0, metre: 0, second: 0, mole: 0, kelvin: 0, ampere: 0, candela: 1}),
        ];

        // I keep adding the unit which reduces the unit taxi-norm the most

        let mut res = ComposedUnit { mole: 0, metre: 0, second: 0, kilogram: 0, kelvin: 0, ampere: 0, candela: 0, N: 0, Pa: 0, J: 0, W: 0, C: 0, V: 0, F: 0, ohm: 0, S: 0, Wb: 0, Tesla: 0, H: 0, lx: 0 };
        let mut current = self.clone();

        while current.taxi_norm() > 0 {
            let mut best = ("", Unit::unitless());
            let mut lowest_norm = current.taxi_norm();
            let mut mult_or_div: i8 = 1;
            let mut new_current = current.clone();

            // multiply
            for (s, u) in derived_units.iter() {
                let div = current.clone() / u.clone();
                let this_norm = div.taxi_norm();
                if this_norm < lowest_norm {
                    best = (s, u.clone());
                    lowest_norm = this_norm;
                    new_current = div;
                    mult_or_div = 1;
                }
            }

            // divide
            for (s, u) in derived_units.iter() {
                let mul = current.clone() * u.clone();
                let this_norm = mul.taxi_norm();
                if this_norm < lowest_norm {
                    best = (s, u.clone());
                    lowest_norm = this_norm;
                    new_current = mul;
                    mult_or_div = -1;
                }
            }

            current = new_current;

            match best.0 {
                "N" =>     { res.N += mult_or_div }
                "Pa" =>    { res.Pa += mult_or_div }
                "J" =>     { res.J += mult_or_div }
                "W" =>     { res.W += mult_or_div }
                "C" =>     { res.C += mult_or_div }
                "V" =>     { res.V += mult_or_div }
                "F" =>     { res.F += mult_or_div }
                "ohm" =>   { res.ohm += mult_or_div }
                "S" =>     { res.S += mult_or_div }
                "Wb" =>    { res.Wb += mult_or_div }
                "Tesla" => { res.Tesla += mult_or_div }
                "H" =>     { res.H += mult_or_div }
                "lx" =>    { res.lx += mult_or_div }
                "kg" =>    { res.kilogram += mult_or_div} 
                "m" =>     { res.metre += mult_or_div} 
                "s" =>     { res.second += mult_or_div} 
                "mol" =>   { res.mole += mult_or_div} 
                "K" =>     { res.kelvin += mult_or_div} 
                "A" =>     { res.ampere += mult_or_div} 
                "cd" =>    { res.candela += mult_or_div} 
                _ => { panic!("Found '{}' which is an unknown unit", best.0) }
            }

        } 

        res        
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_composed_unit())
    }
}

macro_rules! disp_unit {
    ($selff:ident, $string:ident, $first:ident, $counter:ident, $field: ident, $name:expr) => {
        if $selff.$field != 0 {
            if $first { $first = false; }else{ $string.push('.'); }
            let mut n: String = if $selff.$field != 1 { $selff.$field.to_string() }else{ String::new() };
            n = n.chars().map(|c: char| {
                return match c {
                    '0' => '⁰',
                    '1' => '¹',
                    '2' => '²',
                    '3' => '³',
                    '4' => '⁴',
                    '5' => '⁵',
                    '6' => '⁶',
                    '7' => '⁷',
                    '8' => '⁸',
                    '9' => '⁹',
                    '+' => '⁺',
                    '-' => '⁻',
                    _ =>   c,
                }
            }).collect();
            $string.push_str(&format!("{}{}", $name, n)[..]);
            $counter += 1;
        }
    };
}

impl std::fmt::Display for ComposedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        let mut first = true;
        let mut counter: u8 = 0;

        disp_unit!(self, string, first, counter, N, "N");
        disp_unit!(self, string, first, counter, Pa, "Pa");
        disp_unit!(self, string, first, counter, J, "J");
        disp_unit!(self, string, first, counter, W, "W");
        disp_unit!(self, string, first, counter, C, "C");
        disp_unit!(self, string, first, counter, V, "V");
        disp_unit!(self, string, first, counter, F, "F");
        disp_unit!(self, string, first, counter, ohm, "Ω");
        disp_unit!(self, string, first, counter, S, "S");
        disp_unit!(self, string, first, counter, Wb, "Wb");
        disp_unit!(self, string, first, counter, Tesla, "Tesla");
        disp_unit!(self, string, first, counter, H, "H");
        disp_unit!(self, string, first, counter, lx, "lx");
        disp_unit!(self, string, first, counter, kilogram, "kg");
        disp_unit!(self, string, first, counter, ampere, "A");
        disp_unit!(self, string, first, counter, mole, "mol");
        disp_unit!(self, string, first, counter, metre, "m");
        disp_unit!(self, string, first, counter, second, "s");
        disp_unit!(self, string, first, counter, kelvin, "K");
        disp_unit!(self, string, first, counter, candela, "cd");

        if counter <= 1 {
            write!(f, "{}", string)
        }else{
            write!(f, "|{}|", string)
        }
    }
}


// impl ops::Add<Unit> for Unit { type Output = Unit; fn add(self, _: Unit) -> Unit { self } }
// impl ops::Sub<Unit> for Unit { type Output = Unit; fn sub(self, _: Unit) -> Unit { self } }
impl ops::Mul<Unit> for Unit {
    type Output = Unit; 
    fn mul(self, rhs: Unit) -> Unit { 
        Unit {
            mole: self.mole + rhs.mole, metre: self.metre + rhs.metre, second: self.second + rhs.second, 
            kilogram: self.kilogram + rhs.kilogram, kelvin: self.kelvin + rhs.kelvin, ampere: self.ampere + rhs.ampere,
            candela: self.candela + rhs.candela,
        }
    }
}
impl ops::Div<Unit> for Unit {
    type Output = Unit; 
    fn div(self, rhs: Unit) -> Unit { 
        Unit {
            mole: self.mole - rhs.mole, metre: self.metre - rhs.metre, second: self.second - rhs.second, 
            kilogram: self.kilogram - rhs.kilogram, kelvin: self.kelvin - rhs.kelvin, ampere: self.ampere - rhs.ampere,
            candela: self.candela - rhs.candela,
        }
    }
}

#[inline]
fn squared(x: f64) -> f64 { x*x }

// Quantity with a value an uncertainty and it's unit
#[derive(Debug, Clone, PartialEq)]
pub struct Quantity {
    pub re: f64,    // real part
    pub im: f64,    // imaginary part
    pub vre: f64,   // squared error over the real part
    pub vim: f64,   // squared error over the imaginary part
    pub unit: Unit, // units
}

impl PartialEq<f64> for Quantity {
    fn eq(&self, other: &f64) -> bool {
        self.re == *other && self.vre == 0.0 && self.im == 0.0 && self.vim == 0.0 && self.unit.is_unitless()
    }
}

impl Into<Quantity> for f64 {
    fn into(self) -> Quantity {
        Quantity { re: self, im: 0.0, vre: 0.0, vim: 0.0, unit: Unit::unitless() }
    }
}

impl ops::Add<Quantity> for Quantity {
    type Output = Quantity; 
    fn add(self, rhs: Quantity) -> Quantity { 
            Quantity {
                re: self.re + rhs.re,
                im: self.im + rhs.im,
                vre: self.vre + rhs.vre,
                vim: self.vim + rhs.vim,
                unit: self.unit
            }
    }
}
impl ops::Sub<Quantity> for Quantity {
    type Output = Quantity; 
    fn sub(self, rhs: Quantity) -> Quantity { 
            Quantity {
                re: self.re - rhs.re,
                im: self.im - rhs.im,
                vre: self.vre + rhs.vre,
                vim: self.vim + rhs.vim,
                unit: self.unit
            }
    }
}
impl ops::Mul<Quantity> for Quantity {
    type Output = Quantity; 

    fn mul(self, rhs: Quantity) -> Quantity {
        let a  = self.re;   let b  = self.im;
        let c  = rhs.re;    let d  = rhs.im;
        let va = self.vre;  let vb = self.vim;
        let vc = rhs.vre;   let vd = rhs.vim;
        Quantity {
            // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
            re: a*c - b*d,
            im: a*d + b*c,
            // c^2 * va + d^2 * vb + a^2 * vc + b^2 * vd
            vre: c*c * va + d*d * vb + a*a * vc + b*b * vd,
            // d^2 * va + c^2 * vb + b^2 * vc + a^2 * vd
            vim: d*d * va + c*c * vb + b*b * vc + a*a * vd,
            unit: self.unit * rhs.unit
        }
    }
}
impl ops::Div<Quantity> for Quantity {
    type Output = Quantity; 

    fn div(self, rhs: Quantity) -> Quantity {
        let a  = self.re;   let b  = self.im;
        let c  = rhs.re;    let d  = rhs.im;
        let va = self.vre;  let vb = self.vim;
        let vc = rhs.vre;   let vd = rhs.vim;
        // (a + bi)/(c + di) = (a + bi)(c - di)/(c^2 + d^2) = { (ac + bd) + (bc - ad)i } / (c^2 + d^2)
        let denom = c*c + d*d;
        let denom2 = denom*denom;
        let denom4 = denom2*denom2;
        let re = a*c + b*d;
        let im = b*c - a*d;
        Quantity {
            re:  re / denom,
            im:  im / denom,
            vre: 
                c*c*va/denom2 + 
                d*d*vb/denom2 + 
                squared(a*denom - 2.0*c*re)*vc/denom4 + 
                squared(b*denom - 2.0*d*re)*vd/denom4,
            vim: 
                d*d*va/denom2 +
                c*c*vb/denom2 +
                squared(b*denom - 2.0*c*im)*vc/denom4 +
                squared(a*denom - 2.0*d*im)*vd/denom4,
            unit: self.unit / rhs.unit,
        }
    }
}
impl ops::Neg for Quantity {
    type Output = Quantity;
    fn neg(self) -> Quantity {
        Quantity { re: -self.re, im: -self.im, vre: self.vre, vim: self.vim, unit: self.unit.clone() }
    }
}

impl ops::Mul<f64> for Quantity {
    type Output = Quantity; 

    fn mul(self, factor: f64) -> Quantity {
        Quantity {
            re: self.re * factor,
            im: self.im * factor,
            vre: self.vre * factor * factor,
            vim: self.vim * factor * factor,
            unit: self.unit
        }
    }
}

impl Quantity {
    pub fn is_real(&self) -> bool {
        self.im == 0.0 && self.vim == 0.0
    }

    pub fn is_imaginary(&self) -> bool {
        self.re == 0.0 && self.vre == 0.0 && (self.im != 0.0 && self.vim != 0.0)
    }

    pub fn from_value_decorator(val: f64, dec: &String) -> Quantity {
        let mut unit = Unit::unitless();

        if dec == "" { return Quantity { re: val, im: 0.0, vre: 0.0, vim: 0.0, unit: unit }; }
        else if dec == "i" || dec == "j" { return Quantity { re: 0.0, im: val, vre: 0.0, vim: 0.0, unit: unit }; }

        let factor;
        let shift;
        (unit, factor, shift) = Unit::parse_single_unit(dec);

        Quantity { re: (val + shift) * factor, im: 0.0, vre: 0.0, vim: 0.0, unit: unit }
    }

    pub fn sin(&self) -> Quantity {
        // sin(z) = (e^iz - e^-iz) / 2i = sin(b)*(e^b + e^-b)/2 + i*cos(a)*(e^b - e^-b)/2 = cosh(b)sin(a) + i sinh(b)cos(a)
        let sina = self.re.sin();
        let cosa = self.re.cos();
        let sinhb = self.im.sinh();
        let coshb = self.im.cosh();
        Quantity {
            re: coshb*sina,
            im: sinhb*cosa,
            vre: squared(coshb*cosa)*self.vre + squared(sinhb*sina)*self.vim,
            vim: squared(sinhb*sina)*self.vre + squared(coshb*cosa)*self.vim,
            unit: Unit::unitless(),
        }
    }

    pub fn cos(&self) -> Quantity {
        // cos(z) = (e^iz + e^-iz) / 2 = cosh(b)cos(a) - i sinh(b)sin(a)
        let sina = self.re.sin();
        let cosa = self.re.cos();
        let sinhb = self.im.sinh();
        let coshb = self.im.cosh();
        Quantity {
            re:  coshb*cosa,
            im: -sinhb*sina,
            vre: squared(coshb*sina)*self.vre + squared(sinhb*cosa)*self.vim,
            vim: squared(sinhb*cosa)*self.vre + squared(coshb*sina)*self.vim,
            unit: Unit::unitless(),
        }
    }

    pub fn exp(&self) -> Quantity {
        // exp(z) = e^{z} = e^{x + iy} = e^x e^{iy} = e^x(cos(y) + i sin(y))
        let ex = self.re.exp();
        let excos = ex*self.im.cos();
        let exsin = ex*self.im.sin();
        let excos2 = squared(excos);
        let exsin2 = squared(exsin);
        Quantity { 
            re: excos, 
            im: exsin, 
            vre: excos2*self.vre + excos2*self.vim, 
            vim: exsin2*self.vre + excos2*self.vim, 
            unit: Unit::unitless(),
        } 
    }

    pub fn ln(&self) -> Quantity {
        // ln(z) = ln(A expiθ) = ln(A) + iθ
        todo!();
    }

    // assumes real quantities
    pub fn max(&self, other: &Quantity) -> Quantity {
        if self.re >= other.re {
            self.clone()
        }else{
            other.clone()
        }
    }

    // assumes real quantities
    pub fn min(&self, other: &Quantity) -> Quantity {
        if self.re >= other.re {
            self.clone()
        }else{
            other.clone()
        }
    }

    pub fn real_part(self) -> Quantity {
        Quantity { re: self.re, im: 0.0, vre: self.vre, vim: 0.0, unit: self.unit }
    }

    pub fn imag_part(self) -> Quantity {
        Quantity { re: 0.0, im: self.im, vre: 0.0, vim: self.vim, unit: self.unit }
    }

    pub fn sigma(self) -> Quantity {
        Quantity { re: self.vre.sqrt(), im: self.vim.sqrt(), vre: 0.0, vim: 0.0, unit: self.unit }
    }

    pub fn sigma2(self) -> Quantity {
        Quantity { re: self.vre, im: self.vim, vre: 0.0, vim: 0.0, unit: self.unit.clone()*self.unit }
    }

    pub fn value(self) -> Quantity {
        Quantity { re: self.re, im: self.im, vre: 0.0, vim: 0.0, unit: self.unit }
    }

    pub fn abs(self) -> Quantity {
        Quantity { 
            re: (self.re*self.re + self.im*self.im).sqrt(), 
            im: 0.0, 
            vre: ( self.vre * self.re * self.re + self.vim * self.im * self.im ) / (self.re*self.re + self.im*self.im) , 
            vim: 0.0, 
            unit: self.unit 
        }
    }

    pub fn arg(self) -> Quantity {
        let datan2 = 1.0 / squared(1.0 + self.im*self.im/(self.re*self.re));
        Quantity { 
            re: self.im.atan2(self.re),
            im: 0.0, 
            vre: self.vre * datan2 * (-1.0) * self.im * self.im / squared(self.re*self.re) + self.vim * datan2 / self.re / self.re, 
            vim: 0.0, 
            unit: Unit::unitless() 
        }
    }
}

fn powi(base: i32, exponent: i32) -> f64 {
    if exponent >= 0 {
        i32::checked_pow(base, exponent as u32)
        .expect(&format!("Overflow happened while raising {base} to the power of {exponent}.")) as f64
    }else{
        1.0 / (
            i32::checked_pow(base, (-exponent) as u32)
            .expect(&format!("Overflow happened while raising {base} to the power of {exponent}.")) as f64
        )
    }
}

fn number_to_text(x: f64, sx: f64, force_parenthesis: bool) -> String {
    let og: i32 = x.abs().log10().floor() as i32;
    let ogs: i32 = sx.abs().log10().floor() as i32;
    let common_og = i32::max(og, ogs);
    let powi_common_og = powi(10, common_og);
    let cifre = i32::max(0, common_og - ogs);
    let mantissa_x = format!("{0:.1$}", x / powi_common_og, cifre as usize);
    let mantissa_sx = format!("{0:.1$}", sx / powi_common_og, cifre as usize);
    let common_og_str: String = format!("{common_og}").chars().map(|c: char| {
        return match c {
            '0' => '⁰', '1' => '¹',
            '2' => '²', '3' => '³',
            '4' => '⁴', '5' => '⁵',
            '6' => '⁶', '7' => '⁷',
            '8' => '⁸', '9' => '⁹',
            '+' => '⁺', '-' => '⁻',
            _ =>   c,
        }
    }).collect();
    if common_og == 0 {
        if force_parenthesis {
            return format!("({mantissa_x} ± {mantissa_sx})");
        }else{
            return format!("{mantissa_x} ± {mantissa_sx}");
        }
    }else{
        return format!("({mantissa_x} ± {mantissa_sx})×10{common_og_str}");
    }
}

impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_real() {
            if self.unit.is_unitless() {
                if self.vre == 0.0 {
                    write!(f, "{}", self.re)
                }else{
                    write!(f, "{}", number_to_text(self.re, self.vre.sqrt(), false))
                }
            }else{
                if self.vre == 0.0 {
                    write!(f, "{}{}", self.re, self.unit)
                }else{
                    write!(f, "{}{}", number_to_text(self.re, self.vre.sqrt(), false), self.unit)
                }
            }
        }else{
            if self.unit.is_unitless() {
                if self.vre == 0.0 && self.vim == 0.0 {
                    write!(f, "{} + {}i", self.re, self.im)
                }else{
                    write!(f, "{} + i{}", number_to_text(self.re, self.vre.sqrt(), true), number_to_text(self.im, self.vim.sqrt(), true))
                }
            }else{
                if self.vre == 0.0 && self.vim == 0.0 {
                    write!(f, "({} + {}i){}", self.re, self.im, self.unit)
                }else{
                    write!(f, "{0}{2} + i{1}{2}", number_to_text(self.re, self.vre.sqrt(), true), number_to_text(self.im, self.vim.sqrt(), true), self.unit)
                }
            }
        }
    }
}

impl Quantity {
    pub fn to_text(&self, unit_str: String) -> String {
        let (unit, factor, shift) = if unit_str != "" {
            Unit::parse_unit_block(&unit_str)
        } else {
            (Unit::unitless(), 1.0, 0.0)
        };

        if unit != self.unit && unit != Unit::unitless() {
            panic!("Trying to display a quantity with units '{}' using '{}' which is interpreted as '{}'", self.unit, unit_str, unit);
        }

        // values to display
        let values: Quantity = Quantity { 
            re: (self.re + shift) / factor, 
            im: self.im / factor, 
            vre: self.vre / factor / factor, 
            vim: self.vim / factor / factor, 
            unit: unit,
        };

        if values.is_real() {
            if self.unit.is_unitless() {
                if values.vre == 0.0 {
                    return format!("{}", values.re);
                }else{
                    return format!("{}", number_to_text(values.re, values.vre.sqrt(), false));
                }
            }else{
                if values.vre == 0.0 {
                    if unit_str != "" {
                        return format!("{}{}", values.re, unit_str);
                    }else{
                        return format!("{}{}", values.re, self.unit);
                    }
                }else{
                    if unit_str != "" {
                        return format!("{}{}", number_to_text(values.re, values.vre.sqrt(), true), unit_str);
                    }else{
                        return format!("{}{}", number_to_text(values.re, values.vre.sqrt(), true), self.unit);
                    }
                }
            }
        }else{
            if self.unit.is_unitless() {
                if values.vre == 0.0 && values.vim == 0.0 {
                    return format!("{} + {}i", values.re, values.im);
                }else{
                    return format!("{} + i{}", number_to_text(values.re, values.vre.sqrt(), true), number_to_text(values.im, values.vim.sqrt(), false));
                }
            }else{
                if values.vre == 0.0 && values.vim == 0.0 {
                    if unit_str != "" {
                        return format!("({} + {}i){}", values.re, values.im, unit_str);
                    }else{
                        return format!("({} + {}i){}", values.re, values.im, self.unit);
                    }
                }else{
                    if unit_str != "" {
                        return format!("{}{} + i{}{}", number_to_text(values.re, values.vre.sqrt(), true), unit_str, number_to_text(values.im, values.vim.sqrt(), true), unit_str);
                    }else{
                        return format!("{}{} + i{}{}", number_to_text(values.re, values.vre.sqrt(), true), self.unit, number_to_text(values.im, values.vim.sqrt(), true), self.unit);
                    }
                }
            }
        }
    }
}