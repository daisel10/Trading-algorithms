// Engine orchestrator - coordinates all the "organs"

use tokio::sync::{broadcast, mpsc};
use kairos_domain::{MarketTick, InternalOrder};

pub struct TradingEngine {
    // Broadcast channel for market data (The Feed Handler -> Everyone)
    market_data_tx: broadcast::Sender<MarketTick>,
    
    // MPSC channel for orders (Strategies -> Risk Engine)
    order_tx: mpsc::Sender<InternalOrder>,
    order_rx: mpsc::Receiver<InternalOrder>,
}

impl TradingEngine {
    pub fn new() -> Self {
        let (market_data_tx, _) = broadcast::channel(1000);
        let (order_tx, order_rx) = mpsc::channel(100);

        Self {
            market_data_tx,
            order_tx,
            order_rx,
        }
    }

    /// Returns a subscriber to market data
    pub fn subscribe_market_data(&self) -> broadcast::Receiver<MarketTick> {
        self.market_data_tx.subscribe()
    }

    /// Returns a sender for orders
    pub fn get_order_sender(&self) -> mpsc::Sender<InternalOrder> {
        self.order_tx.clone()
    }

    /// Main engine loop
    pub async fn run(mut self) -> anyhow::Result<()> {
        loop {
            tokio::select! {
                Some(order) = self.order_rx.recv() => {
                    // TODO: Pass order to Risk Engine
                    tracing::debug!("Received order: {:?}", order);
                }
                else => break,
            }
        }
        Ok(())
    }
}

impl Default for TradingEngine {
    fn default() -> Self {
        Self::new()
    }
}
