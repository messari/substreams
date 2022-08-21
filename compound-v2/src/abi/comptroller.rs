    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's events.
    #[allow(dead_code)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct ActionPaused1 {
            pub action: String,
            pub pause_state: bool,
        }
        impl ActionPaused1 {
            const TOPIC_ID: [u8; 32] = [
                239u8,
                21u8,
                157u8,
                154u8,
                50u8,
                178u8,
                71u8,
                46u8,
                50u8,
                176u8,
                152u8,
                249u8,
                84u8,
                243u8,
                206u8,
                98u8,
                210u8,
                50u8,
                147u8,
                159u8,
                28u8,
                32u8,
                112u8,
                112u8,
                181u8,
                132u8,
                223u8,
                24u8,
                20u8,
                222u8,
                45u8,
                224u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v1::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String, ethabi::ParamType::Bool],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    action: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    pause_state: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ActionPaused1 {
            const NAME: &'static str = "ActionPaused1";
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
        pub struct ActionPaused2 {
            pub c_token: Vec<u8>,
            pub action: String,
            pub pause_state: bool,
        }
        impl ActionPaused2 {
            const TOPIC_ID: [u8; 32] = [
                113u8,
                174u8,
                198u8,
                54u8,
                36u8,
                63u8,
                151u8,
                9u8,
                187u8,
                0u8,
                7u8,
                174u8,
                21u8,
                233u8,
                175u8,
                184u8,
                21u8,
                10u8,
                176u8,
                23u8,
                22u8,
                215u8,
                95u8,
                215u8,
                87u8,
                59u8,
                229u8,
                204u8,
                9u8,
                110u8,
                3u8,
                176u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 128usize {
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
                            ethabi::ParamType::String,
                            ethabi::ParamType::Bool,
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    c_token: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    action: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_string()
                        .expect(INTERNAL_ERR),
                    pause_state: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ActionPaused2 {
            const NAME: &'static str = "ActionPaused2";
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
        pub struct CompAccruedAdjusted {
            pub user: Vec<u8>,
            pub old_comp_accrued: ethabi::Uint,
            pub new_comp_accrued: ethabi::Uint,
        }
        impl CompAccruedAdjusted {
            const TOPIC_ID: [u8; 32] = [
                74u8,
                92u8,
                19u8,
                78u8,
                40u8,
                181u8,
                55u8,
                167u8,
                101u8,
                70u8,
                153u8,
                62u8,
                163u8,
                127u8,
                59u8,
                96u8,
                217u8,
                25u8,
                4u8,
                118u8,
                223u8,
                115u8,
                86u8,
                211u8,
                132u8,
                42u8,
                164u8,
                9u8,
                2u8,
                226u8,
                15u8,
                4u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 2usize {
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
                    user: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'user' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    old_comp_accrued: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_comp_accrued: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for CompAccruedAdjusted {
            const NAME: &'static str = "CompAccruedAdjusted";
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
        pub struct CompBorrowSpeedUpdated {
            pub c_token: Vec<u8>,
            pub new_speed: ethabi::Uint,
        }
        impl CompBorrowSpeedUpdated {
            const TOPIC_ID: [u8; 32] = [
                32u8,
                175u8,
                142u8,
                121u8,
                28u8,
                201u8,
                143u8,
                116u8,
                178u8,
                215u8,
                163u8,
                145u8,
                200u8,
                9u8,
                128u8,
                202u8,
                142u8,
                90u8,
                235u8,
                243u8,
                212u8,
                6u8,
                11u8,
                245u8,
                129u8,
                153u8,
                123u8,
                106u8,
                202u8,
                226u8,
                229u8,
                55u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 2usize {
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
                    c_token: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'c_token' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_speed: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for CompBorrowSpeedUpdated {
            const NAME: &'static str = "CompBorrowSpeedUpdated";
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
        pub struct CompGranted {
            pub recipient: Vec<u8>,
            pub amount: ethabi::Uint,
        }
        impl CompGranted {
            const TOPIC_ID: [u8; 32] = [
                152u8,
                178u8,
                248u8,
                42u8,
                58u8,
                7u8,
                242u8,
                35u8,
                160u8,
                190u8,
                100u8,
                179u8,
                208u8,
                244u8,
                119u8,
                17u8,
                198u8,
                77u8,
                204u8,
                209u8,
                254u8,
                175u8,
                185u8,
                74u8,
                162u8,
                129u8,
                86u8,
                179u8,
                140u8,
                217u8,
                105u8,
                92u8,
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
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    recipient: values
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
        impl substreams_ethereum::Event for CompGranted {
            const NAME: &'static str = "CompGranted";
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
        pub struct CompReceivableUpdated {
            pub user: Vec<u8>,
            pub old_comp_receivable: ethabi::Uint,
            pub new_comp_receivable: ethabi::Uint,
        }
        impl CompReceivableUpdated {
            const TOPIC_ID: [u8; 32] = [
                23u8,
                254u8,
                160u8,
                157u8,
                154u8,
                124u8,
                164u8,
                27u8,
                47u8,
                159u8,
                145u8,
                24u8,
                241u8,
                143u8,
                68u8,
                132u8,
                138u8,
                98u8,
                233u8,
                199u8,
                13u8,
                85u8,
                221u8,
                67u8,
                133u8,
                19u8,
                30u8,
                178u8,
                207u8,
                27u8,
                126u8,
                71u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 2usize {
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
                    user: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'user' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    old_comp_receivable: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_comp_receivable: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for CompReceivableUpdated {
            const NAME: &'static str = "CompReceivableUpdated";
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
        pub struct CompSupplySpeedUpdated {
            pub c_token: Vec<u8>,
            pub new_speed: ethabi::Uint,
        }
        impl CompSupplySpeedUpdated {
            const TOPIC_ID: [u8; 32] = [
                222u8,
                175u8,
                204u8,
                208u8,
                192u8,
                183u8,
                104u8,
                178u8,
                82u8,
                159u8,
                125u8,
                203u8,
                190u8,
                88u8,
                225u8,
                85u8,
                214u8,
                2u8,
                48u8,
                89u8,
                21u8,
                11u8,
                116u8,
                144u8,
                237u8,
                69u8,
                53u8,
                204u8,
                55u8,
                68u8,
                185u8,
                45u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 2usize {
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
                    c_token: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'c_token' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_speed: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for CompSupplySpeedUpdated {
            const NAME: &'static str = "CompSupplySpeedUpdated";
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
        pub struct ContributorCompSpeedUpdated {
            pub contributor: Vec<u8>,
            pub new_speed: ethabi::Uint,
        }
        impl ContributorCompSpeedUpdated {
            const TOPIC_ID: [u8; 32] = [
                56u8,
                101u8,
                55u8,
                250u8,
                146u8,
                237u8,
                195u8,
                49u8,
                154u8,
                249u8,
                95u8,
                31u8,
                144u8,
                77u8,
                207u8,
                25u8,
                0u8,
                2u8,
                30u8,
                79u8,
                63u8,
                78u8,
                8u8,
                22u8,
                154u8,
                87u8,
                122u8,
                9u8,
                7u8,
                110u8,
                102u8,
                179u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 2usize {
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
                    contributor: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'contributor' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_speed: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ContributorCompSpeedUpdated {
            const NAME: &'static str = "ContributorCompSpeedUpdated";
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
        pub struct DistributedBorrowerComp {
            pub c_token: Vec<u8>,
            pub borrower: Vec<u8>,
            pub comp_delta: ethabi::Uint,
            pub comp_borrow_index: ethabi::Uint,
        }
        impl DistributedBorrowerComp {
            const TOPIC_ID: [u8; 32] = [
                31u8,
                195u8,
                236u8,
                192u8,
                135u8,
                216u8,
                210u8,
                209u8,
                94u8,
                35u8,
                208u8,
                3u8,
                42u8,
                245u8,
                164u8,
                112u8,
                89u8,
                195u8,
                137u8,
                45u8,
                0u8,
                61u8,
                142u8,
                19u8,
                159u8,
                220u8,
                182u8,
                187u8,
                50u8,
                124u8,
                153u8,
                166u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 3usize {
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
                    c_token: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'c_token' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    borrower: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'borrower' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    comp_delta: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    comp_borrow_index: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for DistributedBorrowerComp {
            const NAME: &'static str = "DistributedBorrowerComp";
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
        pub struct DistributedSupplierComp {
            pub c_token: Vec<u8>,
            pub supplier: Vec<u8>,
            pub comp_delta: ethabi::Uint,
            pub comp_supply_index: ethabi::Uint,
        }
        impl DistributedSupplierComp {
            const TOPIC_ID: [u8; 32] = [
                44u8,
                174u8,
                205u8,
                23u8,
                208u8,
                47u8,
                86u8,
                250u8,
                137u8,
                119u8,
                5u8,
                220u8,
                199u8,
                64u8,
                218u8,
                45u8,
                35u8,
                124u8,
                55u8,
                63u8,
                112u8,
                104u8,
                111u8,
                78u8,
                13u8,
                155u8,
                211u8,
                191u8,
                4u8,
                0u8,
                234u8,
                122u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 3usize {
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
                    c_token: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'c_token' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    supplier: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'supplier' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    comp_delta: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    comp_supply_index: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for DistributedSupplierComp {
            const NAME: &'static str = "DistributedSupplierComp";
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
        pub struct MarketEntered {
            pub c_token: Vec<u8>,
            pub account: Vec<u8>,
        }
        impl MarketEntered {
            const TOPIC_ID: [u8; 32] = [
                58u8,
                178u8,
                58u8,
                176u8,
                213u8,
                28u8,
                204u8,
                192u8,
                195u8,
                8u8,
                90u8,
                236u8,
                81u8,
                249u8,
                146u8,
                40u8,
                98u8,
                90u8,
                161u8,
                169u8,
                34u8,
                179u8,
                168u8,
                202u8,
                137u8,
                162u8,
                107u8,
                15u8,
                32u8,
                39u8,
                161u8,
                165u8,
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
                    c_token: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    account: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for MarketEntered {
            const NAME: &'static str = "MarketEntered";
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
        pub struct MarketExited {
            pub c_token: Vec<u8>,
            pub account: Vec<u8>,
        }
        impl MarketExited {
            const TOPIC_ID: [u8; 32] = [
                230u8,
                153u8,
                166u8,
                76u8,
                24u8,
                176u8,
                122u8,
                197u8,
                183u8,
                48u8,
                26u8,
                162u8,
                115u8,
                243u8,
                106u8,
                34u8,
                135u8,
                35u8,
                158u8,
                185u8,
                80u8,
                29u8,
                129u8,
                149u8,
                6u8,
                114u8,
                121u8,
                74u8,
                251u8,
                162u8,
                154u8,
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
                    c_token: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    account: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for MarketExited {
            const NAME: &'static str = "MarketExited";
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
        pub struct MarketListed {
            pub c_token: Vec<u8>,
        }
        impl MarketListed {
            const TOPIC_ID: [u8; 32] = [
                207u8,
                88u8,
                59u8,
                176u8,
                197u8,
                105u8,
                235u8,
                150u8,
                127u8,
                128u8,
                107u8,
                17u8,
                96u8,
                28u8,
                76u8,
                185u8,
                60u8,
                16u8,
                49u8,
                4u8,
                133u8,
                198u8,
                122u8,
                221u8,
                95u8,
                131u8,
                98u8,
                194u8,
                242u8,
                18u8,
                50u8,
                31u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 1usize {
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
                        &[ethabi::ParamType::Address],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    c_token: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for MarketListed {
            const NAME: &'static str = "MarketListed";
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
        pub struct NewBorrowCap {
            pub c_token: Vec<u8>,
            pub new_borrow_cap: ethabi::Uint,
        }
        impl NewBorrowCap {
            const TOPIC_ID: [u8; 32] = [
                111u8,
                25u8,
                81u8,
                178u8,
                170u8,
                209u8,
                15u8,
                63u8,
                200u8,
                27u8,
                134u8,
                217u8,
                17u8,
                5u8,
                180u8,
                19u8,
                165u8,
                179u8,
                248u8,
                71u8,
                163u8,
                75u8,
                188u8,
                92u8,
                225u8,
                144u8,
                66u8,
                1u8,
                177u8,
                68u8,
                56u8,
                246u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v1::Log) -> bool {
                if log.topics.len() != 2usize {
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
                    c_token: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| format!(
                            "unable to decode param 'c_token' from topic of type 'address': {}",
                            e
                        ))?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_borrow_cap: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for NewBorrowCap {
            const NAME: &'static str = "NewBorrowCap";
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
        pub struct NewBorrowCapGuardian {
            pub old_borrow_cap_guardian: Vec<u8>,
            pub new_borrow_cap_guardian: Vec<u8>,
        }
        impl NewBorrowCapGuardian {
            const TOPIC_ID: [u8; 32] = [
                237u8,
                169u8,
                134u8,
                144u8,
                229u8,
                24u8,
                233u8,
                160u8,
                95u8,
                142u8,
                198u8,
                131u8,
                118u8,
                99u8,
                225u8,
                136u8,
                33u8,
                27u8,
                45u8,
                168u8,
                244u8,
                144u8,
                102u8,
                72u8,
                179u8,
                35u8,
                242u8,
                193u8,
                212u8,
                67u8,
                78u8,
                41u8,
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
                    old_borrow_cap_guardian: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_borrow_cap_guardian: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for NewBorrowCapGuardian {
            const NAME: &'static str = "NewBorrowCapGuardian";
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
        pub struct NewCloseFactor {
            pub old_close_factor_mantissa: ethabi::Uint,
            pub new_close_factor_mantissa: ethabi::Uint,
        }
        impl NewCloseFactor {
            const TOPIC_ID: [u8; 32] = [
                59u8,
                150u8,
                112u8,
                207u8,
                151u8,
                93u8,
                38u8,
                149u8,
                142u8,
                117u8,
                75u8,
                87u8,
                9u8,
                142u8,
                170u8,
                42u8,
                201u8,
                20u8,
                216u8,
                210u8,
                163u8,
                27u8,
                131u8,
                37u8,
                121u8,
                151u8,
                185u8,
                243u8,
                70u8,
                17u8,
                15u8,
                217u8,
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
                    old_close_factor_mantissa: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_close_factor_mantissa: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for NewCloseFactor {
            const NAME: &'static str = "NewCloseFactor";
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
        pub struct NewCollateralFactor {
            pub c_token: Vec<u8>,
            pub old_collateral_factor_mantissa: ethabi::Uint,
            pub new_collateral_factor_mantissa: ethabi::Uint,
        }
        impl NewCollateralFactor {
            const TOPIC_ID: [u8; 32] = [
                112u8,
                72u8,
                62u8,
                101u8,
                146u8,
                205u8,
                81u8,
                130u8,
                212u8,
                90u8,
                201u8,
                112u8,
                224u8,
                91u8,
                198u8,
                44u8,
                220u8,
                201u8,
                14u8,
                157u8,
                142u8,
                242u8,
                194u8,
                219u8,
                230u8,
                134u8,
                207u8,
                56u8,
                59u8,
                205u8,
                127u8,
                197u8,
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
                    c_token: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    old_collateral_factor_mantissa: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_collateral_factor_mantissa: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for NewCollateralFactor {
            const NAME: &'static str = "NewCollateralFactor";
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
        pub struct NewLiquidationIncentive {
            pub old_liquidation_incentive_mantissa: ethabi::Uint,
            pub new_liquidation_incentive_mantissa: ethabi::Uint,
        }
        impl NewLiquidationIncentive {
            const TOPIC_ID: [u8; 32] = [
                174u8,
                186u8,
                90u8,
                108u8,
                64u8,
                168u8,
                172u8,
                19u8,
                129u8,
                52u8,
                191u8,
                241u8,
                170u8,
                166u8,
                93u8,
                235u8,
                242u8,
                89u8,
                113u8,
                24u8,
                138u8,
                88u8,
                128u8,
                75u8,
                173u8,
                113u8,
                127u8,
                130u8,
                240u8,
                236u8,
                19u8,
                22u8,
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
                    old_liquidation_incentive_mantissa: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    new_liquidation_incentive_mantissa: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for NewLiquidationIncentive {
            const NAME: &'static str = "NewLiquidationIncentive";
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
        pub struct NewPauseGuardian {
            pub old_pause_guardian: Vec<u8>,
            pub new_pause_guardian: Vec<u8>,
        }
        impl NewPauseGuardian {
            const TOPIC_ID: [u8; 32] = [
                6u8,
                19u8,
                182u8,
                238u8,
                106u8,
                4u8,
                240u8,
                208u8,
                159u8,
                57u8,
                14u8,
                77u8,
                147u8,
                24u8,
                137u8,
                75u8,
                159u8,
                106u8,
                199u8,
                253u8,
                131u8,
                137u8,
                124u8,
                216u8,
                209u8,
                136u8,
                150u8,
                186u8,
                87u8,
                156u8,
                64u8,
                30u8,
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
                    old_pause_guardian: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_pause_guardian: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for NewPauseGuardian {
            const NAME: &'static str = "NewPauseGuardian";
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
        pub struct NewPriceOracle {
            pub old_price_oracle: Vec<u8>,
            pub new_price_oracle: Vec<u8>,
        }
        impl NewPriceOracle {
            const TOPIC_ID: [u8; 32] = [
                213u8,
                43u8,
                43u8,
                155u8,
                126u8,
                158u8,
                230u8,
                85u8,
                252u8,
                185u8,
                93u8,
                46u8,
                91u8,
                158u8,
                12u8,
                159u8,
                105u8,
                231u8,
                239u8,
                43u8,
                142u8,
                157u8,
                45u8,
                14u8,
                167u8,
                132u8,
                2u8,
                213u8,
                118u8,
                210u8,
                46u8,
                34u8,
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
                    old_price_oracle: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_price_oracle: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for NewPriceOracle {
            const NAME: &'static str = "NewPriceOracle";
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