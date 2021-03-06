use crate::transaction_id::CTransactionId;
use hedera::{query::QueryTransactionGetReceipt, TransactionReceipt};

def_query!(
    QueryTransactionGetReceipt: transaction_get_receipt(CTransactionId) -> TransactionReceipt
);
