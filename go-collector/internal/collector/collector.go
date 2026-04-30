package collector

import (
	"context"
	"encoding/hex"
	"fmt"
	"strings"

	"upgrade-sentinel/go-collector/internal/slots"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/ethclient"
)

type SlotResult struct {
	SlotName   string `json:"slot_name"`
	SlotKey    string `json:"slot_key"`
	RawValue   string `json:"raw_value"`
	Address    string `json:"address"`
	IsZero     bool   `json:"is_zero"`
}

type Report struct {
	TargetAddress string       `json:"target_address"`
	CodeSize      int          `json:"code_size"`
	HasCode       bool         `json:"has_code"`
	Implementation SlotResult  `json:"implementation"`
	Admin          SlotResult  `json:"admin"`
}

func CollectEIP1967(ctx context.Context, rpcURL, target string) (*Report, error) {
	client, err := ethclient.DialContext(ctx, rpcURL)
	if err != nil {
		return nil, fmt.Errorf("failed to connect RPC: %w", err)
	}
	defer client.Close()

	if !common.IsHexAddress(target) {
		return nil, fmt.Errorf("invalid target address: %s", target)
	}

	targetAddr := common.HexToAddress(target)

	code, err := client.CodeAt(ctx, targetAddr, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch code: %w", err)
	}

	implResult, err := readSlot(ctx, client, targetAddr, "implementation", slots.ImplementationSlot)
	if err != nil {
		return nil, err
	}

	adminResult, err := readSlot(ctx, client, targetAddr, "admin", slots.AdminSlot)
	if err != nil {
		return nil, err
	}

	report := &Report{
		TargetAddress:  targetAddr.Hex(),
		CodeSize:       len(code),
		HasCode:        len(code) > 0,
		Implementation: *implResult,
		Admin:          *adminResult,
	}

	return report, nil
}

func readSlot(ctx context.Context, client *ethclient.Client, target common.Address, slotName, slotHex string) (*SlotResult, error) {
	slotKey := common.HexToHash(slotHex)

	raw, err := client.StorageAt(ctx, target, slotKey, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to read %s slot: %w", slotName, err)
	}

	rawHex := "0x" + hex.EncodeToString(raw)
	addr := storageWordToAddress(raw)
	isZero := isZeroWord(raw)

	return &SlotResult{
		SlotName: slotName,
		SlotKey:  slotHex,
		RawValue: rawHex,
		Address:  addr.Hex(),
		IsZero:   isZero,
	}, nil
}

func storageWordToAddress(word []byte) common.Address {
	if len(word) < 20 {
		return common.Address{}
	}
	return common.BytesToAddress(word[len(word)-20:])
}

func isZeroWord(word []byte) bool {
	return strings.TrimLeft(hex.EncodeToString(word), "0") == ""
}
