module.exports = {
    apps: [{
        name: "substreams-sink-csv",
        script: "./node_modules/substreams-sink-csv/dist/bin/cli.mjs",
        env: {
            SUBSTREAMS_API_KEY: '62017651ea666fd0a915f8d0a9c31f378712dd0710de394e',
            MANIFEST: './curve-finance-substream-v0.1.0.spkg',
            MODULE_NAME: 'map_pool_fees',
            SUBSTREAMS_ENDPOINT: 'polygon.substreams.pinax.network',
            FINAL_BLOCKS_ONLY: 'true',
            START_BLOCK: '13479484',
        }
    }]
}