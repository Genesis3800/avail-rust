use avail_rust::{avail, SDK};
 
#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await.unwrap();
 
	let storage_query = avail::storage().data_availability().next_app_id();
	let best_block_hash = sdk
		.rpc
		.chain
		.get_block_hash(None)
		.await
		.map_err(|e| e.to_string())?;
	let result = sdk
		.api
		.storage()
		.at(best_block_hash)
		.fetch(&storage_query)
		.await
		.map_err(|e| e.to_string())?;
 
	dbg!(result);
 
	Ok(())
}