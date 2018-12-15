use hedera::transaction::TransactionCryptoAddClaim;
use hedera::{AccountId, PublicKey};

def_tx_new!(
    TransactionCryptoAddClaim: hedera_transaction__crypto_add_claim__new(AccountId, Vec<u8>)
);

def_tx_method!(
    TransactionCryptoAddClaim: hedera_transaction__crypto_add_claim__add_key(PublicKey): key
);
