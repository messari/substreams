const fs = require('fs');
const { argv } = require('process');
const { S3Client, PutObjectCommand } = require("@aws-sdk/client-s3");
const params = require('../config/params.json');

(async () => {
    const REGION = argv[2];
    const BUCKET = argv[3];

    const client = new S3Client({
        region: REGION,
    });

    const promises = [];
    const spkgs = fs.readdirSync('/tmp/spkgs');
    for (let spkg of spkgs) {
        if (!spkg.endsWith(".spkg")) {
            continue;
        }

        if (!isSpkgConfigured(spkg)) {
            // not uploading spkg if there are no configs for it
            continue;    
        }

        const spkgCommand = new PutObjectCommand({
            Bucket: BUCKET,
            Key: `spkgs/${spkg}`,
            Body: fs.readFileSync(`/tmp/spkgs/${spkg}`, 'binary'),
        });

        promises.push(
            client.send(spkgCommand)
        );
        console.log("Uploading spkg: " + spkg);
    }

    const configs = getAllConfigs();
    for (let protocol in configs) {
        const jsonL = configs[protocol];
        const configCommand = new PutObjectCommand({
            Bucket: BUCKET,
            Key: `configs/${protocol}.json`,
            Body: jsonL
        });
        promises.push(
            client.send(configCommand)
        );
        console.log("Uploading config for: " + protocol);
    }

    try {
        await Promise.all(promises);
    } catch (e) {
        console.error(e);
        process.exit(1);
    }
    console.log("Successfully uploaded spkgs and config to S3");
})();

function isSpkgConfigured(spkg) {
    for (let protocol of params) {
        if (spkg.startsWith(protocol.name)) {
            return true;
        }
    }
    return false;
}

function getAllConfigs() {
    const spkgConfigs = {};
    for (let protocol of params) {
        const configs = [];
        for (let module of protocol.outputModules)  {
            for (let deployment of protocol.deployments) {
                configs.push(JSON.stringify({
                    name: protocol.name,
                    output_module: module,
                    chain_override: deployment.network,
                    param_overrides: paramsToCLIFormat(deployment.params),
                    start_block_overrides: blockOverridesToCLIFormat(deployment.startBlocks),
                }));
            }

            break; // only supporting one module per deployment for now.
        }

        spkgConfigs[protocol.name] = configs.join("\n");
    }
    return spkgConfigs;
}

// grabs param overrides from config and formats them for the parquet sink CLI
function paramsToCLIFormat(params) {
    const ret = [];
    for (let module in params) {
        ret.push({
            module: module,
            value: params[module],
        });
    }
    return ret;
}

// grabs startBlock overrides from config and formats them for the parquet sink CLI
function blockOverridesToCLIFormat(overrides) {
    const ret = [];
    for (let module in overrides) {
        ret.push({
            module: module,
            block_number: overrides[module],
        });
    }
    return ret;
}