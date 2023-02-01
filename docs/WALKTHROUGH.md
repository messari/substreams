# Walkthrough

This document will explain our substream project setup and terminilogy.

## Structure

Our project is made up of substream modules. Each directory (for example `./eth-balance`) contains a group of modules (1 or more). That group of modules describes data around a central theme. If it contains a `README.md` you can learn more there, or by understanding the manifest file, `substreams.yaml` (described [here](https://substreams.streamingfast.io/developers-guide/creating-your-manifest)).

### `substreams.yaml`

This is the manifest file. it describes the substream modules that make up the substream.

### `./src/*.rs`

This is where the mappings are stored. These are the functions that take the input and transform it into the output defined in the manifest file.

## Terminology

- `module` or `substream module` can either be a `store` or `map` type. These are defined in the manifest file and describe the execution hooks the mapping code executes when the substream compute engine is run. Read more [here](https://substreams.streamingfast.io/concepts-and-fundamentals/modules).
- `proto` or `protobuf` or `protobuf definition`. These terms refer to Google Protocol Buffers (`*.proto`). This "schema" format is a generic way to structure data. Substreams relies heavily on protobufs to run. The input and output of modules are protobufs. This is the way the data is moved around. Google's [docs](https://developers.google.com/protocol-buffers) and substream [docs](https://substreams.streamingfast.io/developers-guide/creating-protobuf-schemas).
- A `sink` is the final destination for data. We use this term a lot, but sink is a term used in general ETL. In our case substreams is a Transform layer, but to really get the data in a usable format we need to stream our substream module output into a sink. This can be in the form of a subgraph, parquet file sink, etc.
