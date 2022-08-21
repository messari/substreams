    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's events.
    #[allow(dead_code)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct AccrueInterest {
            pub interest_accumulated: ethabi::Uint,
            pub borrow_index: ethabi::Uint,
            pub total_borrows: ethabi::Uint,
        }
        impl AccrueInterest {
            const TOPIC_ID: [u8; 32] = [
                135u8,
                83u8,
                82u8,
                251u8,
                63u8,
                173u8,
                235u8,
                140u8,
                11u8,
                231u8,
                203u8,
                190u8,
                143u8,
                247u8,
                97u8,
                179u8,
                8u8,
                250u8,
                112u8,
                51u8,
                71u8,
                12u8,
                208u8,
                40u8,
                127u8,
                2u8,
                243u8,
                67u8,
                111u8,
                215u8,
                108u8,
                185u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    interest_accumulated: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    borrow_index: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    total_borrows: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for AccrueInterest {
            const NAME: &'static str = "AccrueInterest";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Approval {
            pub owner: Vec<u8>,
            pub spender: Vec<u8>,
            pub amount: ethabi::Uint,
        }
        impl Approval {
            const TOPIC_ID: [u8; 32] = [
                140u8,
                91u8,
                225u8,
                229u8,
                235u8,
                236u8,
                125u8,
                91u8,
                209u8,
                79u8,
                113u8,
                66u8,
                125u8,
                30u8,
                132u8,
                243u8,
                221u8,
                3u8,
                20u8,
                192u8,
                247u8,
                178u8,
                41u8,
                30u8,
                91u8,
                32u8,
                10u8,
                200u8,
                199u8,
                195u8,
                185u8,
                37u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    owner: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'owner' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    spender: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'spender' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Approval {
            const NAME: &'static str = "Approval";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Borrow {
            pub borrower: Vec<u8>,
            pub borrow_amount: ethabi::Uint,
            pub account_borrows: ethabi::Uint,
            pub total_borrows: ethabi::Uint,
        }
        impl Borrow {
            const TOPIC_ID: [u8; 32] = [
                19u8,
                237u8,
                104u8,
                102u8,
                212u8,
                225u8,
                238u8,
                109u8,
                164u8,
                111u8,
                132u8,
                92u8,
                70u8,
                215u8,
                229u8,
                65u8,
                32u8,
                136u8,
                61u8,
                117u8,
                197u8,
                234u8,
                154u8,
                45u8,
                172u8,
                193u8,
                196u8,
                202u8,
                137u8,
                132u8,
                171u8,
                128u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    borrower: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    borrow_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    account_borrows: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    total_borrows: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Borrow {
            const NAME: &'static str = "Borrow";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Failure {
            pub error: ethabi::Uint,
            pub info: ethabi::Uint,
            pub detail: ethabi::Uint,
        }
        impl Failure {
            const TOPIC_ID: [u8; 32] = [
                69u8,
                185u8,
                111u8,
                228u8,
                66u8,
                99u8,
                2u8,
                100u8,
                88u8,
                27u8,
                25u8,
                126u8,
                132u8,
                187u8,
                173u8,
                168u8,
                97u8,
                35u8,
                80u8,
                82u8,
                197u8,
                161u8,
                170u8,
                223u8,
                255u8,
                158u8,
                164u8,
                228u8,
                10u8,
                150u8,
                154u8,
                160u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    error: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    info: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    detail: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Failure {
            const NAME: &'static str = "Failure";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct LiquidateBorrow {
            pub liquidator: Vec<u8>,
            pub borrower: Vec<u8>,
            pub repay_amount: ethabi::Uint,
            pub c_token_collateral: Vec<u8>,
            pub seize_tokens: ethabi::Uint,
        }
        impl LiquidateBorrow {
            const TOPIC_ID: [u8; 32] = [
                41u8,
                134u8,
                55u8,
                246u8,
                132u8,
                218u8,
                112u8,
                103u8,
                79u8,
                38u8,
                80u8,
                155u8,
                16u8,
                240u8,
                126u8,
                194u8,
                251u8,
                199u8,
                122u8,
                51u8,
                90u8,
                177u8,
                231u8,
                214u8,
                33u8,
                90u8,
                75u8,
                36u8,
                132u8,
                216u8,
                187u8,
                82u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 160usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    liquidator: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    borrower: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    repay_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    c_token_collateral: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    seize_tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for LiquidateBorrow {
            const NAME: &'static str = "LiquidateBorrow";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Mint {
            pub minter: Vec<u8>,
            pub mint_amount: ethabi::Uint,
            pub mint_tokens: ethabi::Uint,
        }
        impl Mint {
            const TOPIC_ID: [u8; 32] = [
                76u8,
                32u8,
                155u8,
                95u8,
                200u8,
                173u8,
                80u8,
                117u8,
                143u8,
                19u8,
                226u8,
                225u8,
                8u8,
                139u8,
                165u8,
                106u8,
                86u8,
                13u8,
                255u8,
                105u8,
                10u8,
                28u8,
                111u8,
                239u8,
                38u8,
                57u8,
                79u8,
                76u8,
                3u8,
                130u8,
                28u8,
                79u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    minter: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    mint_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    mint_tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Mint {
            const NAME: &'static str = "Mint";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct NewAdmin {
            pub old_admin: Vec<u8>,
            pub new_admin: Vec<u8>,
        }
        impl NewAdmin {
            const TOPIC_ID: [u8; 32] = [
                249u8,
                255u8,
                171u8,
                202u8,
                156u8,
                130u8,
                118u8,
                233u8,
                147u8,
                33u8,
                114u8,
                91u8,
                203u8,
                67u8,
                251u8,
                7u8,
                106u8,
                108u8,
                102u8,
                165u8,
                75u8,
                127u8,
                33u8,
                196u8,
                232u8,
                20u8,
                109u8,
                133u8,
                25u8,
                180u8,
                23u8,
                220u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    old_admin: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_admin: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for NewAdmin {
            const NAME: &'static str = "NewAdmin";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct NewComptroller {
            pub old_comptroller: Vec<u8>,
            pub new_comptroller: Vec<u8>,
        }
        impl NewComptroller {
            const TOPIC_ID: [u8; 32] = [
                122u8,
                195u8,
                105u8,
                219u8,
                209u8,
                79u8,
                165u8,
                234u8,
                63u8,
                71u8,
                62u8,
                214u8,
                124u8,
                201u8,
                213u8,
                152u8,
                150u8,
                74u8,
                119u8,
                80u8,
                21u8,
                64u8,
                186u8,
                103u8,
                81u8,
                235u8,
                11u8,
                61u8,
                236u8,
                245u8,
                135u8,
                13u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    old_comptroller: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_comptroller: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for NewComptroller {
            const NAME: &'static str = "NewComptroller";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct NewMarketInterestRateModel {
            pub old_interest_rate_model: Vec<u8>,
            pub new_interest_rate_model: Vec<u8>,
        }
        impl NewMarketInterestRateModel {
            const TOPIC_ID: [u8; 32] = [
                237u8,
                255u8,
                195u8,
                46u8,
                6u8,
                140u8,
                124u8,
                149u8,
                223u8,
                212u8,
                189u8,
                253u8,
                92u8,
                77u8,
                147u8,
                154u8,
                8u8,
                77u8,
                107u8,
                17u8,
                196u8,
                25u8,
                158u8,
                172u8,
                132u8,
                54u8,
                237u8,
                35u8,
                77u8,
                114u8,
                249u8,
                38u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    old_interest_rate_model: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_interest_rate_model: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for NewMarketInterestRateModel {
            const NAME: &'static str = "NewMarketInterestRateModel";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct NewPendingAdmin {
            pub old_pending_admin: Vec<u8>,
            pub new_pending_admin: Vec<u8>,
        }
        impl NewPendingAdmin {
            const TOPIC_ID: [u8; 32] = [
                202u8,
                79u8,
                47u8,
                37u8,
                208u8,
                137u8,
                142u8,
                221u8,
                153u8,
                65u8,
                52u8,
                18u8,
                251u8,
                148u8,
                1u8,
                47u8,
                158u8,
                84u8,
                236u8,
                129u8,
                66u8,
                249u8,
                176u8,
                147u8,
                231u8,
                114u8,
                6u8,
                70u8,
                169u8,
                91u8,
                22u8,
                169u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Address],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    old_pending_admin: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_pending_admin: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for NewPendingAdmin {
            const NAME: &'static str = "NewPendingAdmin";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct NewReserveFactor {
            pub old_reserve_factor_mantissa: ethabi::Uint,
            pub new_reserve_factor_mantissa: ethabi::Uint,
        }
        impl NewReserveFactor {
            const TOPIC_ID: [u8; 32] = [
                170u8,
                166u8,
                131u8,
                18u8,
                226u8,
                234u8,
                157u8,
                80u8,
                225u8,
                106u8,
                245u8,
                6u8,
                132u8,
                16u8,
                171u8,
                86u8,
                225u8,
                161u8,
                253u8,
                6u8,
                3u8,
                123u8,
                26u8,
                53u8,
                102u8,
                72u8,
                18u8,
                195u8,
                15u8,
                130u8,
                20u8,
                96u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    old_reserve_factor_mantissa: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_reserve_factor_mantissa: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for NewReserveFactor {
            const NAME: &'static str = "NewReserveFactor";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Redeem {
            pub redeemer: Vec<u8>,
            pub redeem_amount: ethabi::Uint,
            pub redeem_tokens: ethabi::Uint,
        }
        impl Redeem {
            const TOPIC_ID: [u8; 32] = [
                229u8,
                183u8,
                84u8,
                251u8,
                26u8,
                187u8,
                127u8,
                1u8,
                180u8,
                153u8,
                121u8,
                29u8,
                11u8,
                130u8,
                10u8,
                227u8,
                182u8,
                175u8,
                52u8,
                36u8,
                172u8,
                28u8,
                89u8,
                118u8,
                142u8,
                219u8,
                83u8,
                244u8,
                236u8,
                49u8,
                169u8,
                41u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    redeemer: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    redeem_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    redeem_tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Redeem {
            const NAME: &'static str = "Redeem";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct RepayBorrow {
            pub payer: Vec<u8>,
            pub borrower: Vec<u8>,
            pub repay_amount: ethabi::Uint,
            pub account_borrows: ethabi::Uint,
            pub total_borrows: ethabi::Uint,
        }
        impl RepayBorrow {
            const TOPIC_ID: [u8; 32] = [
                26u8,
                42u8,
                34u8,
                203u8,
                3u8,
                77u8,
                38u8,
                209u8,
                133u8,
                75u8,
                220u8,
                102u8,
                102u8,
                165u8,
                185u8,
                31u8,
                226u8,
                94u8,
                251u8,
                187u8,
                93u8,
                202u8,
                211u8,
                176u8,
                53u8,
                84u8,
                120u8,
                214u8,
                245u8,
                195u8,
                98u8,
                161u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 160usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    payer: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    borrower: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    repay_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    account_borrows: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    total_borrows: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for RepayBorrow {
            const NAME: &'static str = "RepayBorrow";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ReservesReduced {
            pub admin: Vec<u8>,
            pub reduce_amount: ethabi::Uint,
            pub new_total_reserves: ethabi::Uint,
        }
        impl ReservesReduced {
            const TOPIC_ID: [u8; 32] = [
                59u8,
                173u8,
                12u8,
                89u8,
                207u8,
                47u8,
                6u8,
                231u8,
                49u8,
                64u8,
                119u8,
                4u8,
                159u8,
                72u8,
                169u8,
                53u8,
                120u8,
                205u8,
                22u8,
                245u8,
                239u8,
                146u8,
                50u8,
                159u8,
                29u8,
                171u8,
                20u8,
                32u8,
                169u8,
                156u8,
                23u8,
                126u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    admin: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    reduce_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_total_reserves: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ReservesReduced {
            const NAME: &'static str = "ReservesReduced";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Transfer {
            pub from: Vec<u8>,
            pub to: Vec<u8>,
            pub amount: ethabi::Uint,
        }
        impl Transfer {
            const TOPIC_ID: [u8; 32] = [
                221u8,
                242u8,
                82u8,
                173u8,
                27u8,
                226u8,
                200u8,
                155u8,
                105u8,
                194u8,
                176u8,
                104u8,
                252u8,
                55u8,
                141u8,
                170u8,
                149u8,
                43u8,
                167u8,
                241u8,
                99u8,
                196u8,
                161u8,
                22u8,
                40u8,
                245u8,
                90u8,
                77u8,
                245u8,
                35u8,
                179u8,
                239u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    from: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'from' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    to: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'to' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Transfer {
            const NAME: &'static str = "Transfer";
            fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }