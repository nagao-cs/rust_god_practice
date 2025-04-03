use std::fmt::{Display, Formatter};

fn main() {
struct ImaginaryNumber {
    real:   f64,
    img:    f64,
}

impl Display for ImaginaryNumber {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} + {}i", self.real, self.img)
    }
}

let n = ImaginaryNumber {real: 3.0, img: 4.0};
println!("{n}");
}