use std::sync::Arc;
use solana_sdk::signature::Signer;
use tokio::sync::RwLock;
use crate::{send_and_confirm::ComputeBudget, utils::proof_pubkey, Miner};
use crate::jito::{JitoTips, subscribe_jito_tips};

impl Miner {
    pub async fn open(&self) {
        // Return early if miner is already registered
        let signer = self.signer();
        let proof_address = proof_pubkey(signer.pubkey());
        if self.rpc_client.get_account(&proof_address).await.is_ok() {
            return;
        }

        // Sign and send transaction.
        println!("Generating challenge...");
        let ix = ore_api::instruction::open(signer.pubkey(), signer.pubkey(), signer.pubkey());

        // self.send_and_confirm(&[ix], ComputeBudget::Dynamic, false)
        //     .await
        //     .ok();
        let tips = Arc::new(RwLock::new(JitoTips::default()));
        subscribe_jito_tips(tips.clone()).await;
        let tips = *tips.read().await;
        let mut tip = self.priority_fee;
        tip = 20000.max(tips.p50() + 1);
        self.send_and_confirm_by_jito(&[ix], ComputeBudget::Dynamic, tip)
            .await;
    }
}
