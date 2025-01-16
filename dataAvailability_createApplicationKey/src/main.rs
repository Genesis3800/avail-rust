use avail_rust::{Key, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;
 
#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await.unwrap();

    let alice = "This is a random seed phrase please replace with your own";
	let secret_uri = SecretUri::from_str(alice).unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let key = String::from("MyAwesome43287oerwhbiwKey").as_bytes().to_vec();
	let key = Key { 0: key };
    let options = Options::new().nonce(Nonce::BestBlockAndTxPool);

	let result = sdk
		.tx
		.data_availability
		.create_application_key(key, WaitFor::BlockInclusion, &account, Some(options))
		.await?;
 
	println!(
		"Key={:?}, Owner={}, Id={:?}",
		result.event.key, result.event.owner, result.event.id
	);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);
 
	Ok(())
}