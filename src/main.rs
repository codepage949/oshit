#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;
use std::mem::size_of;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::SystemInformation::GetLocalTime;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS,
    KEYEVENTF_KEYUP, VK_CONTROL, VK_ESCAPE, VK_HANGUL, VK_LSHIFT, VK_LWIN, VK_MENU, VK_RSHIFT,
    VK_RWIN, VK_SHIFT,
};
use windows::Win32::UI::Input::Ime::{
    ImmGetDefaultIMEWnd, COMPOSITIONFORM, IMC_SETCOMPOSITIONWINDOW,
};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, EndPaint, FillRect, GetStockObject, InvalidateRect, SelectObject,
    SetBkMode, SetTextColor, TextOutW, BLACK_BRUSH, HBRUSH, PAINTSTRUCT, TRANSPARENT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, CreateWindowExW, DefWindowProcW, DispatchMessageW, GetForegroundWindow,
    GetMessageW, GetSystemMetrics, KillTimer, PostThreadMessageW, RegisterClassW, SendMessageW,
    SetLayeredWindowAttributes, SetTimer, SetWindowPos, SetWindowsHookExW, ShowWindow,
    TranslateMessage, UnhookWindowsHookEx, CS_HREDRAW, CS_VREDRAW, HHOOK, HWND_TOPMOST,
    KBDLLHOOKSTRUCT, LWA_ALPHA, MSG, SM_CXSCREEN, SM_CYSCREEN, SWP_NOACTIVATE, SWP_NOMOVE,
    SWP_NOSIZE, SW_HIDE, SW_SHOWNOACTIVATE, WH_KEYBOARD_LL, WM_IME_CONTROL, WM_KEYDOWN, WM_KEYUP,
    WM_PAINT, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_TIMER, WM_USER, WNDCLASSW, WS_EX_LAYERED,
    WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
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

static LSHIFT_DOWN: AtomicBool = AtomicBool::new(false);
static ESC_DOWN: AtomicBool = AtomicBool::new(false);
static LSHIFT_ALONE: AtomicBool = AtomicBool::new(false);
static IS_COMBO: AtomicBool = AtomicBool::new(false);
static MAIN_THREAD_ID: AtomicUsize = AtomicUsize::new(0);
static HOOK_HANDLE: AtomicUsize = AtomicUsize::new(0);

const WM_TOGGLE_IME: u32 = WM_USER;
const WM_TOGGLE_BYPASS: u32 = WM_USER + 1;
/// SendInput이 주입한 키 입력을 식별하는 플래그
const LLKHF_INJECTED: u32 = 0x00000010;

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

static BYPASS_WINDOWS: OnceLock<Mutex<HashSet<isize>>> = OnceLock::new();
static OSD_HWND: AtomicUsize = AtomicUsize::new(0);
static OSD_TEXT: Mutex<String> = Mutex::new(String::new());

fn is_bypassed(hwnd: HWND) -> bool {
    if let Some(mutex) = BYPASS_WINDOWS.get() {
        let bypass = mutex.lock().unwrap();
        bypass.contains(&(hwnd.0 as isize))
    } else {
        false
    }
}
fn show_osd(text: &str) {
    let hwnd = HWND(OSD_HWND.load(Ordering::SeqCst) as _);
    if hwnd.is_invalid() {
        return;
    }
    {
        let mut t = OSD_TEXT.lock().unwrap();
        *t = text.to_string();
    }
    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        let _ = InvalidateRect(hwnd, None, true);
        let _ = SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        );
        // 1.5초 후 숨김 (기존 타이머가 있으면 리셋됨)
        let _ = SetTimer(hwnd, 1, 1500, None);
    }
}


fn toggle_bypass() {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.is_invalid() {
        return;
    }
    let mutex = BYPASS_WINDOWS.get_or_init(|| Mutex::new(HashSet::new()));
    let mut bypass = mutex.lock().unwrap();
    let val = hwnd.0 as isize;

    if bypass.contains(&val) {
        bypass.remove(&val);
        eprintln!("[DEBUG] Bypass REMOVED for {:?}", hwnd);
        show_osd("[현재 창] LShift 전환 켬");
    } else {
        bypass.insert(val);
        eprintln!("[DEBUG] Bypass ADDED for {:?}", hwnd);
        show_osd("[현재 창] LShift 전환 끔");
    }
}

