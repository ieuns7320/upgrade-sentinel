package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
    rpcURL := flag.String("rpc", "", "RPC URL")
    address := flag.String("address", "", "Target contract address")
    flag.Parse()

    fmt.Println("UpgradeSentinel Go Collector")
    fmt.Printf("rpc: %s\n", *rpcURL)
    fmt.Printf("address: %s\n", *address)

    if *rpcURL == "" || *address == "" {
        fmt.Fprintln(os.Stderr, "usage: go run . --rpc <RPC_URL> --address <ADDRESS>")
        os.Exit(1)
    }
}

