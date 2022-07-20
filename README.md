# Rust_todoList

~~더러운 코드와 함께~~ 내가 쓰려고 간단하게(?) 완성해본 Todo list 임. <br>

powershell이나 자신의 쉘에 등록을 하면 간편하게(?) 쓸수 있음.<br>
+ 윈도우 환경 (on Powershell profile)
  + Set-Alias todo C:\[설치경로]\todoList.exe
+ 리눅스 환경 (on shell)
  + alias todo=[설치경로]\todoList
 
 와 같은 방법으로 사용.
 
### 명령어
+ todo : 명령어를 보여줍니다.
+ todo add [your todo] : [your todo] 에 작성된 할 일을 추가합니다.
+ todo rm [todo's id] : todd id를 기반으로 삭제합니다.
+ todo ls : 작성하고 추가한 할 일을 모두 보여줍니다.
+ todo clear : 리스트 초기화.

기본 Todo.txt 의 기본 경로는 사용자의 document directory에 있습니다. (없다면 만들어주세요)
***
_Result : Rust 기본 문법을 익히고 Rust 파일 시스템, osString, 오류제어 문법을 다루어 보며 실습함._
<br>
_Todo : 러스트를 더 배우며 코드 최적화._
<br>
~~아 코드 개더럽다~~
