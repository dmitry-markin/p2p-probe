# p2p-probe

Probe a libp2p-compatible node and report the `Identify` protocol information.

Example usage for a Polkadot node:
```
p2p-probe /ip4/127.0.0.1/tcp/30333/ws  # for a full node
p2p-probe /ip4/127.0.0.1/tcp/30333     # for a validator
```

## Docker
Example usage:
```shell
docker build . -t p2p-probe 
docker run p2p-probe /dns/polkadot-bootnode-0.polkadot.io/tcp/30333
```

## Example output
```
$ p2p-probe /dns/polkadot-bootnode-0.polkadot.io/tcp/30333

multiaddr: /dns/polkadot-bootnode-0.polkadot.io/tcp/30333/p2p/12D3KooWSz8r2WyCdsfWHgPyvD8GKQdJ1UAiRmrcrs8sQB3fe2KU
peer_id: 12D3KooWSz8r2WyCdsfWHgPyvD8GKQdJ1UAiRmrcrs8sQB3fe2KU
observed_address: /ip4/1.2.3.4/tcp/41384
protocol_version: "/substrate/1.0"
user_agent: "Parity Polkadot/v1.10.0-7049c3c9883 (polkadot-bootnode-0)"
supported_protocols: [
    "/dot/sync/2",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/send_dispute/1",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_chunk/1",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/light/2",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/kad",
    "/polkadot/req_available_data/1",
    "/polkadot/send_dispute/1",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/transactions/1",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/beefy/2",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_available_data/1",
    "/dot/state/2",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/state/2",
    "/ipfs/id/1.0.0",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_statement/1",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_pov/1",
    "/polkadot/req_chunk/1",
    "/dot/light/2",
    "/dot/transactions/1",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/sync/warp",
    "/ipfs/id/push/1.0.0",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_collation/2",
    "/dot/block-announces/1",
    "/dot/kad",
    "/paritytech/grandpa/1",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/sync/2",
    "/polkadot/req_collation/1",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_collation/1",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/beefy/justifications/1",
    "/polkadot/req_pov/1",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/req_attested_candidate/2",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/block-announces/1",
    "/dot/sync/warp",
    "/polkadot/req_statement/1",
    "/91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3/grandpa/1",
    "/ipfs/ping/1.0.0",
]
listen_addresses: [
    "/ip4/127.0.0.1/tcp/30333",
    "/ip4/10.1.1.19/tcp/30334/ws",
    "/ip4/10.1.1.19/tcp/30333",
    "/ip6/2001:41d0:700:1d85::/tcp/30334/ws",
    "/ip4/51.75.144.133/tcp/30334/ws",
    "/ip6/fe80::a6bf:1ff:fe28:da8e/tcp/30334/ws",
    "/ip4/51.75.144.133/tcp/30333",
    "/ip4/127.0.0.1/tcp/30334/ws",
    "/ip6/::1/tcp/30334/ws",
    "/dns/51.75.144.133/tcp/30333",
    "/ip6/2001:41d0:700:1d85::/tcp/30334/ws",
    "/ip4/51.75.144.133/tcp/30334/ws",
    "/dns4/cc1-0.parity.tech/tcp/30334/ws",
    "/ip4/51.75.144.133/tcp/30333",
    "/ip6/2001:41d0:700:1d85::/tcp/30333",
    "/dns/cc1-0.parity.tech/tcp/30333",
    "/dns4/cc1-0.parity.tech/tcp/30333",
    "/dns/cc1-0.parity.tech/tcp/30334/ws",
    "/dns/51.75.144.133/tcp/30334/ws",
]
```
