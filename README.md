# oshit

윈도우에서 왼쪽 시프트키를 통해 한영 전환을 할 수있게 도와주는 유틸리티

# 빌드

```
go build -ldflags -H=windowsgui -o oshit.exe ./main
```

# 제한

* `oshit`이 실행된 권한보다 높은 권한으로 실행된 프로그램에선 동작하지 않음

# 메모

* SendInput 보다 ImmSimulateHotKey나 WM_INPUTLANGCHANGEREQUEST를 사용하고 싶었지만 예상대로 동작하지 않아 포기. 최종적으론 보다 매끄러운 처리를 위해선 두 가지 중 하나를 적용하는 것이 바람직해 보임
* 시프트 단일키는 RegisterHotKey로 핫키 등록이 불가해 전역 키보드 훅을 사용함