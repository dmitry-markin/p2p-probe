# p2p-probe

Probe a Substrate node and collect the Identify protocol information.

Example usage:
```
p2p-probe /ip4/127.0.0.1/tcp/30333/ws  # for full node
p2p-probe /ip4/127.0.0.1/tcp/30333     # for validator
```

## Docker
Example usage:
```shell
docker build . -t p2p-probe 
docker run p2p-probe /dns/polkadot-bootnode-0.polkadot.io/tcp/30333/p2p
```