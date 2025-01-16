use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;
 
#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("wss://turing-rpc.avail.so/ws").await.unwrap();
 
    let alice = "This is a random seed phrase please replace with your own";
	let secret_uri = SecretUri::from_str(alice).unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let amount = 1_600_000_000_000_000_000_000_0u128; // 16_000 Avail tokens
	let root = "5CqgQkrDcdg5QrtuxT3H7WszrqgrBMhdwRbmMVXQzc4VSiEg"; // Alice
	let nominator = "5CqgQkrDcdg5QrtuxT3H7WszrqgrBMhdwRbmMVXQzc4VSiEg"; // Alice
	let bouncer = "5CqgQkrDcdg5QrtuxT3H7WszrqgrBMhdwRbmMVXQzc4VSiEg"; // Alice
 
	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.create(
			amount,
			root,
			nominator,
			bouncer,
			wait_for,
			&account,
			Some(options),
		)
		.await?;
 
	dbg!(result);
 
	Ok(())
}