#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;

use embedded_graphics::pixelcolor;
use embedded_graphics::prelude::*;
use longan_nano::hal::{pac, prelude::*};
use longan_nano::{lcd, lcd_pins};
use micromath::F32Ext;
use optimath::Vector;

fn vec3<T: Copy>(x: T, y: T, z: T) -> Vector<T, 3> {
    [x, y, z].iter().copied().collect()
}

fn vec2<T: Copy>(x: T, y: T) -> Vector<T, 2> {
    [x, y].iter().copied().collect()
}

fn apply_cosine(vec: &mut Vector<f32, 3>) {
    vec[0] = vec[0].cos();
    vec[1] = vec[1].cos();
    vec[2] = vec[2].cos();
}

fn shader(coord: Vector<f32, 2>, time: f32) -> Vector<f32, 3> {
    let vec = vec3(0.0, 2.0, 4.0);
    let scale = vec3(0.5, 0.5, 0.5);
    let uv_xyx = vec3(coord[0], coord[1], coord[0]);
    let time = vec3(time, time, time);
    let sum = &time + &uv_xyx;
    let mut sum = &sum + &vec;
    apply_cosine(&mut sum);
    let result = &scale * &sum;
    &scale + &result
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();
    let mut afio = dp.AFIO.constrain(&mut rcu);

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    let mut time = 0.0f32;
    loop {
        for x in 0..width {
            for y in 0..height {
                let coord = vec2(
                    (1.0f32 / width as f32) * x as f32,
                    (1.0f32 / height as f32) * y as f32,
                );

                let color = shader(coord, time);
                let color = pixelcolor::Rgb888::new(
                    (color[0] * 255.0f32) as u8,
                    (color[1] * 255.0f32) as u8,
                    (color[2] * 255.0f32) as u8,
                );
                let color = pixelcolor::Rgb565::from(color);
                Pixel(Point::new(x, y), color).draw(&mut lcd).unwrap();
            }
        }
        time += 0.1;
    }
}
