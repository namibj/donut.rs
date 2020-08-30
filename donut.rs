use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut ay: f64 = 0.0;
    let mut bee: f64 = 0.0;
    let mut framecounter = 0;
    let mut pgm = std::fs::File::create("donut.pgm")?;

    while framecounter < 50 * 60 * 1 {
        let mut k: isize = -1;
        let mut i: f64 = 0.0;
        let mut j: f64 = 0.0;
        let mut z = [0.00; 1760].to_vec();
        let mut b_16 = [0 as u16; 1760];

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
                    b_16[o] = (en * ((u16::MAX as f64) / (8 as f64))) as u16;
                }
            }
        }

        write!(pgm, "P2\n{} {}\n{}\n", 80, 22, u16::MAX)?;

        while 1759 > k {
            k += 1;
            write!(pgm, "{} ", b_16[(k as usize)])?;
        }

        write!(pgm, "\n")?;
        ay += 0.04;
        bee += 0.02;
        framecounter += 1;
    };
    Ok(())
}
