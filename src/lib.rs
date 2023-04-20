use std::ffi::CStr;
use std::os::raw::c_char;
use std::process::Command;

#[no_mangle]
pub extern "C" fn cmd_system(
    command: *const c_char,
    result_buf: *mut c_char,
    result_buf_size: i32,
) -> i32 {
    let cmd = unsafe { CStr::from_ptr(command).to_string_lossy().into_owned() };
    let result_buf = unsafe { std::slice::from_raw_parts_mut(result_buf as *mut u8, result_buf_size as usize) };
    let output = Command::new("sh").arg("-c").arg(cmd).output();

    if let Ok(output) = output {
        if let Ok(result) = String::from_utf8(output.stdout) {
            if let Err(_) = write_string_to_buffer(result, result_buf) {
                return -1;
            }
            return 0;
        }
    }
    -1
}

fn write_string_to_buffer(s: String, buf: &mut [u8]) -> std::io::Result<()> {
    let len = std::cmp::min(s.len(), buf.len());
    let bytes = s.as_bytes();
    buf[..len].copy_from_slice(&bytes[..len]);
    Ok(())
}

#[no_mangle]
pub extern "C" fn message() {
  println!("Hello C!")
}