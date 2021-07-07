use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

use ethereum_types::H256;
use rand::Rng;

pub use ethereum_types;
pub use ethers;

pub type H248 = [u8; 31];
pub type Preimage = [u8; 62];

#[derive(Debug, Clone, Copy)]
pub struct Deposit {
    pub preimage: Preimage,
    pub commitment: H256,
    pub nullifier_hash: H256,
    pub secret: H248,
    pub nullifier: H248,
}

impl Deposit {
    fn from_preimage(preimage: Preimage) -> Deposit {
        let nullifier: H248 = preimage[0..31]
            .try_into()
            .expect("31 bytes is already there.");
        let secret: H248 = preimage[31..62]
            .try_into()
            .expect("31 bytes is already there.");
        let commitment =
            H256::from(pedersen::hash(&preimage).expect("32 bytes"));
        let nullifier_hash =
            H256::from(pedersen::hash(&nullifier).expect("32 bytes"));
        Deposit {
            preimage,
            commitment,
            nullifier_hash,
            secret,
            nullifier,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Note {
    pub currency: String,
    pub amount: u64,
    pub chain_id: u64,
    pub preimage: Preimage,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum NoteError {
    #[error("Invalid Note Length")]
    InvalidNoteLength,
    #[error(transparent)]
    FromHex(#[from] hex::FromHexError),
    #[error("Invalid Note Prefix expected `anchor`")]
    InvalidNotePrefix,
    #[error("Invalid Note Preimage expected 124hex/62bytes.")]
    InvalidNotePreimage,
    #[error(transparent)]
    ParseIntValue(#[from] std::num::ParseIntError),
}

impl FromStr for Note {
    type Err = NoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        // check note length.
        parts
            .len()
            .eq(&5)
            .then(Default::default)
            .ok_or(NoteError::InvalidNoteLength)?;
        // check note prefix.
        parts[0]
            .eq("anchor")
            .then(Default::default)
            .ok_or(NoteError::InvalidNotePrefix)?;
        let currency = parts[1].to_owned();
        let amount = parts[2].parse::<u64>()?;
        let chain_id = parts[3].parse::<u64>()?;
        let mut preimage: Preimage = [0; 62];
        hex::decode_to_slice(&parts[4][2..], &mut preimage)
            .map_err(|_| NoteError::InvalidNotePreimage)?;
        Ok(Self {
            currency,
            amount,
            chain_id,
            preimage,
        })
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "anchor-{}-{}-{}-0x{}",
            self.currency,
            self.amount,
            self.chain_id,
            hex::encode(&self.preimage)
        )
    }
}

impl From<Note> for Deposit {
    fn from(note: Note) -> Self {
        Self::from_preimage(note.preimage)
    }
}

impl Note {
    pub fn builder() -> NoteBuilder {
        Default::default()
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct NoteBuilder {
    currency: String,
    amount: u64,
    chain_id: u64,
}

impl NoteBuilder {
    pub fn with_currency(self, currency: impl Into<String>) -> Self {
        let mut this = self;
        this.currency = currency.into();
        this
    }

    pub fn with_amount(self, amount: impl Into<u64>) -> Self {
        let mut this = self;
        this.amount = amount.into();
        this
    }

    pub fn with_chain_id(self, chain_id: impl Into<u64>) -> Self {
        let mut this = self;
        this.chain_id = chain_id.into();
        this
    }

    pub fn build<R: Rng>(self, rng: &mut R) -> Note {
        let mut preimage: [u8; 62] = [0; 62];
        rng.fill(&mut preimage[..]);
        Note {
            preimage,
            chain_id: self.chain_id,
            amount: self.amount,
            currency: self.currency,
        }
    }
}

#[cfg(all(test, feature = "integration-tests"))]
mod tests {
    use ethers::prelude::*;
    use std::convert::TryFrom;
    use std::str::FromStr;
    use std::sync::Arc;
    use std::time::Duration;

    use super::*;
    use crate::contracts::anchor::AnchorContract;

    type DynError = Box<dyn std::error::Error + Sync + Send + 'static>;

    struct BeresheetNetwork;
    impl BeresheetNetwork {
        fn endpoint() -> &'static str {
            "http://beresheet3.edgewa.re:9933"
        }

        fn contract_10_tedg_address() -> Address {
            Address::from_str("0x5f771fc87F87DB48C9fB11aA228D833226580689")
                .unwrap()
        }

        fn chain_id() -> u64 {
            2022
        }
    }

    #[tokio::test]
    async fn contract_deposit() -> Result<(), DynError> {
        let provider =
            Provider::<Http>::try_from(BeresheetNetwork::endpoint())?
                .interval(Duration::from_millis(10u64));
        // TODO(shekohex): handle wallets.
        let client = Arc::new(provider);
        let contract = AnchorContract::new(
            BeresheetNetwork::contract_10_tedg_address(),
            client.clone(),
        );
        let mut rng = rand::rngs::OsRng::default();
        let note = Note::builder()
            .with_chain_id(BeresheetNetwork::chain_id())
            .with_amount(10u64)
            .with_currency("tEDG")
            .build(&mut rng);
        println!("Note: {}", note);
        let deposit: Deposit = note.clone().into();
        let tx = contract
            .deposit(deposit.commitment.into())
            .value(note.amount);
        let result = tx.send().await?;
        let receipt = dbg!(result).await?;
        dbg!(receipt);
        Ok(())
    }
}
