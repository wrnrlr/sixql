use pgrx::prelude::*;
use sixel_image::*;
use plotters::prelude::*;
// use sixel_rs::encoder::{Encoder, QuickFrameBuilder};
// use sixel_rs::optflags::EncodePolicy;
// use sixel_rs::sys::PixelFormat;

pgrx::pg_module_magic!();

// #[pg_extern]
// fn line_plot(points:Vec<f32>)->Option<String> {
//     // let plotfail = |e| format!("Plotters failed with: {:?}", e);
//     let buf = &mut vec![0u8; 32];
//     let root_drawing_area = BitMapBackend::with_buffer(buf, (1024, 768)).into_drawing_area();
//     root_drawing_area.fill(&WHITE).unwrap();
//     let mut chart = ChartBuilder::on(&root_drawing_area).build_cartesian_2d(-3.14..3.14, -1.2..1.2).unwrap();
//     chart.draw_series(LineSeries::new((-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())), &RED)).unwrap();
//     // let _ = root_drawing_area.present().map_err(plotfail);
//
//     let sixfail = |e| format_err!("Sixel failed with: {:?}", e);
//     let encoder = Encoder::new().map_err(sixfail)?;
//     encoder.set_encode_policy(EncodePolicy::Fast).map_err(sixfail)?;
//     let frame = QuickFrameBuilder::new().width(xpix).height(ypix).format(PixelFormat::RGBA8888).pixels(raw.to_vec());
//
//     None
// }

#[pg_extern]
fn hello_sixql()->Option<String> {
    let sample = "\u{1b}Pq\"1;1;10;10#0;2;97;47;47#0!10~-!10N\u{1b}";
    let bytes = sample.as_bytes();
    let sixel_image = SixelImage::new(&bytes).unwrap();
    let serialized = sixel_image.serialize();
    info!("{:}", serialized);
    None
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_sixql() {
        assert_eq!("Hello, sixql", crate::hello_sixql());
    }

}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
