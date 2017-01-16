package main

import (
	"fmt"
	"github.com/hypebeast/go-osc/osc"
	"github.com/lpil/sonic-pi-tools/log"
	"os"
)

func main() {
	server := &osc.Server{Addr: "localhost:4558"}

	try(server.Handle("/log/info", func(msg *osc.Message) {
		fmt.Print(log.Info(msg.Arguments))
	}))
	try(server.Handle("/log/multi_message", func(msg *osc.Message) {
		fmt.Print(log.MultiMessage(msg.Arguments))
	}))
	try(server.Handle("/syntax_error", func(msg *osc.Message) {
		fmt.Print(log.SyntaxError(msg.Arguments))
	}))
	try(server.Handle("/error", func(msg *osc.Message) {
		fmt.Print(log.Error(msg.Arguments))
	}))
	// server.Handle("*", func(msg *osc.Message) {
	//	fmt.Println(msg)
	// })

	server.ListenAndServe()
}

func try(err error) {
	if err != nil {
		fmt.Println("ERROR:", err)
		os.Exit(1)
	}
}
