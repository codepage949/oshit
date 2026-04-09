# GitHub Pages 랜딩 페이지 추가

## 구현 계획

- `docs/index.html` 단일 파일 기반 GitHub Pages 사이트를 추가한다.
- 프로젝트 목표가 바로 드러나는 랜딩 페이지를 구성한다.
- 최신 릴리즈 버전과 다운로드 링크를 GitHub Releases API로 동적으로 표시한다.
- 별도 GitHub Actions 워크플로우 없이 GitHub Pages 기본 배포 방식을 사용한다.

## 변경 사항

- `docs/index.html` 추가
- 인라인 CSS/JavaScript로 페이지를 단일 파일로 구성
- 별도 GitHub Pages workflow 파일 없이 `docs/` 배포 구조로 정리
- 최신 릴리즈 버전과 다운로드 버튼을 상단 우측에 배치
- 브랜드 라벨 `oshit`과 메인 상태 텍스트 `오싯!`/`OSHIT!` 중심의 단순한 레이아웃 적용
- 초기 상태를 `OSHIT!`으로 시작하도록 구성
- 실제 `LShift` 단독 입력 시 `오싯!`/`OSHIT!` 상태가 전환되는 상호작용 추가
- 전환 시 새 사선 그라데이션이 이전 그라데이션을 덮어쓰는 방식의 색상 전환 효과 적용
- `LShift` 아이콘과 안내 문구를 통해 데모 입력 유도
- 포커스 복구와 상태 초기화 로직을 포함한 브라우저 데모 입력 안정성 보강

## 테스트 계획

- HTML 내 JavaScript 문법 검사
- 정적 파일 구조와 링크 연결 검토
- `LShift` 단독 입력 판정 로직 검토
- `오싯!`/`OSHIT!` 상태 전환과 사선 그라데이션 전환 효과 검토
- 브라우저 포커스 이탈 시 데모 입력 복구 동작 검토

## 비고

- README 등 기존 문서에는 GitHub Pages 안내를 추가하지 않는다.
- 최신 버전 표시는 클라이언트에서 GitHub Releases API를 호출해 처리한다.
