use std::error::Error;

#[cfg(feature = "generate-contracts")]
mod evm {
    use super::*;
    use ethers::contract::Abigen;

    fn parse_and_write_abigen(
        path: &str,
        out: &str,
        contract_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("cargo:rerun-if-changed=./{}", path);
        println!("cargo:rerun-if-changed=./{}", out);

        let generated_tokens = Abigen::new(contract_name, path)?
            .add_derive("serde::Serialize")?
            .add_derive("serde::Deserialize")?
            .format(false) // don't use rustfmt for now.
            .generate()?
            .to_string();
        let syntax_tree = syn::parse_file(&generated_tokens).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        std::fs::write(out, formatted)?;
        Ok(())
    }

    pub fn build_protocol_solidity_vanchor_verifier(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/VAnchorVerifier.json",
            "src/evm/contract/protocol_solidity/vanchor_verifier.rs",
            "VAnchorVerifierContract",
        )
    }

    pub fn build_protocol_solidity_verifier_2_2() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/Verifier2_2.json",
            "src/evm/contract/protocol_solidity/verifier_2_2.rs",
            "Verifier2x2Contract",
        )
    }

    pub fn build_protocol_solidity_verifier_2_16() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/Verifier2_16.json",
            "src/evm/contract/protocol_solidity/verifier_2_16.rs",
            "Verifier2x16Contract",
        )
    }

    pub fn build_protocol_solidity_verifier_8_2() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/Verifier8_2.json",
            "src/evm/contract/protocol_solidity/verifier_8_2.rs",
            "Verifier8x2Contract",
        )
    }

    pub fn build_protocol_solidity_verifier_8_16() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/Verifier8_16.json",
            "src/evm/contract/protocol_solidity/verifier_8_16.rs",
            "Verifier8x16Contract",
        )
    }
    pub fn build_protocol_tangle_signing_rules() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/tangle/VotableSigningRules.json",
            "src/evm/contract/tangle/votable_signing_rules.rs",
            "SigningRulesContract",
        )
    }

    pub fn build_protocol_solidity_vanchor_base() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/VAnchorBase.json",
            "src/evm/contract/protocol_solidity/vanchor_base.rs",
            "VAnchorBaseContract",
        )
    }

    pub fn build_protocol_solidity_vanchor() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/VAnchor.json",
            "src/evm/contract/protocol_solidity/variable_anchor.rs",
            "VAnchorContract",
        )
    }

    pub fn build_protocol_solidity_vanchor_tree() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/VAnchorTree.json",
            "src/evm/contract/protocol_solidity/variable_anchor_tree.rs",
            "VAnchorTreeContract",
        )
    }

    pub fn build_protocol_solidity_vanchor_encode_inputs(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/VAnchorEncodeInputs.json",
            "src/evm/contract/protocol_solidity/vanchor_encode_inputs.rs",
            "VAnchorEncodeInputsContract",
        )
    }

    pub fn build_protocol_solidity_anchor_handler() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/AnchorHandler.json",
            "src/evm/contract/protocol_solidity/anchor_handler.rs",
            "AnchorHandlerContract",
        )
    }

    pub fn build_protocol_solidity_signature_bridge(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/SignatureBridge.json",
            "src/evm/contract/protocol_solidity/signature_bridge.rs",
            "SignatureBridgeContract",
        )
    }

    pub fn build_protocol_solidity_fungible_token_wrapper(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/FungibleTokenWrapper.json",
            "src/evm/contract/protocol_solidity/fungible_token_wrapper.rs",
            "FungibleTokenWrapperContract",
        )
    }

    pub fn build_protocol_solidity_token_wrapper() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/TokenWrapper.json",
            "src/evm/contract/protocol_solidity/token_wrapper.rs",
            "TokenWrapperContract",
        )
    }
    pub fn build_protocol_solidity_treasury() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/Treasury.json",
            "src/evm/contract/protocol_solidity/treasury.rs",
            "TreasuryContract",
        )
    }

    pub fn build_protocol_solidity_treasury_handler(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/TreasuryHandler.json",
            "src/evm/contract/protocol_solidity/treasury_handler.rs",
            "TreasuryHandlerContract",
        )
    }

    pub fn build_protocol_solidity_token_wrapper_handler(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/TokenWrapperHandler.json",
            "src/evm/contract/protocol_solidity/token_wrapper_handler.rs",
            "TokenWrapperHandlerContract",
        )
    }

    pub fn build_protocol_solidity_erc20_preset_minter_pauser(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/ERC20PresetMinterPauser.json",
            "src/evm/contract/protocol_solidity/erc20_preset_minter_pauser.rs",
            "ERC20PresetMinterPauserContract",
        )
    }

    pub fn build_protocol_solidity_poseidon_t3() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/PoseidonT3.json",
            "src/evm/contract/protocol_solidity/poseidon_t3.rs",
            "PoseidonT3Contract",
        )
    }

    pub fn build_protocol_solidity_poseidon_t4() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/PoseidonT4.json",
            "src/evm/contract/protocol_solidity/poseidon_t4.rs",
            "PoseidonT4Contract",
        )
    }

    pub fn build_protocol_solidity_poseidon_t6() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/PoseidonT6.json",
            "src/evm/contract/protocol_solidity/poseidon_t6.rs",
            "PoseidonT6Contract",
        )
    }

    pub fn build_protocol_solidity_poseidon_hasher(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/PoseidonHasher.json",
            "src/evm/contract/protocol_solidity/poseidon_hasher.rs",
            "PoseidonHasherContract",
        )
    }

    pub fn build_protocol_solidity_masp_vanchor_batch_tree(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/MultiAssetVAnchorBatchTree.json",
            "src/evm/contract/protocol_solidity/masp_vanchor_batch_tree.rs",
            "MultiAssetVAnchorBatchTreeContract",
        )
    }

    pub fn build_protocol_solidity_masp_vanchor_tree(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/MultiAssetVAnchorTree.json",
            "src/evm/contract/protocol_solidity/masp_vanchor_tree.rs",
            "MultiAssetVAnchorTreeContract",
        )
    }

    pub fn build_protocol_solidity_masp_vanchor() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/MultiAssetVAnchor.json",
            "src/evm/contract/protocol_solidity/masp_vanchor.rs",
            "MultiAssetVAnchorContract",
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "generate-contracts")]
    {
        evm::build_protocol_tangle_signing_rules()?;
        evm::build_protocol_solidity_vanchor_base()?;
        evm::build_protocol_solidity_vanchor()?;
        evm::build_protocol_solidity_vanchor_verifier()?;
        evm::build_protocol_solidity_verifier_2_2()?;
        evm::build_protocol_solidity_verifier_2_16()?;
        evm::build_protocol_solidity_verifier_8_2()?;
        evm::build_protocol_solidity_verifier_8_16()?;
        evm::build_protocol_solidity_vanchor_tree()?;
        evm::build_protocol_solidity_vanchor_encode_inputs()?;
        evm::build_protocol_solidity_anchor_handler()?;
        evm::build_protocol_solidity_signature_bridge()?;
        evm::build_protocol_solidity_token_wrapper()?;
        evm::build_protocol_solidity_fungible_token_wrapper()?;
        evm::build_protocol_solidity_token_wrapper_handler()?;
        evm::build_protocol_solidity_treasury()?;
        evm::build_protocol_solidity_treasury_handler()?;
        evm::build_protocol_solidity_erc20_preset_minter_pauser()?;
        evm::build_protocol_solidity_poseidon_t3()?;
        evm::build_protocol_solidity_poseidon_t4()?;
        evm::build_protocol_solidity_poseidon_t6()?;
        evm::build_protocol_solidity_poseidon_hasher()?;
        evm::build_protocol_solidity_masp_vanchor_batch_tree()?;
        evm::build_protocol_solidity_masp_vanchor_tree()?;
        evm::build_protocol_solidity_masp_vanchor()?;
    }
    Ok(())
}
