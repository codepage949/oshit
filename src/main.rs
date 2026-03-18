#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;
use std::mem::size_of;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::SystemInformation::GetLocalTime;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    VK_HANGUL, VK_LSHIFT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, PostThreadMessageW, SetWindowsHookExW, UnhookWindowsHookEx,
    HHOOK, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN,
    WM_SYSKEYUP, WM_USER,
};

fn log_path() -> String {
    std::env::var("USERPROFILE")
        .map(|home| format!("{}/oshit.log", home))
        .unwrap_or_else(|_| "oshit.log".to_string())
}

fn write_log(msg: &str) {
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path())
    {
        let _ = file.write_all(msg.as_bytes());
    }
}

fn local_time_str() -> String {
    unsafe {
        let st = GetLocalTime();
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            st.wYear, st.wMonth, st.wDay, st.wHour, st.wMinute, st.wSecond
        )
    }
}

fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(|s| s.as_str()))
            .unwrap_or("알 수 없음");

        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "알 수 없는 위치".to_string());

        let msg = format!(
            "[{}] PANIC: {} (위치: {})\n",
            local_time_str(),
            payload,
            location
        );
        write_log(&msg);
        std::process::exit(1);
    }));
}

static LSHIFT_ALONE: AtomicBool = AtomicBool::new(false);
static MAIN_THREAD_ID: AtomicU32 = AtomicU32::new(0);

const WM_TOGGLE_IME: u32 = WM_USER;
/// SendInput이 주입한 키 입력을 식별하는 플래그
const LLKHF_INJECTED: u32 = 0x00000010;

unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code >= 0 {
        if l_param.0 == 0 {
            return unsafe { CallNextHookEx(HHOOK::default(), n_code, w_param, l_param) };
        }
        let kb = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };

        // SendInput으로 주입된 키는 무시 (VK_HANGUL 재진입 방지)
        if kb.flags.0 & LLKHF_INJECTED != 0 {
            eprintln!("[DEBUG] 주입된 키 (vk=0x{:02X}) → 무시", kb.vkCode);
            return unsafe { CallNextHookEx(HHOOK::default(), n_code, w_param, l_param) };
        }

        let msg = w_param.0 as u32;
        match msg {
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                if kb.vkCode == VK_LSHIFT.0 as u32 {
                    eprintln!("[DEBUG] LShift DOWN");
                    LSHIFT_ALONE.store(true, Ordering::SeqCst);
                } else {
                    if LSHIFT_ALONE.load(Ordering::SeqCst) {
                        eprintln!(
                            "[DEBUG] 다른 키 DOWN (vk=0x{:02X}) → LShift 단독 해제",
                            kb.vkCode
                        );
                    }
                    LSHIFT_ALONE.store(false, Ordering::SeqCst);
                }
            }
            WM_KEYUP | WM_SYSKEYUP => {
                if kb.vkCode == VK_LSHIFT.0 as u32 {
                    if LSHIFT_ALONE.load(Ordering::SeqCst) {
                        eprintln!("[DEBUG] LShift UP (단독) → IME 전환 요청");
                        LSHIFT_ALONE.store(false, Ordering::SeqCst);
                        // 훅 콜백 밖에서 처리하기 위해 메인 스레드에 메시지 전송
                        let tid = MAIN_THREAD_ID.load(Ordering::SeqCst);
                        let _ = unsafe { PostThreadMessageW(tid, WM_TOGGLE_IME, WPARAM(0), LPARAM(0)) };
                    } else {
                        eprintln!("[DEBUG] LShift UP (조합키 사용됨) → 전환 안 함");
                    }
                }
            }
            _ => {}
        }
    }

    unsafe { CallNextHookEx(HHOOK::default(), n_code, w_param, l_param) }
}

/// VK_HANGUL 키를 시뮬레이션하여 한/영 전환.
/// TSF(Text Services Framework) 앱과 클래식 IMM32 앱 모두
/// 표준 입력 파이프라인을 통해 VK_HANGUL을 처리하므로 범용으로 동작한다.
fn toggle_ime() {
    unsafe {
        eprintln!("[DEBUG] toggle_ime: SendInput(VK_HANGUL)");
        let inputs = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_HANGUL,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
                        ..Default::default()
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_HANGUL,
                        dwFlags: KEYEVENTF_KEYUP,
                        ..Default::default()
                    },
                },
            },
        ];
        SendInput(&inputs, size_of::<INPUT>() as i32);
    }
}

fn main() {
    setup_panic_hook();
    write_log(&format!("[{}] 프로세스 시작\n", local_time_str()));

    unsafe {
        MAIN_THREAD_ID.store(GetCurrentThreadId(), Ordering::SeqCst);
    }

    let hook = unsafe {
        SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), None, 0)
            .expect("키보드 훅 설치 실패")
    };

    println!("왼쪽 Shift로 한/영 전환 활성화됨 (종료: Ctrl+C)");

    let mut msg = MSG::default();
    unsafe {
        loop {
            let ret = GetMessageW(&mut msg, HWND::default(), 0, 0);
            if ret.0 == -1 {
                let err = windows::Win32::Foundation::GetLastError();
                write_log(&format!(
                    "[{}] 메시지 루프 오류 (GetMessageW = -1, LastError = {:?})\n",
                    local_time_str(),
                    err
                ));
                break;
            }
            if !ret.as_bool() {
                write_log(&format!(
                    "[{}] 메시지 루프 종료 (WM_QUIT)\n",
                    local_time_str()
                ));
                break;
            }
            if msg.message == WM_TOGGLE_IME {
                toggle_ime();
            }
        }
        let _ = UnhookWindowsHookEx(hook);
    }
    write_log(&format!("[{}] 프로세스 정상 종료\n", local_time_str()));
}
