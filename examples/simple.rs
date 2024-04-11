use traject::trajectory::*;

fn main() {
    let mut t = Trajectory::default();
    t.range_max = 500;
    t.range_inc = 100;

    t.weight = 150.0;
    t.velocity = 1500.0;

    t.speed = 100.0;

    print_header();
    calc(&t).iter().for_each(print_range);
}

fn print_header() {
    println!("| Range | Drop | Windage | Velocity | Energy | Time |");
    println!("| ----- | ---- | ------- | -------- | ------ | ---- |");
}

fn print_range(r: &Range) {
    println!("| {:4}  |{:.2} |  {:.1}  | {:.1}  | {:.1} | {:.3} |", r.range, r.drop, r.windage, r.velocity, r.energy, r.time)
}
