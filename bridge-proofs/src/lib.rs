mod evm;


#[cfg(test)]
mod tests {
    use ethers::prelude::*;
    use ethers::types::ValueOrArray::{Array, Value};
    use ethers_core::k256::elliptic_curve::rand_core::block;
    use crate::evm::verify_trie_proof::*;
    use ethereum_types::H256 as EthTypeH256;

    #[tokio::test]
    async fn get_proof_works() {
        let provider =
            Provider::<Http>::try_from("https://eth-mainnet.g.alchemy.com/v2/gWJbkK4UtKn4wd21HFa59KRIdNk_xs3q").unwrap();

        let address_vec =
            hex::decode("6fC21092DA55B392b045eD78F4732bff3C580e2c".to_lowercase()).unwrap();
        let mut address_bytes = [0u8; 20];
        address_bytes.copy_from_slice(&address_vec);
        println!("Got address: {:?}, {:?}", address_bytes, address_vec.len());

        let slot = hex::decode(
            "0000000000000000000000000000000000000000000000000000000000000000",
        );
        let mut slot_bytes = [0u8; 32];
        slot_bytes.copy_from_slice(&slot.unwrap());
        
        let proof = provider
            .get_proof(
                NameOrAddress::Address(address_bytes.into()),
                vec![H256(slot_bytes.into())],
                None,
            )
            .await
            .unwrap();
        println!("Got proof: {}", serde_json::to_string(&proof).unwrap());

        let storage_hash = (&proof).storage_hash;

        let first_storage_proof = &(&proof).storage_proof[0].proof;
        let first_storage_key = &(&proof).storage_proof[0].key;

        TrieProver::verify_trie_proof(
            EthTypeH256::from(storage_hash.as_fixed_bytes()),
            H256::to_fixed_bytes(first_storage_key.clone()).to_vec(),
            first_storage_proof.iter().map(|x| x.to_vec()).collect::<Vec<Vec<u8>>>(),
        );
    }
}
