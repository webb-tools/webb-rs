//! Anchor Update Proposal.
use crate::{ResourceId, TypedChainId};

/// Anchor Update Proposal.
///
/// The [`AnchorUpdateProposal`] updates the target Anchor's knowledge of the
/// source Anchor's Merkle roots. This knowledge is necessary to prove
/// membership in the source Anchor's Merkle tree on the target chain.
///
/// The format of the proposal is:
/// ```text
/// ┌────────────────────┬────────────────────────────────────────────────────────────────────┐
/// │                    │                                                                    │
/// │   ResourceId 32B   │ `AnchorHandler::Call::execute_anchor_update_proposal` encoded call │
/// │                    │                                                                    │
/// └────────────────────┴────────────────────────────────────────────────────────────────────┘
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct AnchorUpdateProposal {
    #[builder(default = 42)]
    pallet_index: u8,
    #[builder(default = 1)]
    call_index: u8,
    resource_id: ResourceId,
    src_chain: TypedChainId,
    merkle_root: [u8; 32],
    latest_leaf_index: u32,
}

impl AnchorUpdateProposal {
    /// Get the resource id.
    #[must_use]
    pub const fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    /// Get the source chain.
    #[must_use]
    pub const fn src_chain(&self) -> TypedChainId {
        self.src_chain
    }

    /// Get the latest leaf index.
    #[must_use]
    pub const fn latest_leaf_index(&self) -> u32 {
        self.latest_leaf_index
    }

    /// Get the merkle root.
    #[must_use]
    pub const fn merkle_root(&self) -> &[u8; 32] {
        &self.merkle_root
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(80);
        let call = ExecuteAnchorUpdateProposal {
            r_id: self.resource_id.to_bytes(),
            anchor_metadata: EdgeMetadata {
                src_chain_id: self.src_chain.chain_id(),
                root: Element(self.merkle_root),
                latest_leaf_index: self.latest_leaf_index,
            },
        };
        // add pallet index
        out.push(self.pallet_index);
        // add call index
        out.push(self.call_index);
        scale_codec::Encode::encode_to(&call, &mut out);
        out
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl From<AnchorUpdateProposal> for Vec<u8> {
    fn from(proposal: AnchorUpdateProposal) -> Self {
        proposal.into_bytes()
    }
}

impl TryFrom<Vec<u8>> for AnchorUpdateProposal {
    type Error = scale_codec::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let pallet_index = value.get(0).copied().ok_or_else(|| {
            scale_codec::Error::from("invalid proposal: missing pallet index")
        })?;

        let call_index = value.get(1).copied().ok_or_else(|| {
            scale_codec::Error::from("invalid proposal: missing call index")
        })?;

        let call: ExecuteAnchorUpdateProposal =
            scale_codec::Decode::decode(&mut &value[2..])?;

        let resource_id = ResourceId::from(call.r_id);
        let src_chain = TypedChainId::from(call.anchor_metadata.src_chain_id);
        let merkle_root = call.anchor_metadata.root.0;
        let latest_leaf_index = call.anchor_metadata.latest_leaf_index;
        let proposal = AnchorUpdateProposal {
            pallet_index,
            call_index,
            resource_id,
            src_chain,
            merkle_root,
            latest_leaf_index,
        };
        Ok(proposal)
    }
}

// if we have EVM available, we can convert the EVM proposal to a substrate proposal
#[cfg(feature = "evm")]
impl From<crate::evm::AnchorUpdateProposal> for AnchorUpdateProposal {
    fn from(proposal: crate::evm::AnchorUpdateProposal) -> Self {
        AnchorUpdateProposal::builder()
            .resource_id(proposal.header().resource_id())
            .src_chain(proposal.src_chain())
            .merkle_root(proposal.merkle_root().clone())
            .latest_leaf_index(proposal.latest_leaf_index())
            .build()
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct Element(pub [u8; 32]);

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct EdgeMetadata {
    src_chain_id: u64,
    root: Element,
    latest_leaf_index: u32,
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteAnchorUpdateProposal {
    r_id: [u8; 32],
    anchor_metadata: EdgeMetadata,
}

#[cfg(test)]
mod tests {
    use crate::{ResourceId, TargetSystem};

    use super::*;

    #[test]
    fn encode() {
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let src_chain = TypedChainId::Evm(1);
        let latest_leaf_index = 0x0001;
        let merkle_root = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let proposal = AnchorUpdateProposal::builder()
            .resource_id(resource_id)
            .src_chain(src_chain)
            .merkle_root(merkle_root.clone())
            .latest_leaf_index(latest_leaf_index)
            .build();
        let bytes = proposal.to_bytes();
        let expected = concat!(
          "2a01", // pallet index, call index
          "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004", // resource id
          "0100000000010000000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f01000000", // resource id
        );
        let bytes_hex = hex::encode(bytes);
        assert_eq!(bytes_hex, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
          "2a01" // pallet index, call index
          "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004" // resource id
          "0100000000010000000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f01000000" // metadata
        );

        let proposal = AnchorUpdateProposal::try_from(bytes.to_vec()).unwrap();
        assert_eq!(proposal.pallet_index, 0x2a);
        assert_eq!(proposal.call_index, 0x01);
        assert_eq!(
            proposal.resource_id.target_system(),
            TargetSystem::new_contract_address(hex_literal::hex!(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            ))
        );
        assert_eq!(proposal.resource_id.typed_chain_id(), TypedChainId::Evm(4));
        assert_eq!(proposal.src_chain, TypedChainId::Evm(1));
        assert_eq!(
            proposal.merkle_root,
            [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
                0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
                0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
                0x1e, 0x1f,
            ]
        );
        assert_eq!(proposal.latest_leaf_index, 0x0001);
    }
}
