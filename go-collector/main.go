package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"time"

	"upgrade-sentinel/go-collector/internal/collector"
)

func main() {
	rpcURL := flag.String("rpc", "", "RPC URL")
	address := flag.String("address", "", "Target proxy address")
	timeoutSec := flag.Int("timeout", 10, "Timeout in seconds")
	flag.Parse()

	if *rpcURL == "" || *address == "" {
		fmt.Fprintln(os.Stderr, "usage: go run . --rpc <RPC_URL> --address <ADDRESS> [--timeout 10]")
		os.Exit(1)
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Duration(*timeoutSec)*time.Second)
	defer cancel()

	report, err := collector.CollectEIP1967(ctx, *rpcURL, *address)
	if err != nil {
		fmt.Fprintf(os.Stderr, "collector error: %v\n", err)
		os.Exit(1)
	}

	fmt.Println("EIP-1967 Inspection Result")
	fmt.Printf("Target: %s\n", report.TargetAddress)
	fmt.Printf("Has code: %v\n", report.HasCode)
	fmt.Printf("Code size: %d bytes\n", report.CodeSize)

	fmt.Println("\nImplementation Slot")
	fmt.Printf("- slot key: %s\n", report.Implementation.SlotKey)
	fmt.Printf("- raw value: %s\n", report.Implementation.RawValue)
	fmt.Printf("- address: %s\n", report.Implementation.Address)
	fmt.Printf("- is zero: %v\n", report.Implementation.IsZero)

	fmt.Println("\nAdmin Slot")
	fmt.Printf("- slot key: %s\n", report.Admin.SlotKey)
	fmt.Printf("- raw value: %s\n", report.Admin.RawValue)
	fmt.Printf("- address: %s\n", report.Admin.Address)
	fmt.Printf("- is zero: %v\n", report.Admin.IsZero)

	fmt.Println("\nJSON:")
	out, err := json.MarshalIndent(report, "", "  ")
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to marshal report: %v\n", err)
		os.Exit(1)
	}
	fmt.Println(string(out))
}