unsafe extern "system" fn osd_window_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            unsafe {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);
                let rect = ps.rcPaint;

                // 배경 채우기 (반투명은 레이어드 윈도우 속성으로 처리됨)
                FillRect(hdc, &rect, HBRUSH(GetStockObject(BLACK_BRUSH).0));

                SetBkMode(hdc, TRANSPARENT);
                SetTextColor(hdc, windows::Win32::Foundation::COLORREF(0xFFFFFF)); // White

                let font = GetStockObject(windows::Win32::Graphics::Gdi::DEFAULT_GUI_FONT);
                let old_font = SelectObject(hdc, font);

                let text = OSD_TEXT.lock().unwrap();
                let text_u16: Vec<u16> = text.encode_utf16().collect();
                let _ = TextOutW(hdc, 20, 10, &text_u16);

                SelectObject(hdc, old_font);

                let _ = EndPaint(hwnd, &ps);
            }
            LRESULT(0)
        }
        WM_TIMER => {
            if w_param.0 == 1 {
                unsafe {
                    let _ = ShowWindow(hwnd, SW_HIDE);
                    let _ = KillTimer(hwnd, 1);
                }
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) },
    }
}

fn setup_osd(instance: windows::Win32::Foundation::HINSTANCE) -> HWND {
    let class_name = windows::core::w!("OSHIT_OSD");
    let wnd_class = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(osd_window_proc),
        hInstance: instance,
        lpszClassName: class_name,
        hCursor: unsafe { windows::Win32::UI::WindowsAndMessaging::LoadCursorW(None, windows::Win32::UI::WindowsAndMessaging::IDC_ARROW).unwrap() },
        ..Default::default()
    };

    unsafe {
        RegisterClassW(&wnd_class);

        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        let win_w = 450;
        let win_h = 70;
        let x = (screen_w - win_w) / 2;
        let y = screen_h - win_h - 100; // 하단에서 약간 위

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            class_name,
            windows::core::w!("oshit OSD"),
            WS_POPUP,
            x,
            y,
            win_w,
            win_h,
            None,
            None,
            instance,
            None,
        )
        .expect("OSD 윈도우 생성 실패");


        // 80% 투명도
        SetLayeredWindowAttributes(hwnd, windows::Win32::Foundation::COLORREF(0), 200, LWA_ALPHA)
            .unwrap();

        OSD_HWND.store(hwnd.0 as _, Ordering::Relaxed);
        hwnd
    }
}

use std::ffi::c_void;

// ...

unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code >= 0 {
        if l_param.0 == 0 {
            return unsafe { CallNextHookEx(HHOOK(HOOK_HANDLE.load(Ordering::Relaxed) as *mut c_void), n_code, w_param, l_param) };
        }
        let kb = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };

        // SendInput으로 주입된 키는 무시 (VK_HANGUL 재진입 방지)
        if kb.flags.0 & LLKHF_INJECTED != 0 {
            return unsafe { CallNextHookEx(HHOOK(HOOK_HANDLE.load(Ordering::Relaxed) as *mut c_void), n_code, w_param, l_param) };
        }

        let msg = w_param.0 as u32;
        match msg {
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                let vk = kb.vkCode as u16;
                if vk == VK_LSHIFT.0 {
                    if !LSHIFT_DOWN.swap(true, Ordering::Relaxed) {
                        eprintln!("[DEBUG] LShift DOWN");
                        // 다른 수정키(Ctrl, Alt, Win)나 RShift가 눌려있으면 단독 입력으로 보지 않음
                        let ctrl = (unsafe { GetKeyState(VK_CONTROL.0 as i32) } as u16) & 0x8000 != 0;
                        let alt = (unsafe { GetKeyState(VK_MENU.0 as i32) } as u16) & 0x8000 != 0;
                        let win = (unsafe { GetKeyState(VK_LWIN.0 as i32) } as u16) & 0x8000 != 0
                            || (unsafe { GetKeyState(VK_RWIN.0 as i32) } as u16) & 0x8000 != 0;
                        let rshift = (unsafe { GetKeyState(VK_RSHIFT.0 as i32) } as u16) & 0x8000 != 0;

                        if ctrl || alt || win || rshift {
                            LSHIFT_ALONE.store(false, Ordering::Relaxed);
                        } else if ESC_DOWN.load(Ordering::Relaxed) {
                            LSHIFT_ALONE.store(false, Ordering::Relaxed);
                            IS_COMBO.store(true, Ordering::Relaxed);
                        } else {
                            LSHIFT_ALONE.store(true, Ordering::Relaxed);
                        }
                    }
                } else if vk == VK_ESCAPE.0 {
                    if !ESC_DOWN.swap(true, Ordering::Relaxed) {
                        eprintln!("[DEBUG] ESC DOWN");
                        if LSHIFT_DOWN.load(Ordering::Relaxed) {
                            LSHIFT_ALONE.store(false, Ordering::Relaxed);
                            IS_COMBO.store(true, Ordering::Relaxed);
                        }
                    }
                } else if vk == VK_SHIFT.0 || vk == VK_RSHIFT.0 {
                    // 무시
                } else {
                    LSHIFT_ALONE.store(false, Ordering::Relaxed);
                    IS_COMBO.store(false, Ordering::Relaxed);
                    eprintln!("[DEBUG] 다른 키 DOWN (vk=0x{:02X}) → 조합 무효화", kb.vkCode);
                }
            }
            WM_KEYUP | WM_SYSKEYUP => {
                let vk = kb.vkCode as u16;
                if vk == VK_LSHIFT.0 {
                    LSHIFT_DOWN.store(false, Ordering::Relaxed);
                    if LSHIFT_ALONE.swap(false, Ordering::Relaxed) {
                        eprintln!("[DEBUG] LShift UP (단독) → IME 전환 요청");
                        let tid = MAIN_THREAD_ID.load(Ordering::Relaxed);
                        let _ = unsafe { PostThreadMessageW(tid as u32, WM_TOGGLE_IME, WPARAM(0), LPARAM(0)) };
                    } else if IS_COMBO.swap(false, Ordering::Relaxed) {
                        eprintln!("[DEBUG] LShift + ESC (LShift released) → Bypass 전환 요청");
                        let tid = MAIN_THREAD_ID.load(Ordering::Relaxed);
                        let _ = unsafe { PostThreadMessageW(tid as u32, WM_TOGGLE_BYPASS, WPARAM(0), LPARAM(0)) };
                    }
                } else if vk == VK_ESCAPE.0 {
                    ESC_DOWN.store(false, Ordering::Relaxed);
                    if IS_COMBO.swap(false, Ordering::Relaxed) {
                        eprintln!("[DEBUG] LShift + ESC (ESC released) → Bypass 전환 요청");
                        let tid = MAIN_THREAD_ID.load(Ordering::Relaxed);
                        let _ = unsafe { PostThreadMessageW(tid as u32, WM_TOGGLE_BYPASS, WPARAM(0), LPARAM(0)) };
                    }
                } else if vk == VK_SHIFT.0 || vk == VK_RSHIFT.0 {
                    // 무시
                }
            }
            _ => {}
        }
    }

    unsafe { CallNextHookEx(HHOOK(HOOK_HANDLE.load(Ordering::Relaxed) as *mut c_void), n_code, w_param, l_param) }
}

