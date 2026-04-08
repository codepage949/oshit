# 릴리즈 GitHub Actions 추가

## 구현 계획

- 현재 Rust 바이너리 프로젝트 기준으로 윈도우 전용 릴리즈 배포 흐름을 설계한다.
- 수동 실행(`workflow_dispatch`)으로 버전 태그를 입력받아 릴리즈를 생성한다.
- `windows-latest`에서 릴리즈 바이너리를 빌드하고 zip으로 압축한다.
- 생성된 산출물을 GitHub Release에 첨부한다.

## 변경 사항

- `.github/workflows/release.yml` 추가
- `.gitignore`에 `.github/workflows/release.yml` 예외 규칙 추가
- `src/bin/prepare-release-version.rs` 추가
- `Cargo.toml`에 `toml_edit` 의존성 추가
- 입력한 태그가 이미 존재하는지 확인하는 단계 추가
- 입력한 태그를 기준으로 `Cargo.toml` 버전을 갱신하고 `Cargo.lock`을 재생성하는 단계 추가
- 버전 변경이 있으면 GitHub Actions가 자동 커밋/푸시하는 단계 추가
- Windows 환경에서 `cargo build --release` 후 `oshit.exe`를 zip으로 압축하는 단계 추가
- GitHub CLI로 태그 생성, 릴리즈 노트 작성, Release 생성 단계 추가

## 테스트 계획

- YAML 문법 파싱 확인
- `prepare-release-version` 테스트 실행
- 워크플로우 파일의 주요 필드와 step 구조 검토

## 비고

- 현재 저장소는 Windows 전용 유틸리티이므로 Linux/macOS 빌드 매트릭스는 추가하지 않는다.
- 릴리즈 태그는 `v0.1.0` 형식을 가정하고 `Cargo.toml`에는 `0.1.0`으로 반영한다.
