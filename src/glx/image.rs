use crate::glx::gl;
use anyhow::Result;
use glutin::dpi::PhysicalSize;
use glutin::window::Window;
use std::path::PathBuf;
use std::rc::Rc;

pub fn save_image(gl: Rc<gl::Gl>, filepath: &PathBuf, window: &Window) -> Result<()> {
    let PhysicalSize { width, height } = window.inner_size();
    let nr_channels = 3;
    let mut stride = nr_channels * width;
    stride += if stride % 4 != 0 { 4 - stride % 4 } else { 0 };
    let buffer_size = stride * height;
    let mut data = Vec::<u8>::with_capacity(buffer_size as usize);
    data.resize(data.capacity(), 0);

    unsafe {
        gl.PixelStorei(gl::PACK_ALIGNMENT, 4);
        gl.ReadBuffer(gl::FRONT);
        gl.ReadPixels(
            0,
            0,
            width as i32,
            height as i32,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *mut _,
        );
    }

    let mut img = image::RgbImage::new(width, height);
    for y in 0..height {
        let mut pos = (y * stride) as usize;
        for x in 0..width {
            let red = data[pos];
            pos += 1;
            let green = data[pos];
            pos += 1;
            let blue = data[pos];
            pos += 1;
            *img.get_pixel_mut(x, height - 1 - y) = image::Rgb([red, green, blue]);
        }
    }

    Ok(img.save(filepath)?)
}
