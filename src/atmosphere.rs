pub const ATMOS_DENSSTD: f64 = 0.076474;
const ATMOS_T0: f64 = 459.67;
const ATMOS_VV1: f64 = 49.0223;
const ATMOS_A0: f64 = 1.24871;
const ATMOS_A1: f64 = 0.0988438;
const ATMOS_A2: f64 = 0.00152907;
const ATMOS_A3: f64 = -3.07031e-06;
const ATMOS_A4: f64 = 4.21329e-07;
const ATMOS_PRESSSTD: f64 = 29.92;
const ATMOS_ETCONV: f64 = 3.342e-04;
const ATMOS_TSTDABS: f64 = 518.67;

#[derive(Debug)]
pub struct Atmosphere {
    // temperature in °F
    temperature: f64,
    // pressure in in Hg
    pressure: f64,
    humidity: f64,
    // altitude in feet
    altitude: f64,
    // mach 1.0 in feet/sec
    pub mach: f64,
    pub density: f64,
}

impl Default for Atmosphere {
    fn default() -> Self {
        let mut a = Atmosphere {
            temperature: 59.0,
            pressure: 29.92,
            humidity: 0.0,
            altitude: 0.0,
            mach: 0.0,
            density: 0.0,
        };
        let (m, d) = mach_density(&a);
        a.mach = m;
        a.density = d;
        a
    }
}

fn mach_density(atmos: &Atmosphere) -> (f64, f64) {
    let t = atmos.temperature;
    let p = atmos.pressure;
    let hc = if t > 0.0 {
        let et0 = ATMOS_A0 + t * (ATMOS_A1 + t * (ATMOS_A2 + t * (ATMOS_A3 + t * ATMOS_A4)));
        let et = ATMOS_ETCONV * atmos.humidity * et0;
        (p - 0.3783 * et) / ATMOS_PRESSSTD
    } else {
        1.0
    };

    let d = ATMOS_DENSSTD * (ATMOS_TSTDABS / (t + ATMOS_T0)) * hc;
    let m = (t + ATMOS_T0).sqrt() * ATMOS_VV1;
    (m, d)
}
