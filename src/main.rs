use std::env::args;
use std::mem::{size_of, zeroed};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
};

const VK_CONVERT: u16 = 0x1C;
const VK_NONCONVERT: u16 = 0x1D;

fn send_key(key: u16) {
    unsafe {
        let mut inputs: [INPUT; 2] = [zeroed(); 2];

        inputs[0].r#type = INPUT_KEYBOARD;
        inputs[0].Anonymous.ki = KEYBDINPUT {
            wVk: key,
            wScan: 0,
            dwFlags: 0,
            time: 0,
            dwExtraInfo: 0,
        };

        inputs[1].r#type = INPUT_KEYBOARD;
        inputs[1].Anonymous.ki = KEYBDINPUT {
            wVk: key,
            wScan: 0,
            dwFlags: KEYEVENTF_KEYUP,
            time: 0,
            dwExtraInfo: 0,
        };

        let result = SendInput(2, inputs.as_mut_ptr(), size_of::<INPUT>() as i32);
        if result != 2 {
            eprintln!("SendInput failed: only {} events were sent", result);
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <mode>", args[0]);
        eprintln!("   mode: 'ja' for non-conversion key, 'en' for conversion key");
        return;
    }

    let mode = args[1].to_lowercase();
    match mode.as_str() {
        "ja" | "japanese" => {
            send_key(VK_CONVERT);
        }
        "en" | "english" => {
            send_key(VK_NONCONVERT);
        }
        _ => {
            eprintln!("Invalid mode: {}", mode);
            eprintln!("Usage: {} <mode>", args[0]);
            eprintln!("   mode: 'ja' for non-conversion key, 'en' for conversion key");
        }
    }
}
