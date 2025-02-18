use avail_rust::{
	avail::{self, runtime_types::bounded_collections::bounded_vec::BoundedVec},
	SDK,
};
 
#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await.unwrap();
 
	let key = String::from("gohan");
	let key = BoundedVec(key.as_bytes().to_vec());
	let storage_query = avail::storage().data_availability().app_keys(key);
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