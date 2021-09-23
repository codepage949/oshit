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