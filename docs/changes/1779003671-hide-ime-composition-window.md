# IME 조합창 숨김 처리 (CFS_EXCLUDE 적용)

## 구현 계획

1. `Cargo.toml`에 `Win32_UI_Input_Ime` 기능을 추가한다.
2. `src/main.rs`에서 `toggle_ime` 함수 실행 시 현재 포커스된 윈도우의 IME 조합창이 나타나지 않도록 `CFS_EXCLUDE` 스타일을 적용한다.
   - `GetForegroundWindow`로 현재 윈도우 획득
   - `ImmGetDefaultIMEWnd`로 기본 IME 윈도우 획득
   - `SendMessageW`를 통해 `IMC_SETCOMPOSITIONWINDOW` 메시지 전송
   - `COMPOSITIONFORM` 구조체에 문서화되지 않은 `CFS_EXCLUDE` (0x80) 스타일 적용

## 변경 사항

- `Cargo.toml`: `windows` 크레이트의 `features`에 `Win32_UI_Input_Ime` 추가
- `src/main.rs`:
  - `windows::Win32::UI::Input::Ime` 임포트 추가
  - `toggle_ime` 내부에 `CFS_EXCLUDE`를 이용한 조합창 숨김 로직 구현 (커밋 30722e16 방식)

## 테스트

- 한글 입력 중 왼쪽 Shift를 눌렀을 때 조합창(글자 밑줄 상태)이 나타나지 않고 정상적으로 한/영 전환이 이루어지는지 육안으로 확인한다.
- 커밋 30722e16에서 검증된 방식인 `CFS_EXCLUDE` 스타일이 실제 환경에서 동작하는지 확인한다.
