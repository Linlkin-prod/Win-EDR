use windows::Win32::{
    System::Threading::*,
    Foundation::*,
};
use windows::core::PWSTR;

pub fn get_image_path(pid: u32) -> Option<String> {
    unsafe {
        let handle = match OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) {
            Ok(h) => h,
            Err(_) => return None,
        };

        let mut buf = [0u16; 260];
        let mut size = buf.len() as u32;

        let result = QueryFullProcessImageNameW(handle, Default::default(), PWSTR(buf.as_mut_ptr()), &mut size);
        let _ = CloseHandle(handle);

        if result.is_ok() {
            Some(String::from_utf16_lossy(&buf[..size as usize]).to_string())
        } else {
            None
        }
    }
}
