use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 5000_u32;

    let w = 400_u32;
    let h = 175_u32;
    let mut backend = BitMapBackend::new("heatmap.png", (w, h));

    let width = w as f64;
    let height = h as i32;
    let slice_x_ratio = width / n as f64;
    let slice_width = slice_x_ratio.recip() as i32;
    for idx in 0..n {
        let i = idx as f64;
        let slice_x = (i * slice_x_ratio) as i32;
        let red = (i / n as f64 * 255.0) as u8;
        let c = RGBColor(red, 0, 255_u8 - red);
        backend.draw_rect((slice_x, 0), ((slice_x + slice_width), height), &c, true)?;
    }
    backend.present()?;
    Ok(())
}
