use std::str::FromStr;

use ethabi::ethereum_types::Address;
use substreams::store;
use substreams::store::StoreNew;
use substreams::store::StoreSetIfNotExists;
use substreams::store::StoreSetIfNotExistsProto;
use substreams_ethereum::pb::eth as pbeth;

use substreams_helper::hex::Hexable;

use crate::abi;
use crate::abi::LendingPoolConfigurator::events as lending_pool_configurator;
use crate::pb::aave_v2::v1::AToken;
use crate::pb::aave_v2::v1::AtokenType;
use crate::pb::aave_v2::v1::Contract;

#[substreams::handlers::store]
fn store_observed_contracts(
    contract_params: String,
    block: pbeth::v2::Block,
    store: store::StoreSetIfNotExistsProto<Contract>,
) {
    let mut observed_addresses: Vec<Address> = vec![];
    for val in contract_params.split(";").into_iter() {
        let addr = Address::from_str(&val);
        match addr {
            Ok(addr) => {
                observed_addresses.push(addr);
                store.set_if_not_exists(0, addr.to_hex(), &Contract { token: None });
            }
            Err(_) => {
                panic!("Invalid contract address");
            }
        }
    }

    for log in block.logs() {
        let addr = Address::from_slice(&log.address());
        if !observed_addresses.contains(&addr) {
            continue;
        }

        if !lending_pool_configurator::ReserveInitialized::match_log(log.log) {
            continue;
        }

        let event = lending_pool_configurator::ReserveInitialized::decode(log.log).unwrap();
        let token_params = |token_address: Vec<u8>, atype: AtokenType| -> AToken {
            let name = abi::AToken::functions::Name {}
                .call(token_address.clone())
                .unwrap();
            AToken {
                r#type: atype.into(),
                name: name,
                address: token_address.to_hex(),
                asset: event.asset.clone().to_hex(),
            }
        };

        let a_token = token_params(event.a_token, AtokenType::Atoken);
        let stable_token = token_params(event.stable_debt_token, AtokenType::StableDebt);
        let var_token = token_params(event.variable_debt_token, AtokenType::VariableDebt);

        store.set_if_not_exists(
            0,
            a_token.address.to_owned(),
            &Contract {
                token: Some(a_token),
            },
        );
        store.set_if_not_exists(
            0,
            stable_token.address.to_owned(),
            &Contract {
                token: Some(stable_token),
            },
        );
        store.set_if_not_exists(
            0,
            var_token.address.to_owned(),
            &Contract {
                token: Some(var_token),
            },
        );
    }
}
