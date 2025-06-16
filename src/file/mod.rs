pub fn download_file(filename : &str, contents : &str) {
    let data = contents.as_bytes().to_vec();
    unsafe {
        js_create_download(
            data.as_ptr(),
            data.len(),
            filename.as_ptr(),
            filename.len()
        );
    }
}

pub fn save_file(filename : &str, contents : &str)-> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

unsafe extern "C" {
    unsafe fn js_create_download(ptr: *const u8, len : usize, name_ptr: *const u8, name_len: usize);
}
