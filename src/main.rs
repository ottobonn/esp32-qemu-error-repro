#![no_std]
#![no_main]

// mod mn12832l_display;
// use mn12832l_display::{MN12832LDisplay, MN12832LDisplayPins};

// mod esp32_qemu_display;

// use embedded_graphics::{
//     image::Image,
//     pixelcolor::BinaryColor,
//     prelude::*,
//     primitives::{Circle, PrimitiveStyleBuilder, Rectangle},
// };
use esp_backtrace as _;
// use esp_hal::{
//     clock::CpuClock,
//     gpio::{Level, Output, OutputConfig},
//     main,
// };
// use tinybmp::Bmp;

use esp_hal::main;

#[main]
fn main() -> ! {
    loop {}
}

// fn main() -> ! {
//     let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
//     let peripherals = esp_hal::init(config);

//     let mut display = MN12832LDisplay::new(MN12832LDisplayPins {
//         blk: Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default()),
//         lat: Output::new(peripherals.GPIO27, Level::Low, OutputConfig::default()),
//         clk: Output::new(peripherals.GPIO32, Level::Low, OutputConfig::default()),
//         sin: Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default()),
//         ef: Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default()),
//     });

//     let bytes = include_bytes!("sprites.bmp");
//     let sprites = Bmp::<BinaryColor>::from_slice(bytes).unwrap();

//     let size = Size::new(32, 32);
//     let frame1 = sprites.sub_image(&Rectangle::new(Point::new(0, 0), size));
//     let frame2 = sprites.sub_image(&Rectangle::new(Point::new(0, 32), size));

//     let blank = Rectangle::new(Point::new(0, 0), Size::new(128, 32)).into_styled(
//         PrimitiveStyleBuilder::new()
//             .fill_color(BinaryColor::Off)
//             .build(),
//     );

//     blank.draw(&mut display).unwrap();
//     frame1.draw(&mut display).unwrap();
//     Image::new(&frame2, Point::zero())
//         .translate(Point::new(32, 0))
//         .draw(&mut display)
//         .unwrap();

//     let circle = Circle::with_center(Point::new(80, 16), 10).into_styled(
//         PrimitiveStyleBuilder::new()
//             .stroke_color(BinaryColor::On)
//             .stroke_width(1)
//             .build(),
//     );
//     circle.draw(&mut display).unwrap();

//     // let text = "Hello, MN12832L!";
//     // font.render_aligned(
//     //     text,
//     //     display.bounding_box().center() + Point::new(0, 16),
//     //     VerticalPosition::Baseline,
//     //     HorizontalAlignment::Center,
//     //     FontColor::Transparent(BinaryColor::On),
//     //     &mut display,
//     // )
//     // .unwrap();

//     // let mut frameIndex = 0;
//     // let frames = [
//     //     frame1, frame2
//     // ]

//     display.enable();
//     loop {
//         display.update();
//     }
// }
