package main

import (
	"bytes"
	"log"

	x11 "github.com/linuxdeepin/go-x11-client"
)

type PTY struct {
	Master int
	Slave  int
}

type X11 struct {
	FileDescriptor int
	Display        x11.Conn
	Screen         int
	Root           x11.Window

	Termwin x11.Window
	TermGC  x11.GContext
	Col_fg  uint
	Col_bg  uint
	W       int
	H       int

	Xfont       x11.FontProp
	Font_width  int
	Font_height int

	Buffer bytes.Buffer
	Buf_w  int
	Buf_h  int
	Buf_x  int
	Buf_y  int
}

type XColor struct{}

func (x *X11) x11_setup() error {
	var Cmap x11.Colormap
	var Color XColor

	return nil
}

func main() {
	var pty PTY
	var x11 X11

	err := x11.x11_setup()
	if err != nil {
		log.Fatalf("Error: [%v]", err)
		return
	}
}
