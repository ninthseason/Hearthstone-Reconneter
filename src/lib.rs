use std::process::{Command, Stdio};
use windows::core::Result;

use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::{
    Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    },
    ProcessStatus::GetModuleFileNameExW,
    Threading::{OpenProcess, PROCESS_QUERY_INFORMATION},
};

pub unsafe fn get_executable_path(name: &str) -> Result<Option<String>> {
    unsafe {
        let snap_handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut pe = PROCESSENTRY32W::default();
        pe.dwSize = size_of_val(&pe) as u32;
        Process32FirstW(snap_handle, &mut pe)?;
        while Process32NextW(snap_handle, &mut pe).is_ok() {
            let exe_name = String::from_utf16(
                &pe.szExeFile
                    .iter()
                    .copied()
                    .filter(|&x| x != 0)
                    .collect::<Vec<u16>>(),
            )
            .unwrap();
            if exe_name == name {
                let proc_handle = OpenProcess(PROCESS_QUERY_INFORMATION, false, pe.th32ProcessID)?;
                let mut buffer: [u16; 260] = [0; 260];
                GetModuleFileNameExW(Some(proc_handle), None, &mut buffer);
                let exe_path = String::from_utf16(
                    &buffer
                        .iter()
                        .copied()
                        .filter(|&x| x != 0)
                        .collect::<Vec<u16>>(),
                )
                .unwrap();
                CloseHandle(proc_handle)?;
                CloseHandle(snap_handle)?;
                return Ok(Some(exe_path));
            }
        }
        CloseHandle(snap_handle)?;
        Ok(None)
    }
}

pub fn is_firewall_rule_exist() -> Result<bool> {
    let mut handle = Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "show",
            "rule",
            "name=HearthstoneReconnecter",
        ])
        .stdout(Stdio::null())
        .spawn()?;
    let rc = handle.wait()?;
    Ok(rc.success())
}

pub fn create_firewall_rule() -> Result<bool> {
    let exe_path;
    unsafe {
        exe_path = get_executable_path("Hearthstone.exe")?;
    }
    let exe_path = exe_path.expect("can't find hearthstone process");
    println!("find hearthstone path: {}", exe_path,);

    let mut handle = Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "add",
            "rule",
            "name=HearthstoneReconnecter",
            "dir=out",
            "action=block",
            &format!("program={}", exe_path),
            "enable=NO",
        ])
        .stdout(Stdio::null())
        .spawn()?;
    let rc = handle.wait()?;
    Ok(rc.success())
}

pub fn enable_firewall_rule() -> Result<bool> {
    let mut handle = Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "set",
            "rule",
            "name=HearthstoneReconnecter",
            "new",
            "enable=YES",
        ])
        .stdout(Stdio::null())
        .spawn()?;
    let rc = handle.wait()?;
    Ok(rc.success())
}

pub fn disable_firewall_rule() -> Result<bool> {
    let mut handle = Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "set",
            "rule",
            "name=HearthstoneReconnecter",
            "new",
            "enable=NO",
        ])
        .stdout(Stdio::null())
        .spawn()?;
    let rc = handle.wait()?;
    Ok(rc.success())
}

