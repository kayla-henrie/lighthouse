use crate::*;
use serde::{Deserialize, Serialize};

/// Validator registration, for use in interacting with servers implementing the builder API.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct SignedValidatorRegistrationData {
    pub message: ValidatorRegistrationData,
    pub signature: Signature,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct ValidatorRegistrationData {
    pub fee_recipient: Address,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub gas_limit: u64,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub timestamp: u64,
    pub pubkey: PublicKeyBytes,
    pub signature: Signature,
}
