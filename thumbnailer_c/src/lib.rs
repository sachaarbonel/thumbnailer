use std::ffi::CStr;
use std::os::raw::c_char;

use thumbnailer::Thumbnailer;

#[no_mangle]
pub extern "C" fn thumbnailer_c(path_ptr: *const c_char, output_path_ptr: *const c_char) {
    let path_cstr = unsafe { CStr::from_ptr(path_ptr) };
    let path_str = path_cstr.to_str().unwrap();
    let output_path_cstr = unsafe { CStr::from_ptr(output_path_ptr) };
    let output_path_str = output_path_cstr.to_str().unwrap();
    let mut thumbnailer = Thumbnailer::from_path(path_str).unwrap();
    thumbnailer.save_image(output_path_str);
}
