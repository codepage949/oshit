// ref https://play.golang.org/p/2JzHDalGN7Q
package winapi

import (
	"syscall"
	"unsafe"
)

var (
	user32             = syscall.NewLazyDLL("user32.dll")
	_GetMessage        = user32.NewProc("GetMessageA")
	_SetWindowsHookExA = user32.NewProc("SetWindowsHookExA")
	_CallNextHookEx    = user32.NewProc("CallNextHookEx")
	_SendInput         = user32.NewProc("SendInput")
)

func GetMessage(m *MSG, hwnd uintptr, wMsgFilterMin, wMsgFilterMax uint32) int32 {
	r, _, _ := _GetMessage.Call(uintptr(unsafe.Pointer(m)),
		hwnd,
		uintptr(wMsgFilterMin),
		uintptr(wMsgFilterMax))

	return int32(r)
}

func SetWindowsHookExA(idHook int32, lpfn Hookproc, hmod uintptr, dwThreadId uint32) int32 {
	r, _, _ := _SetWindowsHookExA.Call(uintptr(idHook), syscall.NewCallback(lpfn), hmod, uintptr(dwThreadId))

	return int32(r)
}

func CallNextHookEx(hhk uintptr, nCode int32, wParam uintptr, lParam unsafe.Pointer) int32 {
	r, _, _ := _CallNextHookEx.Call(hhk, uintptr(nCode), wParam, uintptr(lParam))

	return int32(r)
}

func SendInput(cInputs uint32, pInputs *INPUT, cbSize int32) int32 {
	r, _, _ := _SendInput.Call(uintptr(cInputs), uintptr(unsafe.Pointer(pInputs)), uintptr(cbSize))

	return int32(r)
}
