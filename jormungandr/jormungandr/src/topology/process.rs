use super::{Gossip, Gossips, P2pTopology, Peer};
use crate::intercom::{NetworkMsg, PropagateMsg, TopologyMsg};
use crate::settings::start::network::Configuration;
use crate::utils::async_msg::{MessageBox, MessageQueue};
use tokio::time::Interval;
use tokio_stream::StreamExt;

struct Process {
    input: MessageQueue<TopologyMsg>,
    network_msgbox: MessageBox<NetworkMsg>,
    gossip_interval: Interval,
    topology: P2pTopology,
}

pub struct TaskData {
    pub network_msgbox: MessageBox<NetworkMsg>,
    pub topology_queue: MessageQueue<TopologyMsg>,
    pub initial_peers: Vec<Peer>,
    pub config: Configuration,
}

pub async fn start(task_data: TaskData) {
    let TaskData {
        network_msgbox,
        topology_queue,
        initial_peers,
        config,
    } = task_data;

    let mut topology = P2pTopology::new(&config);

    topology.accept_gossips(Gossips::from(
        initial_peers
            .into_iter()
            .map(Gossip::from)
            .collect::<Vec<_>>(),
    ));

    let mut process = Process {
        input: topology_queue,
        gossip_interval: tokio::time::interval(config.gossip_interval),
        network_msgbox,
        topology,
    };
    process.handle_input().await;
}

impl Process {
    async fn handle_input(&mut self) {
        loop {
            tokio::select! {
                Some(input) = self.input.next() => {
                    match input {
                        TopologyMsg::AcceptGossip(gossip) => self.topology.accept_gossips(gossip),
                        TopologyMsg::DemotePeer(id) => self.topology.report_node(&id),
                        TopologyMsg::PromotePeer(id) => self.topology.promote_node(&id),
                        TopologyMsg::View(selection, handle) => {
                            handle.reply_ok(self.topology.view(selection))
                        }
                        TopologyMsg::ListAvailable(handle) => {
                            handle.reply_ok(self.topology.list_available())
                        }
                        TopologyMsg::ListNonPublic(handle) => {
                            handle.reply_ok(self.topology.list_non_public())
                        }
                        TopologyMsg::ListQuarantined(handle) => {
                            handle.reply_ok(self.topology.list_quarantined())
                        }
                    }
                },
                _ = self.gossip_interval.tick() => {
                        self.topology.update_gossip();
                        let view = self.topology.view(poldercast::layer::Selection::Any);
                        for peer in view.peers {
                            // Peers returned by the topology will always have a NodeId
                            let gossip = self.topology.initiate_gossips(&peer.id());

                            self.network_msgbox
                                // do not block the current thread to avoid deadlocks
                                .try_send(NetworkMsg::Propagate(PropagateMsg::Gossip(peer, gossip)))
                                .unwrap_or_else(|e| {
                                    tracing::error!(
                                        reason = ?e,
                                        "cannot send PropagateMsg request to network"
                                    )
                                });
                        }
                    }
            }
        }
    }
}
