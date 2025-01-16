use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;
 
#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await.unwrap();
 
	let alice = "This is a random seed phrase please replace with your own";
	let secret_uri = SecretUri::from_str(alice).unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let value = 1_000_000_000_000_000_000u128; // 1 Avail
    let options = Options::new().nonce(Nonce::BestBlockAndTxPool);

	let result = sdk
		.tx
		.staking
		.unbond(value, WaitFor::BlockInclusion, &account, Some(options))
		.await?;
 
	println!(
		"Stash={}, Amount={:?}",
		result.event.stash, result.event.amount
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);
 
	Ok(())
}