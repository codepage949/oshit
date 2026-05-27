# LShift + ESC를 이용한 특정 윈도우 한/영 전환 예외 처리 기능 추가

## 변경 사항
- `LShift + ESC` 단축키 조합 감지 로직 추가
- 현재 포커스된 윈도우에 대해 한/영 전환 기능을 비활성화(Bypass)할 수 있는 기능 추가
- 비활성화된 윈도우 목록을 관리하기 위해 `HashSet`과 `Mutex` 사용
- `LShift + ESC` 입력 시 해당 윈도우의 비활성화 상태를 토글함
- **시각적 피드백 추가**: 상태 변경 시 화면 하단에 반투명한 OSD(On-Screen Display)를 1.5초간 표시하여 현재 상태("켬" 또는 "끔")를 알림

## 구현 세부 사항
- `keyboard_hook`에서 `LShift`와 `ESC`만 눌린 상태를 추적하기 위해 `LSHIFT_ESC_ONLY` 원자적 플래그 도입
- `WM_TOGGLE_BYPASS` 사용자 정의 메시지 추가
- `BYPASS_WINDOWS` 정적 변수를 통해 비활성화된 `HWND` 목록 관리
- `toggle_ime` 호출 시 현재 포커스된 윈도우가 비활성화 목록에 있는지 확인
- **OSD 구현**: `WS_EX_LAYERED`, `WS_EX_TRANSPARENT` 속성을 가진 레이어드 윈도우를 생성하고 GDI를 사용하여 텍스트 렌더링
