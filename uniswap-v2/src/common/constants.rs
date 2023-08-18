// pub const WETH_ADDRESS: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
// pub const USDC_ADDRESS: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";

// pub const UNISWAP_V2_FACTORY: &str = "0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f";

// pub const STABLE_COINS: [&str; 5] = [
//     "0x6b175474e89094c44da98b954eedeac495271d0f", // DAI
//     "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", // USDC
//     "0xdac17f958d2ee523a2206206994597c13d831ec7", // USDT
//     "0x0000000000085d4780b73119b644ae5ecd22b376", // TUSD
//     "0x956f47f50a910163d8bf957cf5846d573e7f87ca", // FEI
// ];

// pub const PAIR_COINS: [&str; 4] = [
//     "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2", // WETH
//     "0x6b175474e89094c44da98b954eedeac495271d0f", // DAI
//     "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", // USDC
//     "0xdac17f958d2ee523a2206206994597c13d831ec7", // USDT
// ];

// pub const WHITELIST_TOKENS: [&str; 21] = [
//     "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2", // WETH
//     "0x6b175474e89094c44da98b954eedeac495271d0f", // DAI
//     "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", // USDC
//     "0xdac17f958d2ee523a2206206994597c13d831ec7", // USDT
//     "0x0000000000085d4780b73119b644ae5ecd22b376", // TUSD
//     "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599", // WBTC
//     "0x5d3a536e4d6dbd6114cc1ead35777bab948e3643", // cDAI
//     "0x39aa39c021dfbae8fac545936693ac917d5e7563", // cUSDC
//     "0x86fadb80d8d2cff3c3680819e4da99c10232ba0f", // EBASE
//     "0x57ab1ec28d129707052df4df418d58a2d46d5f51", // sUSD
//     "0x9f8f72aa9304c8b593d555f12ef6589cc3a579a2", // MKR
//     "0xc00e94cb662c3520282e6f5717214004a7f26888", // COMP
//     "0x514910771af9ca656af840dff83e8264ecf986ca", // LINK
//     "0xc011a73ee8576fb46f5e1c5751ca3b9fe0af2a6f", // SNX
//     "0x0bc529c00c6401aef6d220be8c6ea1667f6ad93e", // YFI
//     "0x111111111117dc0aa78b770fa6a738034120c302", // 1INCH
//     "0xdf5e0e81dff6faf3a7e52ba697820c5e32d806a8", // yCurv
//     "0x956f47f50a910163d8bf957cf5846d573e7f87ca", // FEI
//     "0x7d1afa7b718fb893db30a3abc0cfc608aacfebb0", // MATIC
//     "0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9", // AAVE
//     "0xfe2e637202056d30016725477c5da089ab0a043a", // sETH2
// ];

pub const WETH_ADDRESS: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
pub const USDC_ADDRESS: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";

pub const UNISWAP_V2_FACTORY: &str = "0xcA143Ce32Fe78f1f7019d7d551a6402fC5350c73";

pub const STABLE_COINS: [&str; 5] = [
    "0x55d398326f99059ff775485246999027b3197955", // BUSD
    "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d", // USDC
    "0xe9e7cea3dedca5984780bafc599bd69add087d56", // USDT
    "0x1af3f329e8be154074d8769d1ffa4ee058b1dbc3", // DAI
    "0x40af3827F39D0EAcBF4A168f8D4ee67c121D11c9", // TUSD
];

pub const PAIR_COINS: [&str; 4] = [
    "0x2170ed0880ac9a755fd29b2688956bd959f933f8", // WETH
    "0x1af3f329e8be154074d8769d1ffa4ee058b1dbc3", // DAI
    "0x55d398326f99059ff775485246999027b3197955", // USDC
    "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d", // USDT
];

pub const WHITELIST_TOKENS: [&str; 11] = [
    "0x2170ed0880ac9a755fd29b2688956bd959f933f8", // WETH
    "0x1af3f329e8be154074d8769d1ffa4ee058b1dbc3", // DAI
    "0x55d398326f99059ff775485246999027b3197955", // USDC
    "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d", // USDT
    "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c", // wBNB
    "0x3ee2200efb3400fabb9aacf31297cbdd1d435d47", // ADA
    "0xcc42724c6683b7e57334c4e856f4c9965ed682bd", // Matic
    "0xf8a0bf9cf54bb92f17374d9e9a321e6a111a51bd", // Chainlink
    "0xbf5140a22578168fd562dccf235e5d43a02ce9b1", // UNI
    "0xad29abb318791d579433d831ed122afeaf29dcfe", // FTM
    "0x26433c8127d9b4e9b71eaa15111df99ea2eeb2f8", // MANA
];
