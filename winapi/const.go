package winapi

import "unsafe"

type MSG struct {
	Hwnd    uintptr
	Message uint32
	WParam  uintptr
	LParam  uintptr
	Time    uint32
	Pt      struct {
		x, y int32
	}
	LPrivate uint32
}

type INPUTUNION struct {
	_ [20]byte // NOTE: fixed
	_ uintptr  // NOTE: non-fixed
}

type INPUT struct {
	Type           uint32
	DUMMYUNIONNAME INPUTUNION
}

type KEYBDINPUT struct {
	WVk     uint16
	WScan   uint16
	DwFlags uint32
	Time    uint32
}

type KBDLLHOOKSTRUCT struct {
	VkCode uint32
}

type COMPOSITIONFORM struct {
	Style uint8
	_     [192]byte
}

type Hookproc func(int32, uintptr, unsafe.Pointer) uintptr

const (
	INPUT_KEYBOARD           = 1
	WH_KEYBOARD_LL           = 13
	VK_LSHIFT                = 0xA0
	VK_HANGUL                = 0x15
	KEYEVENTF_KEYUP          = 0x2
	WM_KEYDOWN               = 0x100
	WM_KEYUP                 = 0x101
	WM_IME_CONTROL           = 0x283
	IMC_GETCOMPOSITIONWINDOW = 0xB
	IMC_SETCOMPOSITIONWINDOW = 0xC
	IMC_GETCONVERSIONMODE    = 1
	IMC_SETCONVERSIONMODE    = 2
	CFS_EXCLUDE              = 0x80
	HC_ACTION                = 0
	VK_LWIN                  = 0x5B
	VK_RETURN                = 0xD
)
