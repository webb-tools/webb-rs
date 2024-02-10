#[cfg(test)]
mod tests {
    use ark_ff::{BigInteger, PrimeField};
    use webb::evm::contract::protocol_solidity::fungible_token_wrapper::FungibleTokenWrapperContract;
    use webb::evm::contract::protocol_solidity::variable_anchor_tree::{
        CommonExtData, Encryptions, PublicInputs,
    };
    use webb::evm::ethers::core::rand::thread_rng;
    use webb::evm::ethers::utils::{keccak256, parse_ether};

    use crate::types::{ExtData, IntoAbiToken};
    use crate::utils::{
        deconstruct_public_inputs_el, setup_utxos, setup_vanchor_circuit,
        vanchor_2_2_fixtures,
    };
    use crate::{
        v_bridge::{TokenConfig, VAnchorBridgeDeploymentConfig},
        LocalEvmChain,
    };
    use circom_proving::types::Proof as SolidityProof;
    use webb::evm::ethers::types::{H256, U256};
    use webb::evm::{
        contract::protocol_solidity::variable_anchor_tree::VAnchorTreeContract,
        ethers::signers::{LocalWallet, Signer},
    };

    #[tokio::test]
    async fn test_vanchor_deposit() {
        // Get fixtures
        let (params_2_2, wc_2_2) = vanchor_2_2_fixtures();
        let token_config = TokenConfig::default();

        // Deploy Hermes chain.
        let hermes_chain =
            LocalEvmChain::new(5001, String::from("Hermes"), None);

        let secret_key = hermes_chain.keys()[0].clone();
        let deployer_wallet1 =
            LocalWallet::from(secret_key).with_chain_id(5001u32);

        let hermes_bridge_config = VAnchorBridgeDeploymentConfig::builder()
            .deployer(deployer_wallet1.clone())
            .token_config(token_config.clone())
            .max_edges(1)
            .build();
        let hermes_bridge = hermes_bridge_config
            .deploy_variable_anchor_bridge(&hermes_chain)
            .await
            .unwrap();

        println!("Hermes bridge deployed: {:?}", hermes_bridge);

        // Vanchor instance on hermes chain
        let vanchor = VAnchorTreeContract::new(
            hermes_bridge.vanchor,
            hermes_chain.client(),
        );

        let fungible_token_wrapper = FungibleTokenWrapperContract::new(
            hermes_bridge.fungible_token_wrapper,
            hermes_chain.client(),
        );

        // Approve token spending on vanchor.
        fungible_token_wrapper
            .approve(vanchor.address(), parse_ether(1000).unwrap())
            .send()
            .await
            .unwrap();

        // Mint tokens on wallet.
        fungible_token_wrapper
            .mint(deployer_wallet1.address(), parse_ether(1000).unwrap())
            .send()
            .await
            .unwrap();

        let recipient_wallet =
            LocalWallet::new(&mut thread_rng()).with_chain_id(5002u32);
        let relayer_wallet =
            LocalWallet::new(&mut thread_rng()).with_chain_id(5001u32);

        let recipient = recipient_wallet.address();
        let relayer = relayer_wallet.address();
        let typed_source_chain_id = hermes_chain.typed_chain_id();
        let types_target_chain_id = hermes_chain.typed_chain_id();
        let ext_amount = 10_i128;
        let public_amount = 10_i128;
        let fee = 0_i128;
        let refund = 0_i128.into();
        let token = hermes_bridge.fungible_token_wrapper;

        let input_chain_ids = [typed_source_chain_id, types_target_chain_id];
        let input_amounts = [0, 0];
        let input_indices = [0, 0];
        let output_chain_ids = [typed_source_chain_id, types_target_chain_id];
        let output_amount = [10, 0];
        let output_indices = [0, 0];

        let input_utxos =
            setup_utxos(input_chain_ids, input_amounts, Some(input_indices));
        let output_utxos =
            setup_utxos(output_chain_ids, output_amount, Some(output_indices));

        let encrypted_output1 =
            output_utxos[0].commitment.into_repr().to_bytes_be();
        let encrypted_output2 =
            output_utxos[1].commitment.into_repr().to_bytes_be();

        let leaf0 = input_utxos[0].commitment.into_repr().to_bytes_be();
        let leaf1 = input_utxos[1].commitment.into_repr().to_bytes_be();

        let leaves: Vec<Vec<u8>> = vec![leaf0, leaf1];

        let ext_data = ExtData::builder()
            .recipient(recipient)
            .relayer(relayer)
            .ext_amount(ext_amount)
            .fee(fee.into())
            .refund(refund)
            .token(token)
            .encrypted_output1(encrypted_output1.clone())
            .encrypted_output2(encrypted_output2.clone())
            .build();

        let ext_data_hash: H256 = keccak256(ext_data.encode_abi_token()).into();
        let root = vanchor.get_last_root().call().await.unwrap();
        let neighbor_roots =
            vanchor.get_latest_neighbor_roots().call().await.unwrap();

        let ext_data_hash_be = U256::from_big_endian(&ext_data_hash.0);

        let (proof, public_inputs) = setup_vanchor_circuit(
            public_amount,
            typed_source_chain_id,
            ext_data_hash_be,
            input_utxos,
            output_utxos.clone(),
            root,
            [neighbor_roots[0]],
            leaves,
            &params_2_2,
            wc_2_2,
        );

        let solidity_proof = SolidityProof::try_from(proof).unwrap();
        let proof_bytes = solidity_proof.encode().unwrap();

        let common_ext_data = CommonExtData {
            recipient,
            ext_amount: ext_data.ext_amount.into(),
            relayer,
            fee: ext_data.fee.into(),
            refund,
            token,
        };

        // Deconstructing public inputs
        let (
            _chain_id,
            public_amount,
            root_set,
            nullifiers,
            commitments,
            ext_data_hash,
        ) = deconstruct_public_inputs_el(&public_inputs);

        let flattened_root: Vec<u8> = root_set
            .iter()
            .flat_map(|x| {
                let mut be_bytes = [0u8; 32];
                x.to_big_endian(&mut be_bytes);
                be_bytes
            })
            .collect();
        let public_inputs = PublicInputs {
            roots: flattened_root.into(),
            extension_roots: b"0x".to_vec().into(),
            input_nullifiers: nullifiers,
            output_commitments: commitments,
            public_amount,
            ext_data_hash,
        };

        let encryptions = Encryptions {
            encrypted_output_1: encrypted_output1.into(),
            encrypted_output_2: encrypted_output2.into(),
        };

        vanchor
            .transact(
                proof_bytes.into(),
                [0u8; 32].into(),
                common_ext_data,
                public_inputs.clone(),
                encryptions,
            )
            .send()
            .await
            .unwrap();

        // Shutdown chains.
        hermes_chain.shutdown();
    }
}