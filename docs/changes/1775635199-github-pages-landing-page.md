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
- 큰 `LShift` 키캡과 한/영 상태 배지 토글 애니메이션 중심 메인 비주얼 적용
- 실제 `LShift` 단독 입력 시 데모 상태가 바뀌는 상호작용 추가
- 자동 반복 애니메이션 제거, 실제 입력 기반 상호작용만 유지
- `Input State` 대신 한/A 커서가 있는 타이핑 데모 카드 적용
- 타이핑 데모를 제거하고 한/A 슬라이드 전환 카드로 단순화
- 상태 텍스트를 `오싯!`/`OSHIT!`로 바꾸고 클리핑 기반 전환 효과 적용

## 테스트 계획

- HTML 내 JavaScript 문법 검사
- 정적 파일 구조와 링크 연결 검토
- CSS 애니메이션과 상태 토글 표현 검토
- `LShift` 단독 입력 판정 로직 검토
- 자동 애니메이션 제거 후 기본 상태 표현 검토
- 타이핑 데모 진행과 상태 전환 타이밍 검토
- 한/A 슬라이드 전환 애니메이션 검토
- `오싯!`/`OSHIT!` 클리핑 전환 효과 검토

## 비고

- README 등 기존 문서에는 GitHub Pages 안내를 추가하지 않는다.
- 최신 버전 표시는 클라이언트에서 GitHub Releases API를 호출해 처리한다.
