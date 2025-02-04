use alloy::{
    eips::eip7702::Authorization,
    network::{EthereumWallet, TransactionBuilder, TransactionBuilder7702},
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{local::PrivateKeySigner, SignerSync},
    sol,
};
use dotenv::dotenv;
use eyre::Result;
use std::env;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    Counter,
    "foundry/out/Counter.sol/Counter.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL")
        .expect("RPC_URL must be set in .env file")
        .parse()?;
    let signer: PrivateKeySigner = env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY must be set in .env file")
        .parse()?;

    let wallet = EthereumWallet::from(signer.clone());

    // let provider = ProviderBuilder::new().disable_recommended_fillers().wallet(wallet).on_http(rpc_url);
    let provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);
    let latest_block = provider.get_block_number().await?;
    println!("Latest block number: {latest_block}");

    let contract = Counter::deploy(&provider).await?;
    println!("Deployed contract at address: {}", contract.address());

    let chain_id = provider.get_chain_id().await?;

    let authorization = Authorization {
        chain_id: U256::from(chain_id),
        // Reference to the contract that will be set as code for the authority.
        address: *contract.address(),
        nonce: provider
            .get_transaction_count(signer.clone().address())
            .await?
            + 1,
    };

    let signed_authorization = authorization.clone().into_signed(
        signer
            .clone()
            .sign_hash_sync(&authorization.signature_hash())?,
    );

    let tx = TransactionRequest::default()
        .with_to(address!("0x0000000000000000000000000000000000000000"))
        .with_authorization_list(vec![signed_authorization]);
    let pending_tx = provider.send_transaction(tx).await?;
    println!("Pending transaction... {}", pending_tx.tx_hash());
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    let instance = Counter::new(signer.clone().address(), provider.clone());
    let mut count = instance.number().call().await?;
    println!("Count on {:?}: {:?}", instance.address(), count);

    let pending_increase = instance.setNumber(U256::from(7702)).send().await?;
    let increase_receipt = pending_increase.get_receipt().await?;
    println!(
        "Transaction included in block {}",
        increase_receipt
            .block_number
            .expect("Failed to get block number")
    );

    count = instance.number().call().await?;
    println!("Count on {:?} now: {:?}", instance.address(), count);

    let eoa_bytecode = provider.get_code_at(signer.clone().address()).await?;

    println!("Bytecode: {eoa_bytecode:?}");

    let redeployed_contract = Counter::deploy(&provider).await?;
    println!(
        "Redeployed contract at address: {}",
        redeployed_contract.address()
    );

    let reauthorization = Authorization {
        chain_id: U256::from(chain_id),
        // Reference to the contract that will be set as code for the authority.
        address: *redeployed_contract.address(),
        nonce: provider
            .get_transaction_count(signer.clone().address())
            .await?
            + 1,
    };
    let signed_reauthorization = reauthorization.clone().into_signed(
        signer
            .clone()
            .sign_hash_sync(&reauthorization.signature_hash())?,
    );

    let tx = TransactionRequest::default()
        .with_to(address!("0x0000000000000000000000000000000000000000"))
        .with_authorization_list(vec![signed_reauthorization]);
    let pending_tx = provider.send_transaction(tx).await?;
    println!("Pending transaction... {}", pending_tx.tx_hash());
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    let eoa_bytecode = provider.get_code_at(signer.clone().address()).await?;

    println!("Bytecode: {eoa_bytecode:?}");

    count = instance.number().call().await?;
    println!("Count on {:?} now: {:?}", instance.address(), count);

    assert!(receipt.status());
    Ok(())
}
