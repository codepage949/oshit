# Refactor Core Synchronization and Safety

- Refactored `OSD_HWND` from `AtomicU32` to `AtomicUsize` for 64-bit compatibility.
- Implemented global `HOOK_HANDLE` for `CallNextHookEx` compliance.
- Switched atomic ordering from `SeqCst` to `Relaxed` for hot paths.
- Replaced hardcoded "Malgun Gothic" with default system font handle (GetStockObject).
- Streamlined `unsafe` blocks in `keyboard_hook` for readability and safety.
