#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { name: "pe pnft",symbol:"NFT",minter:"nibi10vcakmc8g3nvd60c2ejzasnxjqxn6ru65j7cwh" };
        let info = mock_info("creator", &coins(1000, "nibi"));

        // 尝试实例化合约
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len()); // 确保没有发送消息
        /**
         * pub token_id: String,
    /// The owner of the newly minter NFT
    pub owner: String,
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
         */
        let mint = ExecuteMsg::Mint{
            token_id: 1,
            token_uri: "www.baidu.com".to_string(),
            owner: "nibi10vcakmc8g3nvd60c2ejzasnxjqxn6ru65j7cwh".to_string(),
        };
        let execute_res = execute(deps.as_mut(), mock_env(), info, mint).unwrap();
        // execute
        // 查询合约状态
        let res = query(deps.as_ref(), mock_env(), QueryMsg::NftInfo {tokenId:1}).unwrap();
        let value: NftInfoResponse = from_binary(&res).unwrap();
        assert_eq!(Some("www.baidu.com2"), value.token_uri);
    }
}