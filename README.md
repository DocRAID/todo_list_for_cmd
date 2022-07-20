# Rust_todoList

~~더러운 코드와 함께~~ 간단하게 완성해본 Todo list 임. <br>

powershell이나 자신의 쉘에 등록을 하면 간편하게 쓸수 있음.<br>
+ 윈도우 환경 (on Powershell profile)
  + Set-Alias todo C:\[설치경로]\todoList.exe
+ 리눅스 환경 (on shell)
  + alias todo=[설치경로]\todoList
 
 와 같은 방법으로 사용.
 
#명령어
+ todo add [your todo] : input your Todos
+ todo rm [todo's id] : delete your Todos
+ todo ls : show your todo's id and Todos
+ todo clear : clear Todos.

기본 Todo.txt 의 기본 경로는 사용자의 문서폴더에 있습니다. (없다면 만들어주세요)
