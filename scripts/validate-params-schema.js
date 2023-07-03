const Validator = require('jsonschema').Validator;
const paramsSchema = require('../config/schemas/params.schema.json');
const params = require('../config/params');

(() => {
    var v = new Validator();
    const res = v.validate(params, paramsSchema)

    if (res.valid) {
        console.info("Params Schema OK!")
        return;
    }

    console.error('Invalid params.json file', res.errors);
    throw new Error('Invalid params.json file');
})();
