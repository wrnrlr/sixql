use std::ffi::{c_char, c_int, c_uchar, c_void, CStr, CString};
use std::process::Output;
use std::ptr;
use pgrx::prelude::*;
use sixel_image::*;
use plotters::prelude::*;
use sixel_rs::{encoder::{Encoder,QuickFrameBuilder,QuickFrame},optflags::EncodePolicy,sys::PixelFormat,pixelformat::PixelFormatChan};
// use sixel_rs::{optflags::EncodePolicy,sys::PixelFormat,pixelformat::PixelFormatChan};
// use encoder::{Encoder,QuickFrame,QuickFrameBuilder};
use failure::{format_err};


pgrx::pg_module_magic!();

const format:PixelFormat = PixelFormat::RGBA8888;

#[pg_extern]
fn line_plot(points:Vec<f32>)->Option<String> {
    let plotfail = |e| format_err!("Plotters failed with: {:?}", e);
    let width = 100; let height = 100; let size = (width * height * format.channels_per_pixel() as u32) as usize;
    let mut buf = vec![0u8; size];
    {
        let area = BitMapBackend::with_buffer(&mut buf, (width, height)).into_drawing_area();
        // let area = BitMapBackend::new("/tmp/test.png", (width, height)).into_drawing_area();
        area.fill(&BLUE).unwrap();
        let mut chart = ChartBuilder::on(&area).build_cartesian_2d(-3.14..3.14, -1.2..1.2).unwrap();
        chart.draw_series(LineSeries::new((-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())), &RED)).unwrap();
        // area.draw_rect((50, 50), (200, 150), &GREEN, true)?;
        area.present().map_err(plotfail).ok()?;
    }

    encode(&buf, width as usize, height as usize);

    // let sixfail = |e| format_err!("Sixel failed with: {:?}", e);
    // let encoder = Encoder::new().map_err(sixfail).ok()?;
    // info!("encoder1 ready, length {} {} {} {} {}", &buf.len(), &buf[0], &buf[1], &buf[2], &buf[3]);
    // encoder.set_encode_policy(EncodePolicy::Fast).map_err(sixfail).ok()?;
    // info!("encoder2 ready");
    // let frame = QuickFrameBuilder::new().width(width as usize).height(height as usize).format(format).pixels(buf);
    // info!("encoder3 ready");
    // encoder.encode_bytes(&frame).map_err(sixfail).ok()?;
    // // encoder.encode_file(std::path::Path::new("/tmp/test.sixel")).map_err(sixfail).ok()?;
    // info!("encoder4 ready");

    // let pixels = get_pixels(&frame,width,height);
    // info!("pixels ready");
    // info!("encoder1 ready, length {} {} {} {} {}", &pixels.len(), &pixels[0], &pixels[1], &pixels[2], &pixels[3]);
    // let sixel_image = SixelImage::new(&pixels).unwrap();
    // let serialized = sixel_image.serialize();
    // info!("{:}", serialized);

    None
}

fn get_pixels(frame: &QuickFrame, width: u32, height: u32) -> Vec<u8> {
    let mut r:Vec<u8> = vec![];
    for i in 0..height { for j in 0..width { r.extend_from_slice(frame.pixel((i+1) as usize ,j as usize)); } }
    r
}

#[no_mangle] extern "C" fn callback(data: *mut c_char, size: c_int, prev_: *mut c_void) -> c_int {
    let cs = unsafe { CStr::from_ptr(data) };
    let result = unsafe { &mut CStr::from_ptr(prev_ as _) };
    *result = &cs;
    0
}

fn encode(pixels:&Vec<u8>, width:usize, height:usize)->Vec<u8> {
    unsafe {
        let result = &mut CString::new("hello").unwrap();
        let dither = sixel_sys::sixel_dither_get(sixel_sys::BuiltinDither::XTerm256);
        let output = sixel_sys::sixel_output_create(Some(callback), result as *mut _ as *mut c_void);
        sixel_sys::sixel_encode(pixels.as_ptr() as *mut c_uchar, width as c_int, height as c_int, 0, dither, output);
        sixel_sys::sixel_output_destroy(output);
        println!("result: {}", result.clone().to_str().unwrap());
        result.clone().into_bytes()
    }
}

// fn encode(pixels:&Vec<u8>, width:usize, height:usize) {
//     let palette: Vec<sixel_rs::pixelformat::Color3> = vec![];
//     let opt:Vec<u8> = vec![];
//     unsafe {
//         let mut encoder: *mut sixel_sys::Encoder = std::ptr::null_mut() as *mut _;
//         let result = sixel_sys::sixel_encoder_new(&mut encoder, ptr::null_mut() as *mut sixel_sys::Allocator);
//         info!("new encoder {:?}", result);
//         let result = sixel_sys::sixel_encoder_setopt(encoder, sixel_sys::Optfla);
//         // info!("encoder set ops {:?}", result);
//         let result = sixel_sys::sixel_encoder_encode_bytes(encoder, *pixels.as_ptr() as *mut c_uchar, width as c_int, height as c_int, format, palette.as_ptr() as *mut c_uchar, -1 as c_int);
//         info!("encode bytes {:?}", result);
//         sixel_sys::sixel_encoder_unref(encoder);
//         info!("encode bytes {:?}", palette);
//
//         // sixel_sys::sixel_encode();
//     }
// }

#[pg_extern]
fn hello_sixql()->Option<String> {
    let sample = "\u{1b}Pq\"1;1;10;10#0;2;97;47;47#0!10~-!10N\u{1b}";
    let bytes = sample.as_bytes();
    let sixel_image = SixelImage::new(&bytes).unwrap();
    let serialized = sixel_image.serialize();
    info!("{:}", sample);
    info!("{:}", serialized);
    None
}

#[cfg(test)]
mod tests {
    use crate::encode;

    #[test] fn test_encode() {
        let pixels = &vec![0u8, 0, 0, 0];
        encode(pixels, 1, 1);
    }
}

// #[cfg(any(test, feature = "pg_test"))]
// #[pg_schema]
// mod tests {
//     use pgrx::prelude::*;
//
//     #[pg_test]
//     fn test_hello_sixql() {
//         assert_eq!(Some("Hello, sixql".to_string()), crate::hello_sixql());
//     }
//
// }

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
