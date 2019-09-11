use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let b = 1.0;
    let mut rg = [0; 31];
    for _i in 0..200000 {
        let p: f64 = rng.gen_range(0.0, 1.0);
        let n = if rng.gen_bool(0.5) {
            -b * f64::ln(1.0 - p)
        } else {
            b * f64::ln(p)
        };
        let mut a = -5.0;
        let mut j = 0;
        while a < 5.0 {
            if n > a && n <= a + 10.0 / 30.0 {
                rg[j] += 1;
            }
            a += 10.0 / 30.0;
            j += 1;
        }
    }
    for i in 0..=30 {
        print!("{}\t", rg[i]);
        for _j in 0..rg[i] / 1000 {
            print!("*")
        }
        println!();
    }
}
