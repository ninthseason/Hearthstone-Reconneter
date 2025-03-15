use std::{thread, time::Duration};

use hearthstone_reconnecter::*;
use windows::{core::Result, Win32::UI::{Input::KeyboardAndMouse::{RegisterHotKey, MOD_ALT, MOD_NOREPEAT}, WindowsAndMessaging::{GetMessageW, PeekMessageW, MSG, PM_REMOVE, WM_HOTKEY}}};

fn main() -> Result<()> {
    let firewall_rule_exist = is_firewall_rule_exist().expect("can't check firewall rule");
    println!("firewall_rule_exist: {}", firewall_rule_exist);
    if !firewall_rule_exist {
        println!("create_firewall_rule: {}", create_firewall_rule()?);
    }
    unsafe {
        RegisterHotKey(None, 1, MOD_ALT | MOD_NOREPEAT, 0x41).expect("fail to bind hotkey");
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            if msg.message == WM_HOTKEY {
                println!("Reconnect start: {}", enable_firewall_rule()?);
                thread::sleep(Duration::from_secs(3));
                // 移除此时队列中的消息，这是防止用户在拔线期间依然在按快捷键，导致队列中很多消息
                while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {}
                println!("Reconnect end: {}", disable_firewall_rule()?);
            }
        }
    }
    Ok(())
}
