use traject::trajectory::*;

fn main() {
    let mut t = Trajectory::default();
    t.range_max = 500;
    t.range_inc = 100;

    t.weight = 150.0;
    t.velocity = 1500.0;

    t.speed = 100.0;

    calc(&t).iter().for_each(|r| println!("{:?}", r));
}
