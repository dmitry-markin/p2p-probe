use anyhow::{anyhow, Context};
use clap::Parser;
use futures::StreamExt;
use litep2p::{
    config::ConfigBuilder,
    error::Error,
    protocol::libp2p::identify::{Config as IdentifyConfig, IdentifyEvent},
    transport::{tcp::config::Config as TcpConfig, websocket::config::Config as WsConfig},
    Litep2p, Litep2pEvent, PeerId,
};
use multiaddr::{Multiaddr, Protocol};

/// Probe a Substrate node and collect the Identify protocol information.
#[derive(Parser, Debug)]
struct Args {
    /// Address of a Substrate node to probe, optionally including the `/p2p/...` part.
    #[arg(value_name = "MULTIADDR")]
    address: Multiaddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let (identify_config, mut identify_event_stream) = IdentifyConfig::new(
        "/substrate/1.0".to_string(),
        Some("p2p-probe".to_string()),
        Vec::new(),
    );

    let mut litep2p = Litep2p::new(
        ConfigBuilder::new()
            .with_tcp(TcpConfig { listen_addresses: Vec::new(), ..Default::default() })
            .with_websocket(WsConfig { listen_addresses: Vec::new(), ..Default::default() })
            .with_libp2p_identify(identify_config)
            .build(),
    )
    .context("litep2p initialization error")?;

    let detect_peer_id = !matches!(args.address.iter().last(), Some(Protocol::P2p(_)));
    let mut address = if detect_peer_id {
        args.address.clone().with(Protocol::P2p(PeerId::random().into()))
    } else {
        args.address.clone()
    };

    litep2p.dial_address(address.clone()).await.context("connect error")?;

    loop {
        tokio::select! {
            event = litep2p.next_event() => {
                match event {
                    Some(Litep2pEvent::DialFailure { address: _, error }) => {
                        if detect_peer_id {
                            match error {
                                Error::PeerIdMismatch(_, peer_id) => {
                                    address = args
                                        .address
                                        .clone()
                                        .with(Protocol::P2p(peer_id.into()));
                                    litep2p
                                        .dial_address(address.clone())
                                        .await
                                        .context("connect error")?;
                                },
                                _ => {
                                    return Err(error).context("dial failure")
                                }
                            }
                        } else {
                            return Err(error).context("dial failure")
                        }
                    },
                    None => return Err(anyhow!("litep2p event loop has terminated")),
                    _ => {},
                }
            },
            event = identify_event_stream.next() => {
                if let Some(event) = event {
                    print_report(address, event);
                    break
                } else {
                    return Err(anyhow!("Identify stream has terminated"))
                }
            }
        }
    }

    Ok(())
}

fn print_report(address: Multiaddr, event: IdentifyEvent) {
    let IdentifyEvent::PeerIdentified {
        peer,
        protocol_version,
        user_agent,
        supported_protocols,
        observed_address,
        listen_addresses,
    } = event;

    println!("multiaddr: {address}");
    println!("peer_id: {peer}");
    println!("observed_address: {observed_address}");
    println!("protocol_version: {:?}", protocol_version.unwrap_or_default());
    println!("user_agent: {:?}", user_agent.unwrap_or_default());
    println!(
        "supported_protocols: {:#?}",
        supported_protocols.iter().map(ToString::to_string).collect::<Vec<_>>()
    );
    println!("listen_addresses: {:#?}", listen_addresses);
}
