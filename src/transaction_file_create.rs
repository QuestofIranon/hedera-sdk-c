use hedera::transaction::TransactionFileCreate;
use hedera::FileId;

def_tx_new!(TransactionFileCreate: hedera_transaction__file_create__new);

// todo: expires at

def_tx_method!(TransactionFileCreate: hedera_transaction__file_create__set_key(PublicKey): key);

def_tx_method!(TransactionFileCreate: hedera_transaction__file_create__set_contents(Vec<u8>): contents);
