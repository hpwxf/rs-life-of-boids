use crate::glx::{self, gl, WindowSizeInfo};
use crate::shaders::lines::LinesRenderProgram;
use crate::shaders::points::{Point, PointsRenderProgram};
use crate::shaders::triangle::TriangleRenderProgram;
use anyhow::Result;
use std::rc::Rc;

#[derive(Debug)]
pub struct RendererConfig {
    pub size: WindowSizeInfo,
}

pub struct Renderer {
    pub gl: Rc<crate::glx::gl::Gl>,
    triangle_program: TriangleRenderProgram,
    points_program: PointsRenderProgram,
    lines_program: LinesRenderProgram,
}

impl Renderer {
    pub fn new(gl: gl::Gl, config: RendererConfig) -> Result<Renderer> {
        let gl = Rc::new(gl);
        Ok(Renderer {
            triangle_program: TriangleRenderProgram::new(gl.clone())?,
            points_program: PointsRenderProgram::new(gl.clone(), config.size)?,
            lines_program: LinesRenderProgram::new(gl.clone())?,
            gl,
        })
    }

    pub fn initialize(&mut self) -> Result<()> {
        self.points_program.initialize()?;
        self.triangle_program.initialize()?;
        self.lines_program.initialize()?;
        Ok(())
    }

    pub fn render(
        &self,
        t: f32,
        ratio: f32,
        color: [f32; 4],
        points: &[Point],
        size: (u32, u32),
    ) -> Result<()> {
        glx::clear_screen(&self.gl, color);

        self.lines_program.render(size)?;
        unsafe {
            self.gl.UseProgram(0);
        };
        self.triangle_program.render(t, ratio)?;
        unsafe {
            self.gl.UseProgram(0);
        };
        self.points_program.render(points)?;
        unsafe {
            self.gl.UseProgram(0);
        };
        Ok(())
    }
}
