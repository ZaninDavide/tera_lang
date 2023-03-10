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
        let mut factor: f64 = 1.0;
        let mut shift: f64 = 0.0;

        // find the end of the stringy part
        let mut sepid = 0;
        for i in 0..chars.len() {
            if "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ°".find(chars[i]).is_some() {
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
                    if (chars[0] == "u" && chars[1] == "u") || (chars[0] == "u" && chars[1] == "i") {
                        skip = 2;
                        factor = 1.0/1e6; // mu == mi == µ
                    }else{
                        factor = 1.0/1e3; // m
                    }
                }else{
                    factor = 1.0/1e3; // m
                }
            }
            "µ"=> {factor = 1.0/1e6;}
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
                "°" | "deg" => { }
                "°C" => { unit.kelvin = 1; }

                // derived units
                "Hz" => { unit.second = -1; }
                "N" => { unit.kilogram = 1; unit.metre = 1; unit.second = -2; }
                "Pa" => { unit.kilogram = 1; unit.metre = -1; unit.second = -2; }
                "J" => { unit.kilogram = 1; unit.metre = 2; unit.second = -2; }
                "W" => { unit.kilogram = 1; unit.metre = 2; unit.second = -3; }
                "C" => { unit.second = 1; unit.ampere = 1; }
                "V" => { unit.kilogram = 1; unit.metre = 2; unit.second = -3; unit.ampere = -1; }
                "F" => { unit.kilogram = -1; unit.metre = -2; unit.second = 4; unit.ampere = 2; }
                "ohm" => { unit.kilogram = 1; unit.metre = 2; unit.second = -3; unit.ampere = -2; }
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
            let exponent_str = &text[sepid+1..];
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
}
impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from("");
        let mut first = true;
        let mut counter: u8 = 0;

        if self.kilogram != 0 {
            if first { first = false; }else{ string.push('.'); }
            let n: String = if self.kilogram != 1 { self.kilogram.to_string() }else{ String::from("") };
            string.push_str(&format!("kg{}", n)[..]);
            counter += 1;
        }
        if self.ampere != 0 {
            if first { first = false; }else{ string.push('.'); }
            let n: String = if self.ampere != 1 { self.ampere.to_string() }else{ String::from("") };
            string.push_str(&format!("A{}", n)[..]);
            counter += 1;
        }
        if self.mole != 0 {
            if first { first = false; }else{ string.push('.'); }
            let n: String = if self.mole != 1 { self.mole.to_string() }else{ String::from("") };
            string.push_str(&format!("mol{}", n)[..]);
            counter += 1;
        }
        if self.metre != 0 {
            if first { first = false; }else{ string.push('.'); }
            let n: String = if self.metre != 1 { self.metre.to_string() }else{ String::from("") };
            string.push_str(&format!("m{}", n)[..]);
            counter += 1;
        }
        if self.second != 0 {
            if first { first = false; }else{ string.push('.'); }
            let n: String = if self.second != 1 { self.second.to_string() }else{ String::from("") };
            string.push_str(&format!("s{}", n)[..]);
            counter += 1;
        }
        if self.kelvin != 0 {
            if first { first = false; }else{ string.push('.'); }
            let n: String = if self.kelvin != 1 { self.kelvin.to_string() }else{ String::from("") };
            string.push_str(&format!("K{}", n)[..]);
            counter += 1;
        }
        if self.candela != 0 {
            if first { first = false; }else{ string.push('.'); }
            let n: String = if self.candela != 1 { self.candela.to_string() }else{ String::from("") };
            string.push_str(&format!("cd{}", n)[..]);
            counter += 1;
        }

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
            vre: self.vre * factor,
            vim: self.vim * factor,
            unit: self.unit
        }
    }
}

impl Quantity {
    pub fn is_real(&self) -> bool {
        self.im == 0.0 && self.vim == 0.0
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

    pub fn from_value_decorator(val: f64, dec: &String) -> Quantity {
        let mut unit = Unit::unitless();

        if dec == "" { return Quantity { re: val, im: 0.0, vre: 0.0, vim: 0.0, unit: unit }; }
        else if dec == "i" || dec == "j" { return Quantity { re: 0.0, im: val, vre: 0.0, vim: 0.0, unit: unit }; }

        let factor;
        let shift;
        (unit, factor, shift) = Unit::parse_single_unit(dec);

        Quantity { re: (val + shift) * factor, im: 0.0, vre: 0.0, vim: 0.0, unit: unit }
    }
}

impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_real() {
            if self.unit.is_unitless() {
                if self.vre == 0.0 {
                    write!(f, "{}", self.re)
                }else{
                    write!(f, "{} ± {}", self.re, self.vre.sqrt())
                }
            }else{
                if self.vre == 0.0 {
                    write!(f, "{}{}", self.re, self.unit)
                }else{
                    write!(f, "({} ± {}){}", self.re, self.vre.sqrt(), self.unit)
                }
            }
        }else{
            if self.unit.is_unitless() {
                if self.vre == 0.0 && self.vim == 0.0 {
                    write!(f, "{} + {}i", self.re, self.im)
                }else{
                    write!(f, "({} ± {}) + i({} ± {})", self.re, self.vre.sqrt(), self.im, self.vim.sqrt())
                }
            }else{
                if self.vre == 0.0 && self.vim == 0.0 {
                    write!(f, "({} + {}i){}", self.re, self.im, self.unit)
                }else{
                    write!(f, "({} ± {}){} + i({} ± {}){}", self.re, self.vre.sqrt(), self.unit, self.im, self.vim.sqrt(), self.unit)
                }
            }
        }
    }
}