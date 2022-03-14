use std::ffi::CStr;

use libc::c_char;
use thumbnailer::Thumbnailer;

#[no_mangle]
pub extern "C" fn thumbnail_file(video_path_ptr: *const c_char, thumbnail_path_ptr: *const c_char) {
    //, thumbnail_path_ptr: *const c_char

    let video_path_cstr = unsafe {
        assert!(!video_path_ptr.is_null());

        CStr::from_ptr(video_path_ptr)
    };
    let thumbnail_path_cstr = unsafe {
        assert!(!thumbnail_path_ptr.is_null());

        CStr::from_ptr(thumbnail_path_ptr)
    };
    let video_path_str = video_path_cstr.to_string_lossy();
    let thumbnail_path_str = thumbnail_path_cstr.to_string_lossy();
    // fun_name(&video_path_str);
    println!("video path {}", fun_name(&video_path_str));
    println!("thumbnail path {}", fun_name(&thumbnail_path_str));
    // let thumbnail_path_ptr_cstr = unsafe { CStr::from_ptr(thumbnail_path_ptr) };
    // let thumbnail_path_str = thumbnail_path_ptr_cstr.to_str().unwrap();

    // print!("video thumbnail path: {}", thumbnail_path_str);
    let video_path = "assets/vp9-opus.webm".to_string();
    let output_path = "assets/vp9-opus.png".to_string();
    let mut thumbnailer = Thumbnailer::from_path(video_path).unwrap();
    println!("instantiated");
    thumbnailer.save_image(output_path);
}

fn fun_name(video_path_str: &str) -> String {
    video_path_str.to_owned()
    // print!("video path: {}", video_path_str);
}
