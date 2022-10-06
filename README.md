# oshit

윈도우에서 왼쪽 시프트키를 통해 한영 전환을 할 수있게 도와주는 유틸리티

# 빌드

```
go build -ldflags -H=windowsgui -o oshit.exe ./main
```

# 제한

* `oshit`이 실행된 권한보다 높은 권한으로 실행된 프로그램에선 동작하지 않음

# 메모

* ref https://stackoverflow.com/questions/64280975/immgetcontext-returns-zero-always
* 시프트 단일키는 RegisterHotKey로 핫키 등록이 불가해 전역 키보드 훅을 사용함
* 22H2 업데이트 후 한영 전환이 되지 않는 현상 발생시
  * 설정 > 시간 및 언어 > 한국어::언어 옵션 > Microsoft 입력기::키보드 옵션에서 이번 버전 IME 사용 활성화
  * <img width="340" alt="image" src="https://user-images.githubusercontent.com/26968918/194363138-94a9ce10-9c58-43df-8273-7ce4d610ef54.png">
