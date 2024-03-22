#[cfg(test)]
mod tests {
    // use super::*;
    use crate::msg::{InstantiateMsg,ExecuteMsg,QueryMsg,MintMsg};
    use cw721:: NftInfoResponse;
    use crate::state::Cw721Contract;
    use cosmwasm_std::testing::{mock_env,mock_dependencies, mock_info};
    use cosmwasm_std::{coins, StdResult, from_json,Response,Empty};
    const USER: &str = "nibi10vcakmc8g3nvd60c2ejzasnxjqxn6ru65j7cwh";
    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let token_id = "1".to_string();

        let msg: InstantiateMsg = InstantiateMsg { name: "pe pnft".to_string(),symbol:"NFT".to_string(),minter:USER.to_string() };
        let info = mock_info(USER, &coins(1000, "unibi"));


        let contract = Cw721Contract::default();
        // 尝试实例化合约
        let res:Response<Empty> = contract.instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len()); // 确保没有发送消息

        // change_base_uri 设置base_uri
        let new_base_uri = "www.baidu.com/".to_string();
        let res_chage = contract.change_base_uri(deps.as_mut(), info.clone(), new_base_uri).unwrap();
        assert_eq!(0, res_chage.messages.len()); // 确保没有发送消息

        // 根据 MintMsg 结构体来创建一个实例
        let mint_msg = MintMsg {
            token_id: token_id, // 或者其他符合 T 类型的值
            owner: USER.to_string(),
            extension: "Some data".to_string(), // 假设 T 就是 String 类型
            // 填充其他字段...
        };
        let mint = ExecuteMsg::<_>::Mint(mint_msg);
        let _execute_res = contract.execute(deps.as_mut(), mock_env(), info.clone(), mint).unwrap();
        // execute
        // 查询合约状态
        let res = contract.query(deps.as_ref(), mock_env(), QueryMsg::NftInfo {token_id:1.to_string()}).unwrap();
        // let value: NftInfoResponse<T> = from_json(&res).unwrap();
        let nft_info: StdResult<NftInfoResponse<String>> = from_json(&res);
        let uri = nft_info.unwrap().token_uri.unwrap(); // 只有当你确定 token_uri 总是有值时使用
        println!("Token URI: {}", uri);
        assert_eq!("www.baidu.com/1", uri);
    }
}