const Validator = require('jsonschema').Validator;
const subgraphsSchema = require('./schemas/subgraphs.schema.json');
const subgraphs = require('./subgraphs');
const substreams = require('./params');

(() => {
    var v = new Validator();
    const res = v.validate(subgraphs, subgraphsSchema)

    if (!res.valid) {
        console.error('Invalid subgraphs.json file', res.errors);
        throw new Error('Invalid subgraphs.json file');
    }

    validateSubgraphSlugs(subgraphs, substreams);
    console.info("Subgraphs Schema OK!")
})();

// subgraph slugs need to be of the form `{substream_name}/{deployment_name}`.
function validateSubgraphSlugs(subgraphs, substreams) {
    const substreamSlugs = {};
    for (let substream of substreams) {
        for (let deployment of substream.deployments) {
            substreamSlugs[`${substream.name}/${deployment.name}`] = true;
        }
    }

    for (let slug in subgraphs) {
        if (!substreamSlugs[slug]) {
            throw new Error(`Invalid subgraph slug ${slug}. It should be of the form {substream_name}/{deployment_name} for an existing substream and deployment.`);
        }
    }
}
