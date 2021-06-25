package main

import (
    "context"
    "fmt"
    "os"
    "time"

    "google.golang.org/grpc"

    "github.com/oasisprotocol/oasis-core/go/common"
    cmnGrpc "github.com/oasisprotocol/oasis-core/go/common/grpc"
    "github.com/oasisprotocol/oasis-core/go/common/logging"
    "github.com/oasisprotocol/oasis-core/go/common/quantity"

    "github.com/oasisprotocol/oasis-sdk/client-sdk/go/client"
    "github.com/oasisprotocol/oasis-sdk/client-sdk/go/modules/accounts"
    "github.com/oasisprotocol/oasis-sdk/client-sdk/go/testing"
    "github.com/oasisprotocol/oasis-sdk/client-sdk/go/types"
)

// In reality these would come from command-line arguments, the environment
// or a configuration file.
const (
    // This is the default runtime ID as used in oasis-net-runner. It can
    // be changed by using its --fixture.default.runtime.id argument.
    runtimeIDHex = "8000000000000000000000000000000000000000000000000000000000000000"
    // This is the default client node address as set in oasis-net-runner.
    nodeAddress = "unix:/tmp/rt-test/net-runner/network/client-0/internal.sock"
)

// The global logger.
var logger = logging.GetLogger("minimal-runtime-client")

// Client contains the client helpers for communicating with the runtime. This is a simple wrapper
// used for convenience.
type Client struct {
    client.RuntimeClient

    // Accounts are the accounts module helpers.
    Accounts accounts.V1
}

// showBalances is a simple helper for displaying account balances.
func showBalances(ctx context.Context, rc *Client, address types.Address) {
    // Query the runtime, specifically the accounts module, for the given address' balances.
    rsp, err := rc.Accounts.Balances(ctx, client.RoundLatest, address)
    if err != nil {
        logger.Error("failed to fetch account balances",
            "err", err,
        )
        os.Exit(1)
    }

    fmt.Printf("=== Balances for %s ===\n", address)
    for denom, balance := range rsp.Balances {
        fmt.Printf("%s: %s\n", denom, balance)
    }
    fmt.Printf("\n")
}

func main() {
    // Initialize logging.
    if err := logging.Initialize(os.Stdout, logging.FmtLogfmt, logging.LevelDebug, nil); err != nil {
        fmt.Fprintf(os.Stderr, "ERROR: Unable to initialize logging: %v\n", err)
        os.Exit(1)
    }

    // Decode hex runtime ID into something we can use.
    var runtimeID common.Namespace
    if err := runtimeID.UnmarshalHex(runtimeIDHex); err != nil {
        logger.Error("malformed runtime ID",
            "err", err,
        )
        os.Exit(1)
    }

    // Establish a gRPC connection with the client node.
    logger.Info("connecting to local node")
    conn, err := cmnGrpc.Dial(nodeAddress, grpc.WithInsecure())
    if err != nil {
        logger.Error("failed to establish connection",
            "addr", nodeAddress,
            "err", err,
        )
        os.Exit(1)
    }
    defer conn.Close()

    // Create the runtime client with account module query helpers.
    c := client.New(conn, runtimeID)
    rc := &Client{
        RuntimeClient: c,
        Accounts:      accounts.NewV1(c),
    }

    ctx, cancelFn := context.WithTimeout(context.Background(), 30*time.Second)
    defer cancelFn()

    // Show initial balances for Alice's and Bob's accounts.
    logger.Info("dumping initial balances")
    showBalances(ctx, rc, testing.Alice.Address)
    showBalances(ctx, rc, testing.Bob.Address)

    // Get current nonce for Alice's account.
    nonce, err := rc.Accounts.Nonce(ctx, client.RoundLatest, testing.Alice.Address)
    if err != nil {
        logger.Error("failed to fetch account nonce",
            "err", err,
        )
        os.Exit(1)
    }

    // Perform a transfer from Alice to Bob.
    logger.Info("performing transfer", "nonce", nonce)
    // Create a transfer transaction with Bob's address as the destination and 10 native base units
    // as the amount.
    tb := rc.Accounts.Transfer(
            testing.Bob.Address,
            types.NewBaseUnits(*quantity.NewFromUint64(10), types.NativeDenomination),
        ).
        // Configure gas as set in genesis parameters. We could also estimate it instead.
        SetFeeGas(100).
        // Append transaction authentication information using a single signature variant.
        AppendAuthSignature(testing.Alice.Signer.Public(), nonce)
    // Sign the transaction using the signer. Before a transaction can be submitted it must be
    // signed by all configured signers. This will automatically fetch the corresponding chain
    // domain separation context for the runtime.
    if err = tb.AppendSign(ctx, testing.Alice.Signer); err != nil {
        logger.Error("failed to sign transfer transaction",
            "err", err,
        )
        os.Exit(1)
    }
    // Submit the transaction and wait for it to be included and a runtime block.
    if err = tb.SubmitTx(ctx, nil); err != nil {
        logger.Error("failed to submit transfer transaction",
            "err", err,
        )
        os.Exit(1)
    }

    // Show final balances for Alice's and Bob's accounts.
    logger.Info("dumping final balances")
    showBalances(ctx, rc, testing.Alice.Address)
    showBalances(ctx, rc, testing.Bob.Address)
}
