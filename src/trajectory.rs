use crate::atmosphere::{Atmosphere, ATMOS_DENSSTD};

use crate::bc::BC;
use crate::quants::Distance::Inch;
use crate::vector::{Vector, V3};


pub const MIN_RI: i32 = 1;
pub const MAX_RI: i32 = 500;
pub const DEF_RI: i32 = 100;

const TRAJ_GRAVITY: V3 = [0.0, TRAJ_GM, 0.0];

const TRAJ_GM: f64 = -32.17;
const TRAJ_MAXITCNT: usize = 10;
const TRAJ_DX: f64 = 3.0;
const TRAJ_ERROR: f64 = 0.02 / 12.0;
const TRAJ_NSTEPS: usize = 100;
const TRAJ_MINCHRONO: f64 = 0.1;
const TRAJ_ABSMAXVEL: f64 = 5000.0;
const TRAJ_ABSMINVX: f64 = 50.0;
const TRAJ_ABSMINY: f64 = -1999.9 / 12.0;

#[derive(Debug)]
pub struct Trajectory {
    pub velocity: f64,
    pub chronod: f64,
    pub weight: f64,
    azimuth: f64,
    elevation: f64,
    los_angle: f64,
    cant_angle: f64,
    sight_height: f64,
    sight_offset: f64,
    pub speed: f64,
    speed_angle: f64,
    pub range_min: i32,
    pub range_max: i32,
    pub range_inc: i32,
    zero: Vector,
    wind: Vector,
    gravity: Vector,
    options: Options,
    atmos: Atmosphere,
    pub bc: BC,
}

