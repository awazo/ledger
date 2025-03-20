use chrono::NaiveDate;
use serde::{
    Deserialize,
    Serialize,
};

use ledger_db::{
    Account,
    AmountSide,
    Db,
    Transaction,
    TransactionDetail,
};

use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAmount {
    pub account: String,
    pub amount: f32,
}

impl AccountAmount {

    pub async fn into_transaction_detail(
        &self,
        db: &Db,
        side: AmountSide,
    ) -> Result<TransactionDetail, Error> {
        let acc_search_result
            = Account::by_name(db, &self.account).await;
        let account_type = match acc_search_result {
            Ok(acc) => acc.account_type,
            Err(ledger_db::Error::RowNotFound)
                => return Err(Error::AccountNotFound(self.account.clone())),
            Err(err) => return Err(err.into()),
        };
        match side {
            AmountSide::Debit => Ok(TransactionDetail {
                account_name: self.account.clone(),
                account_type,
                debit_amount: self.amount,
                credit_amount: 0_f32,
            }),
            AmountSide::Credit => Ok(TransactionDetail {
                account_name: self.account.clone(),
                account_type,
                debit_amount: 0_f32,
                credit_amount: self.amount,
            }),
        }
    }

    pub fn from_transaction_detail(
        td: &TransactionDetail,
    ) -> (Self, AmountSide) {
        let (amount, side)
            = if td.credit_amount == 0_f32 {
                (td.debit_amount, AmountSide::Debit)
            } else if td.debit_amount == 0_f32 {
                (td.credit_amount, AmountSide::Credit)
            } else if td.debit_amount >= td.credit_amount {
                (td.debit_amount - td.credit_amount, AmountSide::Debit)
            } else {
                (td.credit_amount - td.debit_amount, AmountSide::Credit)
            };
        (AccountAmount { account: td.account_name.clone(), amount }, side)
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Journal {
    pub transaction_type: String,
    pub date: NaiveDate,
    pub debit: Vec<AccountAmount>,
    pub credit: Vec<AccountAmount>,
    pub desc: String,
}

impl Journal {

    pub async fn into_transaction(
        &self,
        db: &Db,
    ) -> Result<Transaction, Error> {
        let transaction_type = (&self.transaction_type).into();
        let mut details = Vec::new();
        for debit in &self.debit {
            details.push(
                debit.into_transaction_detail(db, AmountSide::Debit)
                .await?
            );
        }
        for credit in &self.credit {
            details.push(
                credit.into_transaction_detail(db, AmountSide::Credit)
                .await?
            );
        }
        Ok(Transaction {
            transaction_id: 0,
            transaction_date: self.date,
            transaction_type,
            description: self.desc.clone(),
            details,
        })
    }

    pub fn from_transaction(
        tran: &Transaction,
    ) -> Self {
        let mut debit = Vec::new();
        let mut credit = Vec::new();
        for d in &tran.details {
            let (aa, side) = AccountAmount::from_transaction_detail(d);
            match side {
                AmountSide::Debit => debit.push(aa),
                AmountSide::Credit => credit.push(aa),
            }
        }
        Journal {
            transaction_type: tran.transaction_type.into_japanese(),
            date: tran.transaction_date,
            debit,
            credit,
            desc: tran.description.clone(),
        }
    }

}

