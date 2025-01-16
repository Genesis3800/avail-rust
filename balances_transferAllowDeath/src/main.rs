use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;
 
#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await.unwrap();
 
    let alice = "This is a random seed phrase please replace with your own";
	let secret_uri = SecretUri::from_str(alice).unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
	let amount = 1_000_000_000_000_000_00u128; // 1 Avail
    let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
 
	let result = sdk
		.tx
		.balances
		.transfer_allow_death(dest, amount, WaitFor::BlockInclusion, &account, Some(options))
		.await?;
 
	println!(
		"From={}, To={}, Amount={}",
		result.event.from, result.event.to, result.event.amount
	);
	if let Some(event) = result.event2 {
		println!("Killed={}", event.account);
	}
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);
 
	Ok(())
}