impl Default for Trajectory {
    fn default() -> Self {
        Trajectory {
            velocity: 0.0,
            chronod: 0.0,
            weight: 0.0,
            azimuth: 0.0,
            elevation: 0.0,
            los_angle: 0.0,
            cant_angle: 0.0,
            sight_height: 0.0,
            sight_offset: 0.0,
            speed: 0.0,
            speed_angle: 0.0,
            range_min: 0,
            range_max: 0,
            range_inc: 0,
            zero: [100.0 * TRAJ_DX, 0.0, 0.0].into(),
            wind: [0.0, 0.0, 0.0].into(),
            gravity: TRAJ_GRAVITY.into(),
            options: Default::default(),
            atmos: Default::default(),
            bc: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
struct Options {}

#[derive(Debug, Default)]
pub struct Range {
    pub range: f64,
    pub velocity: f64,
    pub energy: f64,
    pub momentum: f64,
    pub drop: f64,
    pub windage: f64,
    pub lead: f64,
    pub time: f64,
}

fn correct_gravity(traj: &Trajectory) -> Vector {
    let cl = traj.los_angle.cos();
    let sl = traj.los_angle.sin();
    let cc = traj.cant_angle.cos();
    let sc = traj.cant_angle.sin();
    Vector::new(TRAJ_GM * sl, TRAJ_GM * cl * cc, -TRAJ_GM * cl * sc)
}

fn correct_velocity(traj: &Trajectory) -> f64 {
    let mut v = traj.velocity;
    if traj.chronod > TRAJ_MINCHRONO {
        let dx = -traj.chronod / TRAJ_NSTEPS as f64;
        let eq = dx * traj.atmos.density / ATMOS_DENSSTD;
        let mut m = 0.0;
        for _ in 0..TRAJ_NSTEPS {
            m = v / traj.atmos.mach;
            let tv = v - 0.5 * eq * v * traj.bc.drag(m);
            m = tv / traj.atmos.mach;
            v = v - eq * tv * traj.bc.drag(m);
            if v > TRAJ_ABSMAXVEL {
                break;
            }
        }
    }
    v
}

fn correct_wind(traj: &Trajectory) -> Vector {
    let cl = traj.los_angle.cos();
    let sl = traj.los_angle.sin();
    let cc = traj.cant_angle.cos();
    let sc = traj.cant_angle.sin();
    let mut w = traj.wind;

    // todo;; ensure the proper xy here
    let t = w.y * cl - w.x * sl;
    w = Vector::new(w.x * cl + w.y * sl, t * cc + w.z * sc, w.z * cc - t * sc);

    Vector::new(mph_to_fps(w.x), mph_to_fps(w.y), mph_to_fps(w.z))
}

fn mph_to_fps(mph: f64) -> f64 {
    mph * 5280.0 / 3600.0
}

fn energy(w: f64, v: f64) -> f64 {
    0.5 * w * v * v / (-TRAJ_GM * 7000.0)
}

fn momentum(w: f64, v: f64) -> f64 {
    w * v / (-TRAJ_GM * 7000.0)
}

fn lead(t: f64, s: f64, a: f64) -> f64 {
    a.sin() * t * s
}

pub fn calc(traj: &Trajectory) -> Vec<Range> {
    //let i = (traj.range_max - traj.range_min) / traj.range_inc + 1;
    let z = traj.zero;
    let z = Vector::new(z.x * TRAJ_DX, Inch(z.y).to_feet(), Inch(z.z).to_feet());

    // todo
    //  o = trajectory->options;
    //  if (options_getoption(o, TRAJ_OPT_ALTI)) atmos_standardalt(trajectory->atmos);
    //  atmos_atmos(trajectory->atmos);

    let mach = traj.atmos.mach;
    let eq = traj.atmos.density / ATMOS_DENSSTD;
    let sp = traj.speed;
    let sa = traj.speed_angle;
    let g = correct_gravity(traj);
    let w = correct_wind(traj);
    let mv = correct_velocity(traj);

    // todo
    //   if (options_getoption(o, TRAJ_OPT_AZIM)) azim = 0.0;
    //   else azim = trajectory->azimuth;
    //   if (options_getoption(o, TRAJ_OPT_ELEV)) elev = 0.0;
    //   else elev = trajectory->elevation;
    let azim: f64 = 0.0;
    let elev: f64 = 0.0;

    let mut err = 0.0;
    let mut itcnt = 0;
    let mut ranges = vec![];

    while err > TRAJ_ERROR && itcnt < TRAJ_MAXITCNT || itcnt == 0 {
        let mut vm = mv;
        let mut t = 0.0;
        let mut r = Vector::new(0.0, -traj.sight_height, -traj.sight_offset);
        let mut v =
            Vector::new(elev.cos() * azim.cos(), elev.sin(), elev.cos() * azim.sin()).mul_by(vm);

        let mut dy: f64 = 0.0;
        let mut dz: f64 = 0.0;
        let mut dr = Vector::default();
        let mut dt: f64 = 0.0;
        let mut tv = Vector::default();
        let mut drg: f64 = 0.0;

        let k = traj.range_max.max(z.x as i32);
        // println!("k: {}", k);
        for i in 0..k {
            if vm < TRAJ_ABSMINVX || r.y < TRAJ_ABSMINY {
                println!("break: a1 {} | {}", vm, r.y);
                break;
            }
            if i >= traj.range_min && i <= traj.range_max && i % traj.range_inc == 0 {
                if vm < TRAJ_ABSMINVX || r.y < TRAJ_ABSMINY {
                    println!("break: a2");
                    break;
                }
                if i >= traj.range_min && i <= traj.range_max && i % traj.range_inc == 0 {
                    ranges.push(Range {
                        range: r.x / TRAJ_DX,
                        velocity: vm,
                        energy: energy(traj.weight, vm),
                        momentum: momentum(traj.weight, vm),
                        drop: r.y,
                        windage: r.z,
                        lead: lead(t, sp, sa),
                        time: t,
                    });
                }
            }

            dt = 0.5 * TRAJ_DX / v.x;
            tv = v.sub(w);
            vm = tv.len();
            drg = eq * vm * traj.bc.drag(vm / mach);
            tv = v.sub(tv.mul_by(drg).sub(g).mul_by(dt));

            dt = TRAJ_DX / tv.x;
            tv = tv.sub(w);
            vm = tv.len();
            drg = eq * vm * traj.bc.drag(vm / mach);

            //                                           inner
            // v   = SUBTRACT(v, MULTIPLY(dt, SUBTRACT(MULTIPLY(drg, tv), g)));
            let inner_mul = tv.mul_by(drg);
            let inner_sub = inner_mul.sub(g);
            let mul = inner_sub.mul_by(dt);
              v = v.sub(mul);
            //   v = v.sub(tv.mul_by(drg).sub(g).mul_by(dt));

            dr = Vector::new(TRAJ_DX, v.y * dt, v.z * dt);
            r = r.add(dr);
            vm = v.len();
            t = t + dr.len() / vm;

            if (r.x - z.x).abs() < 0.5 * TRAJ_DX {
                dy = r.y - z.y;
                dz = r.z - z.z;
                err = 0.0;
                // if (options_getoption(o, TRAJ_OPT_ELEV))
                // {
                //     err = err + fabs(dy);
                //     elev = elev - dy/r.x;
                // }
                // if (options_getoption(o, TRAJ_OPT_AZIM))
                // {
                //     err = err + fabs(dz);
                //     azim = azim - dz/r.x;
                // }
                if err > TRAJ_ERROR {
                    println!("overrun error {}", err);
                    break;
                }
            }
        }
        itcnt += 1;
    }

    ranges
}
