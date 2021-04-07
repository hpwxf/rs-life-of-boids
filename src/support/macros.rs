#[macro_use]

macro_rules! check_compile {
    ($gl:ident, $vs:ident, $loc:literal) => {{
        // Setup shader compilation checks
        let mut success = i32::from(gl::FALSE);
        const SIZE : usize = 512;
        let mut info_log = Vec::<std::os::raw::c_char>::with_capacity(SIZE);
        info_log.set_len(SIZE - 1); // -1 to skip trialing null character
        $gl.GetShaderiv($vs, gl::COMPILE_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            $gl.GetShaderInfoLog(
                $vs,
                SIZE as i32,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() // as *mut gl::types::GLchar : FIXME useless ?
            );
            let message = CStr::from_ptr(info_log.as_ptr()).to_str().unwrap();
            panic!(
                "ERROR in {} : COMPILATION FAILED\n{}",
                $loc,
                message
            );
        }
    }};
}

macro_rules! check_link {
    ($gl:ident, $prog:ident, $loc:literal) => {{
        // Setup shader compilation checks
        let mut success = i32::from(gl::FALSE);
        const SIZE : usize = 512;
        let mut info_log = Vec::<std::os::raw::c_char>::with_capacity(SIZE);
        info_log.set_len(SIZE - 1); // -1 to skip trialing null character
        $gl.GetProgramiv($prog, gl::LINK_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            $gl.GetProgramInfoLog(
                $prog,
                SIZE as i32,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() // as *mut gl::types::GLchar : FIXME useless ?
            );
            let message = CStr::from_ptr(info_log.as_ptr()).to_str().unwrap();
            panic!(
                "ERROR in {} : LINK FAILED\n{}",
                $loc,
                message
            );
        }
    }};
}