pub fn downscale(buffer: &[u8], width: usize, height: usize, factor: usize) -> Vec<u8> {
    let new_w = width / factor;
    let new_h = height / factor;
    let mut down = vec![0u8; new_w * new_h * 4];

    for y in 0..new_h {
        for x in 0..new_w {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            let mut count = 0;

            for dy in 0..factor {
                for dx in 0..factor {
                    let ix = (y * factor + dy) * width * 4 + (x * factor + dx) * 4;
                    r_sum += buffer[ix + 2] as u32;
                    g_sum += buffer[ix + 1] as u32;
                    b_sum += buffer[ix + 0] as u32;
                    count += 1;
                }
            }

            let ix_new = y * new_w * 4 + x * 4;
            down[ix_new + 2] = (r_sum / count) as u8;
            down[ix_new + 1] = (g_sum / count) as u8;
            down[ix_new + 0] = (b_sum / count) as u8;
            down[ix_new + 3] = 255;
        }
    }

    down
}
