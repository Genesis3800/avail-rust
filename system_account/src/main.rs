use avail_rust::{avail, AccountId, SDK};
use core::str::FromStr;
 
#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await.unwrap();
 
	let account_id =
		AccountId::from_str("5DDY2yzh8uCysYFAiRSTeQVwtZSKNF49CkQkyPH852xvrYKk").unwrap(); // Alice_Stash
 
	let storage_query = avail::storage().system().account(account_id);
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