use crate::abi;
use crate::pb;

use abi::erc20::functions;
use lazy_static;
use pb::erc20::v1::Erc20Token;
use std::collections::HashMap;
use substreams::scalar::BigInt;
use substreams::Hex;

lazy_static::lazy_static! {
    pub static ref TOKENS: HashMap<&'static str, &'static str> = {
        let token_mapping: HashMap<&str, &str> = HashMap::from([
            ("USD", "dac17f958d2ee523a2206206994597c13d831ec7"),
            ("CRO", "a0b73e1ff0b80914ab6fe0444e65848c4c34450b"),
            ("STMX", "be9375c6a420d2eeb258962efb95551a5b722803"),
            ("SRM", "476c5e26a75bd202a9683ffd34359c0cc15be0ff"),
            ("ETH", "c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
            ("BAND", "ba11d00c5f74255f56a5e366f4f77f5a186d7f55"),
            ("ERN", "bbc2ae13b23d715c30720f079fcd9b4a74093505"),
            ("COVER", "4688a8b1f292fdab17e9a90c8bc379dc1dbd8713"),
            ("BAT", "0d8775f648430679a709e98d2b0cb6250d2887ef"),
            ("REN", "408e41876cccdc0f92210600ef50372656052a38"),
            ("WING", "cb3df3108635932d912632ef7132d03ecfc39080"),
            ("LON", "0000000000095413afc295d19edeb1ad7b71c952"),
            ("FRAX", "853d955acef822db058eb8505911ed77f175b99e"),
            ("LRC", "bbbbca6a901c926f240b89eacb641d8aec7aeafd"),
            ("GRT", "c944e90c64b2c07662a292be6244bdf05cda44a7"),
            ("COMP", "c00e94cb662c3520282e6f5717214004a7f26888"),
            ("HUSD", "df574c24545e5ffecb9a659c229253d4111d87e1"),
            ("BNT", "1f573d6fb3f13d689ff844b4ce37794d79a7ff1c"),
            ("DNT", "0abdace70d3790235af448c88547603b945604ea"),
            ("OKB", "75231f58b43240c9718dd58b4967c5114342a86c"),
            ("ADX", "ade00c28244d5ce17d72e40330b1c318cd12b7c3"),
            ("MKR", "9f8f72aa9304c8b593d555f12ef6589cc3a579a2"),
            ("ENJ", "f629cbd94d3791c9250152bd8dfbdf380e2a3b9c"),
            ("TRU", "4c19596f5aaff459fa38b0f7ed92f11ae6543784"),
            ("ZRX", "e41d2489571d322189246dafa5ebde1f4699f498"),
            ("FTM", "4e15361fd6b4bb609fa63c81a2be19d873717870"),
            ("RARI", "fca59cd816ab1ead66534d82bc21e7515ce441cf"),
            ("LINK", "514910771af9ca656af840dff83e8264ecf986ca"),
            ("OGN", "8207c1ffc5b6804f6024322ccf34f29c3541ae26"),
            ("SAND", "3845badade8e6dff049820680d1f14bd3903a5d0"),
            ("TUSD", "0000000000085d4780b73119b644ae5ecd22b376"),
            ("USDT", "dac17f958d2ee523a2206206994597c13d831ec7"),
            ("PAX", "8e870d67f660d95d5be530380d0ec0bd388289e1"),
            ("PERP", "bc396689893d065f41bc2c6ecbee5e0085233447"),
            ("DIGG", "798d1be841a82a273720ce31c822c61a67a601c3"),
            ("BTC", "2260fac5e5542a773aa44fbcfedf7c193bc2c599"),
            ("RUNE", "3155ba85d5f96b2d030a4966af206230e46849cb"),
            ("AMPL", "d46ba6d942050d489dbd938a2c909a5d5039a161"),
            ("RAMP", "33d0568941c0c64ff7e0fb4fba0b11bd37deed9f"),
            ("RAI", "03ab458634910aad20ef5f1c8ee96f1d6ac54919"),
            ("RLC", "607f4c5bb672230e8672085532f7e901544a7375"),
            ("LDO", "5a98fcbea516cf06857215779fd812ca3bef1b32"),
            ("AAVE", "7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9"),
            ("UNI", "1f9840a85d5af5bf1d1762f925bdaddc4201f984"),
            ("OMG", "d26114cd6ee289accf82350c8d8487fedb8a0c07"),
            ("BADGER", "3472a5a71965499acd81997a54bba8d852c6e53d"),
            ("BUSD", "4fabb145d64652a948d72533023f6e7a623c7c53"),
            ("KNC", "dd974d5c2e2928dea5f71b9825b8b646686bd200"),
            ("FXS", "3432b6a60d23ca0dfca7761b7ab56459d9c964d0"),
            ("1INCH", "111111111117dc0aa78b770fa6a738034120c302"),
            ("CEL", "aaaebe6fe48e54f431b0c390cfaf0b017d09d42d"),
            ("DAI", "6b175474e89094c44da98b954eedeac495271d0f"),
            ("SNX", "c011a73ee8576fb46f5e1c5751ca3b9fe0af2a6f"),
            ("FEI", "956f47f50a910163d8bf957cf5846d573e7f87ca"),
            ("USDN", "674c6ad92fd080e4004b2312b45f796a192d27a0"),
            ("MATIC", "7d1afa7b718fb893db30a3abc0cfc608aacfebb0"),
            ("YFI", "0bc529c00c6401aef6d220be8c6ea1667f6ad93e"),
            ("OCEAN", "967da4048cd07ab37855c090aaf366e4ce1b9f48"),
            ("ANKR", "8290333cef9e6d528dd5618fb97a76f268f3edd4"),
            ("CREAM", "2ba592f78db6436527729929aaf6c908497cb200"),
            ("MANA", "0f5d2fb29fb7d3cfee444a200298f468908cc942"),
            ("TRIBE", "c7283b66eb1eb5fb86327f08e1b5816b0720212b"),
            ("AMP", "ff20817765cb7f73d4bde2e66e067e58d11095c2"),
            ("ALPHA", "a1faa113cbe53436df28ff0aee54275c13b40975"),
            ("CRV", "d533a949740bb3306d119cc777fa900ba034cd52"),
            ("BZRX", "56d811088235f11c8920698a204a5010a788f4b3"),
            ("ANT", "a117000000f279d81a1d3cc75430faa017fa5a2e"),
            ("USDC", "a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
            ("WOO", "4691937a7508860f876c9c0a2a617e7d9e945d4b"),
            ("MTA", "a3bed4e1c75d00fa6f4e5e6922db7261b5e9acd2"),
            ("VSP", "1b40183efb4dd766f11bda7a7c3ad8982e998421"),
            ("PAXG", "45804880de22913dafe09f4980848ece6ecbaf78"),
            ("NMR", "1776e1f26f98b1a5df9cd347953a26dd3cb46671"),
            ("GNO", "6810e776880c02933d47db1b9fc05908e5386b96"),
            ("AUCTION", "a9b1eb5908cfc3cdf91f9b8b3a74108598009096"),
            ("HEGIC", "584bc13c7d411c00c01a62e8019472de68768430"),
            ("TRY", "c12ecee46ed65d970ee5c899fcc7ae133aff9b03"),
            ("AKRO", "8ab7404063ec4dbcfd4598215992dc3f8ec853d7"),
            ("BAL", "ba100000625a3754423978a60c9317c58a424e3d"),
            ("SUSHI", "6b3595068778dd592e39a122f4f5a5cf09c90fe2"),
            ("REP", "221657776846890989a759ba2973e427dff5c9bb"),
            ("MLN", "ec67005c4e498ec7f55e092bd1d35cbc47c91892"),
            ("HT", "6f259637dcd74c767781e37bc6133cd6a68aa161"),
            ("RCN", "f970b8e36e23f7fc3fd752eea86f8be8d83375a6"),
            ("FTT", "50d1c9771902476076ecfc8b2a83ad6b9355a4c9"),
            ("SXP", "8ce9137d39326ad0cd6491fb5cc0cba0e089b6a9"),
            ("NU", "4fe83213d56308330ec302a8bd641f1d0113a4cc"),
            ("STAKE", "0ae055097c6d159879521c384f1d2123d1f195e6"),
            ("UST", "a47c8bf37f92abed4a126bda807a7b7498661acd"),
            ("sDEFI", "e1afe1fd76fd88f78cbf599ea1846231b8ba3b6b"),
            ("INJ", "e28b3b32b6c345a34ff64674606124dd5aceca30"),
            ("YFII", "a1d0e215a23d7030842fc67ce582a6afa3ccab83"),
            ("sUSD", "57ab1ec28d129707052df4df418d58a2d46d5f51"),
            ("ORN", "0258f474786ddfd37abce6df6bbb1dd5dfc4434a"),
            ("FRONT", "f8c3527cc04340b208c854e985240c02f7b7793f"),
            ("RGT", "d291e7a03283640fdc51b121ac401383a46cc623"),
            ("WOM", "bd356a39bff2cada8e9248532dd879147221cf76"),
            ("OXT", "4575f41308ec1483f3d399aa9a2826d74da13deb"),
            ("BOND", "0391d2021f89dc339f60fff84546ea23e337750f"),
            ("KP3R", "1ceb5cb57c4d4e2b2433641b95dd330a33185a44"),
            ("SFI", "b753428af26e81097e7fd17f40c88aaa3e04902c"),
            ("DIA", "84ca8bc7997272c7cfb4d0cd3d55cd942b3c9419"),
            ("UMA", "04fa0d235c4abf4bcf4787af4cf447de572ef828"),
            ("SWAP", "cc4304a31d09258b0029ea7fe63d032f52e44efe"),
            ("DPI", "1494ca1f11d487c2bbe4543e90080aeba4ba3c2b"),
            ("SUSD", "57ab1ec28d129707052df4df418d58a2d46d5f51"),
            ("WNXM", "0d438f3b5175bebc262bf23753c1e53d03432bde"),
            ("ALCX", "dbdb4d16eda451d0503b854cf79d55697f90c8df"),
            ("LUSD", "5f98805a4e8be255a32880fdec7f6728c6568ba0"),
            ("GHST", "3f382dbd960e3a9bbceae22651e88158d2791550"),
            ("FARM", "a0246c9032bc3a600820415ae600c6388619a14d"),
            ("STETH", "ae7ab96520de3a18e5e111b5eaab095312d7fe84"),
            ("OHM", "383518188c0c6d7730d91b2c03a03c837814a899"),
            ("ZCN", "b9ef770b6a5e12e45983c5d80545258aa38f3b78"),
            ("OM", "3593d125a4f7849a1b059e64f4517a86dd60c95d"),
            ("USDK", "1c48f86ae57291f7686349f12601910bd8d470bb"),
            ("ACH", "ed04915c23f00a313a544955524eb7dbd823143d"),
            ("USDP", "8e870d67f660d95d5be530380d0ec0bd388289e1"),
            ("ATA", "a2120b9e674d3fc3875f415a7df52e382f141225"),
            ("GUSD", "056fd409e1d7a124bd7017459dfea2f387b6d5cd"),
            ("AUDIO", "18aaa7115705e8be94bffebde57af9bfc265b998"),
            ("MASK", "69af81e73a73b40adf4f3d4223cd9b1ece623074"),
            ("FET", "aea46a60368a7bd060eec7df8cba43b7ef41ad85"),
            ("AXS", "bb0e17ef65f82ab018d8edd776e8dd940327b28b"),
            ("EURT", "c581b735a1688071a1746c968e0798d642ede491"),
            ("CTSI", "491604c0fdf08347dd1fa4ee062a822a5dd06b5d"),
            ("XSUSHI", "8798249c2e607446efb7ad49ec89dd1865ff4272"),
            ("LUNA", "d2877702675e6ceb975b4a1dff9fb7baf4c91ea9"),
            ("ALUSD", "bc6da0fe9ad5f3b0d58160288917aa56653660e9"),
            ("CELR", "4f9254c83eb525f9fcf346490bbb3ed28a81c667"),
            ("BORING", "bc19712feb3a26080ebf6f2f7849b417fdd792ca"),
            ("GTC", "de30da39c46104798bb5aa3fe8b9e0e1f348163f"),
            ("YGG", "25f8087ead173b73d6e8b84329989a8eea16cf73"),
            ("DODO", "43dfc4159d86f3a37a5a4b3d4580b888ad7d4ddd"),
            ("FLOKI", "43f11c02439e2736800433b4594994bd43cd066d"),
            ("SPELL", "090185f2135308bad17527004364ebcc2d37e5f6"),
            ("DYDX", "92d6c1e31e14520e676a687f0a93788b716beff5"),
            ("DATA", "33d63ba1e57e54779f7ddaeaa7109349344cf5f1"),
            ("FORTH", "77fba179c79de5b7653f68b5039af940ada60ce0"),
            ("XCN", "a2cd3d43c775978a96bdbf12d733d5a1ed94fb18"),
        ]);

        token_mapping
    };
}

pub fn get_erc20_token(token_address: String) -> Option<Erc20Token> {
    let token_address_vec = Hex::decode(token_address.clone()).unwrap();

    let name = functions::Name {}
        .call(token_address_vec.clone())
        .unwrap_or(String::new());
    let symbol = functions::Symbol {}
        .call(token_address_vec.clone())
        .unwrap_or(String::new());
    let decimals = functions::Decimals {}
        .call(token_address_vec.clone())
        .unwrap_or(BigInt::zero())
        .to_u64();

    Some(Erc20Token {
        address: token_address,
        name: name,
        symbol: symbol,
        decimals: decimals,
    })
}
