use windows::Win32::{
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    },
    Foundation::{INVALID_HANDLE_VALUE, CloseHandle},
};

#[derive(Debug)]
pub struct RawProcess {
    pub pid: u32,
    pub ppid: u32,
    pub image: String,
}

pub fn snapshot() -> Vec<RawProcess> {

    let mut processes = Vec::new();

    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }.unwrap_or(INVALID_HANDLE_VALUE);

    if snapshot == INVALID_HANDLE_VALUE {
        return processes;
    }

    let mut entry = PROCESSENTRY32W::default();
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

    if unsafe { Process32FirstW(snapshot, &mut entry) }.is_ok() {
        loop {
            let image = String::from_utf16_lossy(
                &entry.szExeFile
                    .iter()
                    .take_while(|&&c| c != 0)
                    .cloned()
                    .collect::<Vec<u16>>(),
            );

            processes.push(RawProcess {
                pid: entry.th32ProcessID,
                ppid: entry.th32ParentProcessID,
                image,
            });

            if unsafe { Process32NextW(snapshot, &mut entry) }.is_err() {
                break;
            }   
        }

        unsafe { let _ = CloseHandle(snapshot); };
    }
    processes
}