fn main() {
    let mut ay: f64 = 0.0;
    let mut bee: f64 = 0.0;

    print!("\x1b[2J");

    loop {
        let mut k: isize = -1;
        let mut i: f64 = 0.0;
        let mut j: f64 = 0.0;
        let mut z = [0.00; 1760].to_vec();
        let mut b = [' '; 1760].to_vec();

        while 6.28 > j {
            j += 0.07;

            while 6.28 > i {
                i += 0.02;

                let c = i.sin();
                let d = j.cos();
                let e = ay.cos();
                let f = j.sin();
                let g = ay.cos();
                let h = d + 2.00;
                let dee = 1.00 / (c * h * e + f * g + 5.00);
                let l = i.cos();
                let m = bee.cos();
                let n = bee.sin() as usize;
                let t = c * h * g - f * e;

                let x = 40.00 + 30.00 * dee * (l * h * m - t * n as f64);
                let y = 12.00 + 15.00 * dee * (l * h * (n as f64) + t * m);
                let o = (x + 80.00 * y) as usize;
                let en = 8.00 * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n as f64);

                if 22.00 > y
                    && y > 0.00
                    && x > 0.00
                    && 80.00 > x
                    && z.get(o).map_or(false, |z_o| dee > *z_o)
                {
                    z[o] = dee;
                    b[o] = ".,-~:;=!*#$@".chars().nth(en as usize).unwrap();
                }
            }
        }

        print!("\x1b[H");

        while 1759 > k {
            k += 1;
            print!(
                "{}",
                if k % 80 != 0 {
                    b[(k as usize)]
                } else {
                    '\n'
                }
            )
        }

        <std::io::Stdout as std::io::Write>::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        ay += 0.04;
        bee += 0.02;
    }
}
