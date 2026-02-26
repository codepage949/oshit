# oshit

왼쪽 Shift 키를 단독으로 눌렀다 떼면 한/영 전환이 되는 Windows 유틸리티.

## 빌드

Visual Studio 2022 C++ Build Tools와 Windows SDK가 필요하다.

```
cargo build --release
```

릴리즈 빌드는 관리자 권한을 요구하는 manifest가 임베드되어 실행 시 UAC 승격 프롬프트가 표시되며, 콘솔 창 없이 백그라운드로 동작한다.

개발 중에는 `cargo run`으로 디버그 빌드를 사용할 수 있다 (관리자 권한 불필요, 콘솔 출력 활성화).

## 동작 원리

### 키보드 훅

`SetWindowsHookExW(WH_KEYBOARD_LL, ...)`로 시스템 전역 저수준 키보드 훅을 설치한다. 이 훅은 모든 프로세스의 키 입력을 가로채며, 훅을 설치한 스레드의 컨텍스트에서 콜백이 실행된다.

### 단독 누름 감지

왼쪽 Shift가 **단독으로** 눌렸는지를 `AtomicBool` 플래그로 추적한다.

1. LShift `WM_KEYDOWN` → 플래그 `true`
2. **다른 키** `WM_KEYDOWN` → 플래그 `false` (Shift+A 등 조합키에서는 전환하지 않음)
3. LShift `WM_KEYUP` → 플래그가 `true`이면 한/영 전환 요청

이 방식으로 Shift를 수식키(modifier)로 사용하는 일반적인 타이핑과 충돌하지 않는다.

### 한/영 전환 (SendInput + VK_HANGUL)

훅 콜백에서 직접 `SendInput`을 호출하면 재진입이나 데드락이 발생할 수 있다. 이를 피하기 위해 `PostThreadMessageW`로 메인 스레드의 메시지 루프에 커스텀 메시지(`WM_USER`)를 보내고, 메시지 루프에서 실제 전환을 수행한다.

전환은 `SendInput`으로 `VK_HANGUL`(0x15) 키의 DOWN/UP을 시뮬레이션한다. 이 방식은 Windows의 표준 입력 파이프라인을 거치기 때문에:

- **TSF(Text Services Framework) 앱** (Windows 11 메모장 등 모던 앱) — TSF가 입력 큐에서 VK_HANGUL을 수신하여 한국어 IME에 전달
- **IMM32 앱** (클래식 Win32 앱) — IMM32 호환 레이어를 통해 동일하게 처리

양쪽 모두에서 동작한다.

### 주입 키 필터링

`SendInput`으로 주입된 `VK_HANGUL`은 `LLKHF_INJECTED`(0x10) 플래그가 설정된 상태로 훅 콜백에 다시 들어온다. 이 플래그를 확인하여 우리가 주입한 키 입력은 무시하고 즉시 `CallNextHookEx`로 넘긴다.

### 메시지 루프

`GetMessageW` 루프는 두 가지 역할을 한다:

1. **훅 유지** — 저수준 키보드 훅이 동작하려면 훅을 설치한 스레드가 메시지를 펌핑해야 한다
2. **전환 처리** — `WM_USER` 메시지를 수신하면 `toggle_ime()`를 호출하여 한/영 전환 수행

## 구조

```
src/main.rs      전체 구현 (훅, 전환, 메시지 루프)
build.rs         릴리즈 빌드 시 manifest 리소스 임베드
app.manifest     관리자 권한(requireAdministrator) 선언
app.rc           manifest를 실행 파일에 임베드하는 리소스 스크립트
Cargo.toml       windows crate v0.58, embed-resource v3 의존성
```
