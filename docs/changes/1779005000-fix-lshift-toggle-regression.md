# LShift 한/영 토글 기능 복구 및 OSD 피드백 추가

- `LShift + ESC` 기능 도입 시 추가된 `HOTKEY_DIRTY` 플래그가 일반 키 입력 시에도 유지되어 이후의 `LShift` 단독 입력을 무효화하던 버그(Poisoning)를 수정했다.
- `HOTKEY_DIRTY` 플래그를 제거하고, `LShift`가 눌리는 시점에 `GetKeyState`를 사용하여 Ctrl, Alt, Win, RShift 등 수정키 상태를 직접 확인하도록 로직을 개선했다.
- 이제 다른 키를 입력한 직후에도 `LShift` 단독 입력 시 한/영 전환이 즉시 정상 동작한다.
