use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 5000_u32;

    let w = 400_u32;
    let h = 175_u32;
    let canvas = BitMapBackend::gif("heatmap_anim.gif", (w, h), 10)
        .unwrap()
        .into_drawing_area();

    // loop invarient conversions
    let width = w as f64;
    let height = h as i32;
    let nf = n as f64;
    let slice_x_ratio = width / nf;
    let slice_width = slice_x_ratio.recip() as i32;

    for grn in (0..=255).step_by(5) {
        for idx in 0..=n {
            let i = idx as f64;
            let slice_x = (i * slice_x_ratio) as i32;
            let red = (i / nf * 255.0) as u8;
            let c = RGBColor(red, grn, 255_u8 - red);
            for x in slice_x..=(slice_x + slice_width) {
                for y in 0..height {
                    canvas.draw_pixel((x, y), &c)?;
                }
            }
        }
        canvas.present()?;
    }
    Ok(())
}
