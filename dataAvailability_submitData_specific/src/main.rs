use avail_rust::{Data, Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;
 
#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await.unwrap();
 
    let alice = "This is a random seed phrase please replace with your own";
	let secret_uri = SecretUri::from_str(alice).unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let data = String::from("Sample data being submitted......").as_bytes().to_vec();
	let data = Data { 0: data };
 
	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
 
	let app_id = 89; // You can change this value to your desired app_id
 
    // Create the options with the app_id included
    let options = Options {
        app_id: Some(app_id),
        mortality: None,
        nonce: Some(Nonce::BestBlockAndTxPool),
        tip: None,       
    };
 
	let result = sdk
		.tx
		.data_availability
		.submit_data(data, wait_for, &account, Some(options))
		.await?;
 
	println!(
		"Who={}, DataHash={:?}",
		result.event.who, result.event.data_hash
	);
	println!("TxData={:?}", result.tx_data.data);
	println!(
		"TxHash={:?}, BlockHash={:?}",
		result.tx_hash, result.block_hash
	);
 
	Ok(())
}