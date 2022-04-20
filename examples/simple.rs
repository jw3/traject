use traject::trajectory::*;

fn bracket<T: PartialOrd>(X: T, L:T, H: T, D: T) -> T {
    if X < L {
      D
    } else if X > H {
        D
    } else {
        X
    }
}

const MIN_RN: i32 = 0;
const MAX_RN: i32 = 1900;
const MIN_RX: i32 = 100;
const MAX_RX: i32 = 2000;
const DEF_RN: i32 = 0;
const DEF_RX: i32 = 1000;
const DEF_WT: f64 = 220.0;
const MIN_WT: f64 = 5.0;
const MAX_WT: f64 = 5000.0;

fn main() {
    //   trajectory->range_inc = BRACKETDEF(tmp, MIN_RI, MAX_RI, DEF_RI);

    let mut t = Trajectory::default();
    t.velocity = 1500.0;
    t.range_min = bracket(0, MIN_RN, MAX_RN, DEF_RN);
    t.range_max = bracket(0, MIN_RX, MAX_RX, DEF_RX);
    t.range_inc = 1;
    t.weight = bracket(0.0, MIN_WT, MAX_WT, DEF_WT);

    calc(&t).iter().for_each(|r| println!("{:?}", r));
}
