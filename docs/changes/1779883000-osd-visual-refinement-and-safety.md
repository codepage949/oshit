# OSD Visual Refinement and Compliance Cleanup

- Implemented horizontal and vertical centering for OSD text using `GetTextExtentPoint32W`.
- Resolved `unused_must_use` warning for `GetTextExtentPoint32W`.
- Fixed `SIZE` struct import error by moving to `windows::Win32::Foundation`.
- Restored `CreateFontW` and `FW_BOLD` symbols for OSD font rendering.
- Applied `unsafe` blocks throughout `keyboard_hook` to comply with Rust 2024 `unsafe_op_in_unsafe_fn` lint.
- Cleaned up unused GDI imports.
