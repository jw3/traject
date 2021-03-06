const BC_PIR: f64 = 2.08551e-04;

#[derive(Debug)]
pub enum DragFn {
    G7,
}

#[derive(Debug)]
pub struct BC {
    v: f64,
    f: DragFn,
}

impl Default for BC {
    fn default() -> Self {
        Self {
            v: 0.65,
            f: DragFn::G7,
        }
    }
}

impl BC {
    pub fn drag(&self, mach: f64) -> f64 {
        match self.f {
            DragFn::G7 => BC_PIR * g7(mach) / self.v,
        }
    }
}

fn g7(mach: f64) -> f64 {
    if mach > 1.9 {
        0.439493 + mach * (-0.0793543 + mach * 0.00448477)
    } else if mach > 1.05 {
        0.642743 + mach * (-0.2725450 + mach * 0.049247500)
    } else if mach > 0.90 {
        -1.69655 + mach * 2.03557
    } else if mach >= 0.60 {
        0.353384 + mach * (-0.69240600 + mach * 0.50946900)
    } else {
        0.119775 + mach * (-0.00231118 + mach * 0.00286712)
    }
}