/// VK_HANGUL 키를 시뮬레이션하여 한/영 전환.
/// TSF(Text Services Framework) 앱과 클래식 IMM32 앱 모두
/// 표준 입력 파이프라인을 통해 VK_HANGUL을 처리하므로 범용으로 동작한다.
fn toggle_ime() {
    unsafe {
        let foreground_hwnd = GetForegroundWindow();
        if foreground_hwnd.is_invalid() {
            return;
        }

        if is_bypassed(foreground_hwnd) {
            eprintln!("[DEBUG] toggle_ime: Bypassed for {:?}", foreground_hwnd);
            return;
        }

        // IME 조합창 숨김 처리 (CFS_EXCLUDE 적용: 커밋 30722e16 방식)
        // 일부 앱에서 한/영 전환 시 나타나는 조합창을 억제하기 위해
        // 현재 포커스된 윈도우의 IME 조합창 설정을 CFS_EXCLUDE(0x80)로 강제한다.
        let ime_hwnd = ImmGetDefaultIMEWnd(foreground_hwnd);
        if !ime_hwnd.is_invalid() {
            // CFS_EXCLUDE (0x80)는 공식적으로는 CANDIDATEFORM에서 사용되지만,
            // COMPOSITIONFORM의 dwStyle에 사용하면 많은 IME에서 조합창을 숨기는 효과가 있음.
            let mut cf = COMPOSITIONFORM {
                dwStyle: 0x80,
                ..Default::default()
            };
            SendMessageW(
                ime_hwnd,
                WM_IME_CONTROL,
                WPARAM(IMC_SETCOMPOSITIONWINDOW as _),
                LPARAM(&mut cf as *mut _ as _),
            );
        }

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

    let instance = unsafe {
        windows::Win32::System::LibraryLoader::GetModuleHandleW(None).unwrap()
    };

    unsafe {
        MAIN_THREAD_ID.store(GetCurrentThreadId() as usize, Ordering::SeqCst);
    }

    let _osd_hwnd = setup_osd(instance.into());

    let hook = unsafe {
        SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), None, 0)
            .expect("키보드 훅 설치 실패")
    };
    HOOK_HANDLE.store(hook.0 as usize, Ordering::Relaxed);

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
            } else if msg.message == WM_TOGGLE_BYPASS {
                toggle_bypass();
            } else {
                let _ = TranslateMessage(&msg);
                let _ = DispatchMessageW(&msg);
            }
        }
        let _ = UnhookWindowsHookEx(hook);
    }
    write_log(&format!("[{}] 프로세스 정상 종료\n", local_time_str()));
}
