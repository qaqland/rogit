package main

import (
	"flag"
	"fmt"
)

var R *Repo

func main() {
	fmt.Println("hello world")

	host := flag.String("host", "0.0.0.0", "Host to listen on")
	port := flag.String("port", "8088", "Port to listen on")

	flag.Parse()
	fmt.Println("flag:", *host, *port)
	R = Init("../alpinelinux/aports")
	// R = Init(".")
	// Serve(*host, *port)
}
