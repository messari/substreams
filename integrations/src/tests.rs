use crate::module::Module;

#[test]
fn test_erc20_price_module_map_price() -> Result<(), Box<dyn std::error::Error>> {
    let module = Module::new(
        "map_price",
        "../erc20-price/substreams.yaml",
        "./target/erc20-price-cache.txt",
    );
    module.test()?;

    Ok(())
}

#[test]
fn test_erc20_market_cap_module_map_market_cap() -> Result<(), Box<dyn std::error::Error>> {
    let mut module = Module::new(
        "map_market_cap",
        "../erc20-market-cap/substreams.yaml",
        "./target/erc20-market-cap-cache.txt",
    );
    module.num_blocks = Some(1);
    module.test()?;

    Ok(())
}

#[test]
fn test_erc721_module_block_to_transfers() -> Result<(), Box<dyn std::error::Error>> {
    let module = Module::new(
        "block_to_transfers",
        "../erc721/substreams.yaml",
        "./target/erc721-cache.txt",
    );
    module.test()?;

    Ok(())
}

#[test]
fn test_uniswap_v2_module_store_pools() -> Result<(), Box<dyn std::error::Error>> {
    let mut module = Module::new(
        "map_pair_created_event,map_pools,store_pools",
        "../uniswap-v2/substreams.yaml",
        "./target/uniswap-v2-cache.txt",
    );
    module.start_block = Some(10008355);
    module.num_blocks = Some(200);
    module.test()?;

    Ok(())
}

// NOTE: Receiving GOAWAY error with grpc clientl;
// May be due to improper API key;
// TODO: Contact SF to understand how to use the gRPC `blocks`
// endpoint.
#[ignore]
#[tokio::test]
async fn test_substreams_client() -> Result<(), Box<dyn std::error::Error>> {
    use crate::substreams_client::{stream_client::StreamClient, Request};
    use tokio_stream::StreamExt;

    const DEFAULT_SUBSTREAMS_ENDPOINT: &str = "https://api-dev.streamingfast.io:443";

    let mut client = StreamClient::connect(DEFAULT_SUBSTREAMS_ENDPOINT).await?;
    let mut request = Request::default();
    request.start_block_num = 13e5 as i64;
    request.stop_block_num = (request.start_block_num + 10) as u64;
    request.output_modules = vec!["sf.ethereum.v1".to_string()];
    let mut stream = client
        .blocks(tonic::Request::new(request))
        .await?
        .into_inner();

    while let Some(output) = stream.next().await {
        println!("{:?}", output?);
    }

    Ok(())
}
