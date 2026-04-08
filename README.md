# oshit

왼쪽 Shift 키를 단독으로 눌렀다 떼면 한/영 입력 상태를 전환하는 Windows 유틸리티입니다.

릴리스 빌드는 관리자 권한을 요구하는 manifest를 포함하며, 콘솔 창 없이 백그라운드에서 동작합니다. 디버그 빌드는 `cargo run`으로 실행할 수 있으며, 콘솔 로그로 동작을 확인할 수 있습니다.

## 기술 스택

- Rust 2024 edition
- `windows` crate 기반 Win32 API 호출
- `embed-resource`를 통한 Windows 리소스 포함

## 프로젝트 구조

```text
src/main.rs                         훅 설치, 단독 입력 판정, 메시지 루프, 로그/패닉 처리
src/bin/prepare-release-version.rs 릴리즈 태그 기준 Cargo.toml 버전 갱신
build.rs                            릴리스 빌드에 리소스 포함
app.manifest                        관리자 권한(requireAdministrator) 선언
app.rc                              manifest를 실행 파일 리소스로 포함
.github/workflows/release.yml       Windows 릴리즈 빌드 및 GitHub Release 생성
```

## 개발 및 테스트 방법

필수 환경:

- Rust toolchain
- Visual Studio 2022 C++ Build Tools
- Windows SDK

디버그 실행:

```powershell
cargo run
```

릴리스 빌드:

```powershell
cargo build --release
```

## 각 기능 설명

### 전역 키보드 훅

`SetWindowsHookExW(WH_KEYBOARD_LL, ...)`로 시스템 전역 저수준 키보드 훅을 설치합니다. 모든 프로세스의 키 입력을 감시하지만, 실제 판정과 전환 요청은 현재 프로세스 내부에서 처리합니다.

### 왼쪽 Shift 단독 입력 판정

왼쪽 Shift가 단독으로 눌렸다가 떼어졌는지를 `AtomicBool`로 추적합니다.

1. `LShift`가 눌리면 상태를 `true`로 설정
2. 그 사이 다른 키가 눌리면 상태를 `false`로 변경
3. `LShift`가 올라올 때 상태가 아직 `true`면 한/영 전환 요청

이 방식은 `Shift + A` 같은 일반 조합 입력과 충돌하지 않습니다.

### 한/영 전환 처리 경로

키보드 훅 콜백 안에서는 직접 `SendInput`를 호출하지 않습니다. 대신 `PostThreadMessageW`로 메인 스레드 메시지 루프에 사용자 정의 메시지(`WM_USER`)를 보낸 뒤, 그곳에서 `toggle_ime()`를 실행합니다.

전환 자체는 `SendInput`으로 `VK_HANGUL` 키의 down/up 이벤트를 주입하는 방식입니다. 이 경로는 Windows의 일반 입력 처리 체인을 타므로 TSF(Text Services Framework) 환경과 레거시 IMM32 기반 환경에서 모두 동작하도록 의도되어 있습니다.

실행 경로는 아래처럼 나뉩니다.

1. 디버그 실행: `cargo run`
2. 수동 릴리스 실행: `target\\release\\oshit.exe`
3. 권장 운영 실행: 작업 스케줄러에서 로그온 시 `oshit.exe` 실행(관리자 권한)

### 주입 이벤트 무시

`SendInput`으로 넣은 입력은 훅으로 다시 들어오기 때문에, `KBDLLHOOKSTRUCT.flags`의 `LLKHF_INJECTED` 비트를 확인해 재귀 처리와 오동작을 막습니다.

### 메시지 루프

메인 스레드는 `GetMessageW` 루프를 돌며 두 가지 역할을 맡습니다.

- 전역 키보드 훅이 살아 있도록 메시지 펌프 유지
- `WM_TOGGLE_IME` 메시지를 받으면 실제 한/영 전환 실행

### 로그와 오류 기록

- 로그 파일: `%USERPROFILE%/oshit.log`
- 프로세스 시작/종료 기록
- `GetMessageW` 실패 기록
- panic 발생 시 시간, 메시지, 위치 기록 후 종료

릴리스 빌드는 콘솔 창이 없기 때문에, 비정상 종료나 초기화 문제를 확인할 때는 이 로그 파일이 가장 직접적인 수단입니다.

## 특이 사항

- 릴리스 바이너리는 `requireAdministrator` manifest를 포함합니다.
- 관리자 권한이 필요한 전역 훅 유틸리티 특성상, 작업 스케줄러를 통한 로그온 시 자동 관리자 권한 실행이 일반적인 실행 방식보다 더 안정적입니다.
