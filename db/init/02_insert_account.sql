\connect ledger

INSERT INTO public.accounts (account_name, account_type)
VALUES
('(前期繰越(借方勘定用))', 'UtilDebit'),
('(前期繰越(貸方勘定用))', 'UtilCredit'),
('(次期繰越(借方勘定用))', 'UtilDebit'),
('(次期繰越(貸方勘定用))', 'UtilCredit'),
('普通預金', 'Asset'),
('売掛金', 'Asset'),
('未収金', 'Asset'),
('前払金', 'Asset'),
('事業主貸', 'Asset'),
('買掛金', 'Liability'),
('未払金', 'Liability'),
('前受金', 'Liability'),
('事業主借', 'Liability'),
('資本金', 'Equity'),
('売上', 'Income'),
('受取利息', 'Income'),
('仮受消費税', 'Income'),
('仮払消費税', 'Expense'),
('会議費', 'Expense'),
('旅費交通費', 'Expense'),
('資料費', 'Expense'),
('消耗品費', 'Expense'),
('通信費', 'Expense'),
('水道光熱費', 'Expense'),
('地代家賃', 'Expense'),
('支払手数料', 'Expense'),
('損益', 'Income')
;

