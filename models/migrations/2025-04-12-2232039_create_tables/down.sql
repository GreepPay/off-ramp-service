DROP TABLE IF EXISTS offramp_service.sep31_transactions;

DROP INDEX IF EXISTS offramp_service.idx_sep31_transactions_transaction_id;

DROP INDEX IF EXISTS offramp_service.idx_sep31_transactions_status;

DROP TABLE IF EXISTS offramp_service.sep6_refund_payments;

DROP INDEX IF EXISTS offramp_service.idx_sep6_refund_payments_refund;

DROP TABLE IF EXISTS offramp_service.sep6_refunds;

DROP INDEX IF EXISTS offramp_service.idx_sep6_refunds_transaction;

DROP TABLE IF EXISTS offramp_service.sep6_transactions;

DROP INDEX IF EXISTS offramp_service.idx_sep6_transactions_transaction_id;

DROP INDEX IF EXISTS offramp_service.idx_sep6_transactions_status;

DROP TABLE IF EXISTS offramp_service.sep38_quotes;

DROP INDEX IF EXISTS offramp_service.idx_sep38_quotes_transaction;

DROP TABLE IF EXISTS offramp_service.sep38_assets;

DROP INDEX IF EXISTS offramp_service.idx_sep38_assets_asset;

DROP TABLE IF EXISTS offramp_service.sep12_customer_files;

DROP INDEX IF EXISTS offramp_service.idx_sep12_customer_files_customer;

DROP TABLE IF EXISTS offramp_service.sep12_customers;

DROP INDEX IF EXISTS offramp_service.idx_sep12_customers_account;

DROP INDEX IF EXISTS offramp_service.idx_sep12_customers_status;

DROP SCHEMA IF EXISTS offramp_service;
