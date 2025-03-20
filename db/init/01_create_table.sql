\connect ledger

CREATE TABLE public.accounts (
    account_id SERIAL PRIMARY KEY,
    account_name VARCHAR(255) NOT NULL,
    account_type VARCHAR(50) NOT NULL,  -- E.g., 'Asset', 'Liability', 'Equity', 'Income', 'Expense', 'UtilDebit', 'UtilCredit'
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE public.accounts OWNER TO postgres;


CREATE TABLE public.transactions (
    transaction_id SERIAL PRIMARY KEY,
    transaction_type VARCHAR(50) NOT NULL,  -- E.g., 'FromPrev', 'InTerm', 'Kessan', 'Soneki', 'ToNext'
    description VARCHAR(255),
    transaction_date DATE NOT NULL,
    -- total_amount DECIMAL(18, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_transactions_date ON transactions(transaction_date);
CREATE INDEX idx_transactions_type ON transactions(transaction_type);

ALTER TABLE public.transactions OWNER TO postgres;


CREATE TABLE public.transaction_details (
    transaction_detail_id SERIAL PRIMARY KEY,
    transaction_id INT REFERENCES transactions(transaction_id) ON DELETE CASCADE,
    account_id INT REFERENCES accounts(account_id) ON DELETE RESTRICT,
    debit_amount DECIMAL(18, 2) DEFAULT 0,
    credit_amount DECIMAL(18, 2) DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CHECK (debit_amount >= 0 AND credit_amount >= 0),
    CHECK (debit_amount = 0 OR credit_amount = 0)  -- Either debit or credit, not both
);

CREATE INDEX idx_transaction_details_account_id ON transaction_details(account_id);

ALTER TABLE public.transaction_details OWNER TO postgres;

