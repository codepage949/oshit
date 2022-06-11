package oshit

import (
	"unsafe"

	"github.com/codepage949/oshit/winapi"
)

var lShiftAlone = false
var otherPressedCount = 0
var otherPressedMap = map[uint32]bool{}

func keyboardHook(nCode int32, wParam uintptr, lParam unsafe.Pointer) uintptr {
	kb := (*winapi.KBDLLHOOKSTRUCT)(lParam)

	if kb.VkCode == winapi.VK_LSHIFT {
		if wParam == winapi.WM_KEYDOWN {
			if otherPressedCount == 0 {
				lShiftAlone = true
			}
		} else if wParam == winapi.WM_KEYUP {
			if lShiftAlone {
				// NOTE: https://stackoverflow.com/questions/64280975/immgetcontext-returns-zero-always
				hwnd := winapi.ImmGetDefaultIMEWnd(winapi.GetForegroundWindow())
				tid := uint32(winapi.GetWindowThreadProcessId(hwnd, 0))
				lid := winapi.GetKeyboardLayout(tid)

				// NOTE: https://docs.microsoft.com/ko-kr/windows/win32/api/winuser/nf-winuser-getkeyboardlayout?redirectedfrom=MSDN#return-value
				//     locale 식별자가 ko-KR일 경우에만 언어 변경 작업 진행
				if lid&0xFFFF == 0x0412 {
					// NOTE: COMPOSITIONFORM에 사용하는 CFS_EXCLUDE에 대한 참고 자료가 없으나 실제로 해보면 composition window가 나타나지 않음
					cf := winapi.COMPOSITIONFORM{Style: winapi.CFS_EXCLUDE}
					_ = winapi.SendMessage(hwnd, winapi.WM_IME_CONTROL, winapi.IMC_SETCOMPOSITIONWINDOW, uintptr(unsafe.Pointer(&cf)))
					mode := 1 ^ winapi.SendMessage(hwnd, winapi.WM_IME_CONTROL, winapi.IMC_GETCONVERSIONMODE, 0)
					_ = winapi.SendMessage(hwnd, winapi.WM_IME_CONTROL, winapi.IMC_SETCONVERSIONMODE, mode)
				}
			}
		}
	} else {
		if wParam&0x1 == 0 { // NOTE: WM_*DOWN
			if !otherPressedMap[kb.VkCode] {
				lShiftAlone = false
				otherPressedCount += 1
				otherPressedMap[kb.VkCode] = true
			}
		} else { // NOTE: WM_*UP
			if otherPressedMap[kb.VkCode] {
				otherPressedCount -= 1
				otherPressedMap[kb.VkCode] = false
			}
		}
	}

	return uintptr(winapi.CallNextHookEx(0, nCode, wParam, lParam))
}

func Run() {
	_ = winapi.SetWindowsHookExA(winapi.WH_KEYBOARD_LL, keyboardHook, 0, 0)
	m := winapi.MSG{}

	for {
		_ = winapi.GetMessage(&m, 0, 0, 0)
	}
}
