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
				inputs := []winapi.INPUT{
					{Type: winapi.INPUT_KEYBOARD, DUMMYUNIONNAME: *(*winapi.INPUTUNION)(unsafe.Pointer(&winapi.KEYBDINPUT{WVk: winapi.VK_HANGUL}))},
					{Type: winapi.INPUT_KEYBOARD, DUMMYUNIONNAME: *(*winapi.INPUTUNION)(unsafe.Pointer(&winapi.KEYBDINPUT{WVk: winapi.VK_HANGUL, DwFlags: winapi.KEYEVENTF_KEYUP}))},
				}
				_ = winapi.SendInput(uint32(len(inputs)), &inputs[0], int32(unsafe.Sizeof(inputs[0])))
			}
		}
	} else {
		if wParam&0x1 == 0 { // WM_*DOWN
			if !otherPressedMap[kb.VkCode] {
				lShiftAlone = false
				otherPressedCount += 1
				otherPressedMap[kb.VkCode] = true
			}
		} else { // WM_*UP
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
