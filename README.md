# Termi

A terminal written in Rust.

From a very long time i have been wondering as to how do terminals work and programs do send requests.
After reading [wayland](https://way-cooler.org/book/wayland_introduction.html) i started thinking as to how would this work.
This is a project to learn while trying to build. Lets see how far this goes before i delete this in shame.

[Info](https://poor.dev/blog/terminal-anatomy/)

Initially thought of writing this in Golang but Golang does not have forkpty coz threads and gorutines exec and handling of errno. You can still do it with Cgo but i am not paid. So no going to Cgo.
For more info -> [Go](https://groups.google.com/g/golang-nuts/c/Be8h4q9AoD4?pli=1)
