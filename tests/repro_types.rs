use ib_flex::parse_activity_flex;
use ib_flex::types::{CashTransactionType, CorporateActionType};

#[test]
fn test_cash_transaction_types() {
    let xml = r#"
    <FlexQueryResponse queryName="Activity" type="AF">
        <FlexStatements count="1">
            <FlexStatement accountId="U1234567" fromDate="2023-01-01" toDate="2023-01-31" whenGenerated="2023-02-01;120000">
                <CashTransactions>
                    <CashTransaction accountId="U1234567" currency="USD" fxRateToBase="1" assetCategory="CASH" symbol="USD" description="Dividend" date="2023-01-15" amount="100.0" type="Dividends" transactionID="1" />
                    <CashTransaction accountId="U1234567" currency="USD" fxRateToBase="1" assetCategory="CASH" symbol="USD" description="Deposit" date="2023-01-01" amount="1000.0" type="Deposits &amp; Withdrawals" transactionID="2" />
                    <CashTransaction accountId="U1234567" currency="USD" fxRateToBase="1" assetCategory="CASH" symbol="USD" description="Interest" date="2023-01-31" amount="5.0" type="Broker Interest Paid" transactionID="3" />
                </CashTransactions>
            </FlexStatement>
        </FlexStatements>
    </FlexQueryResponse>
    "#;

    let statement = parse_activity_flex(xml).unwrap();
    let txns = statement.cash_transactions.items;

    assert_eq!(txns.len(), 3);

    // Updated assertions to use Enum types
    assert_eq!(
        txns[0].transaction_type,
        Some(CashTransactionType::Dividends)
    );
    assert_eq!(
        txns[1].transaction_type,
        Some(CashTransactionType::DepositsWithdrawals)
    );
    assert_eq!(
        txns[2].transaction_type,
        Some(CashTransactionType::BrokerInterestPaid)
    );
}

#[test]
fn test_corporate_action_types() {
    let xml = r#"
    <FlexQueryResponse queryName="Activity" type="AF">
        <FlexStatements count="1">
            <FlexStatement accountId="U1234567" fromDate="2023-01-01" toDate="2023-01-31" whenGenerated="2023-02-01;120000">
                <CorporateActions>
                    <CorporateAction accountId="U1234567" conid="123" symbol="AAPL" assetCategory="STK" type="Stock Split" dateTime="2023-01-15" amount="0" currency="USD" reportDate="2023-01-15" />
                    <CorporateAction accountId="U1234567" conid="456" symbol="MSFT" assetCategory="STK" type="Cash Dividend" dateTime="2023-01-20" amount="15.0" currency="USD" reportDate="2023-01-20" />
                </CorporateActions>
            </FlexStatement>
        </FlexStatements>
    </FlexQueryResponse>
    "#;

    let statement = parse_activity_flex(xml).unwrap();
    let actions = statement.corporate_actions.items;

    assert_eq!(actions.len(), 2);

    // Updated assertions to use Enum types
    assert_eq!(
        actions[0].action_type,
        Some(CorporateActionType::StockSplit)
    );
    assert_eq!(
        actions[1].action_type,
        Some(CorporateActionType::CashDividend)
    );
}
