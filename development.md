# Substreams - Systematic View of Development and Workflow (Proposal)

## Mapping Process:

### Step 1 - Identify: Identify contracts (data sources), events, and call data that you need.
- Sometimes data sources are singular.
- Sometimes event/call data from particular contracts need to be parsed to get all the data sources you need.
    - A common pattern is having the address of a singular data source like a `Factory` or `Controller` with specific event/call data that identifies a new data source (contract) address.
        - This does not always work as sometimes data sources are created ad hoc (not through some call to another contract)
        - Is there a viable way to monitor contract creation and decode/identify them as a certain contract belonging to a certain protocol in this case?

### Step 2 - Store Data Sources: Process the blockchain for datasources (depending on need) and store all datasource addresses.
- If you need a data source template, this will require processing the entire blockchain to extract all data sources that you care about `before` pre-processing, so you can parallelize without missing data.

### Step 3 - Pre-processing: Identify and map transactions that you care about based on your stored datasources and event/call data you are looking out for.
- The simplest way to do this is to process the logs from the transaction, and identify when a log address is from one of your data sources.
    - I can’t think of another reliable way to do this, and I am pretty sure this will always be faster than using the call data - in instances it may be necessary to search call data as well, though.
- By this time, each transaction should contain at least one piece of data that is relevant to the end module you would like to create.
- Prune Transactions
    - Which events, call data, and other miscellaneous data can be done without?
- Can also further process the transactions to cache event and call data as their own messages unbound from the transaction data abstraction.

### Step 4 - Transformation:
- Augment relevant transaction messages?
    - Transaction label
        - Example (Uniswap V2 - Swap)
            - Direct interaction with the pool
            - Pool interacted with through router contract (official Uniswap V2 Router contract or otherwise)
            - 1Inch swap done on Uniswap V2
    - Event/call counts
    - Type of usage (Example: Swap, Deposit, Withdraw)
        - An advantage of this also is that usage can be identified at the actual transaction level instead of at the log level which is a more accurate portrayal of usage.
    - List of protocols used in the transaction
- Pass transaction messages through multiple mapping modules so you can create more useful or atomic messages that are easier to work with downstream.
    - Event messages
    - Call messages
    - Some data abstractions that contains some mixture of event, call, transaction, and block level data.
- Transactions messages, or whatever group of message that are chosen as proper to work with, are then mapped into very specific data abstractions (Post-Transformation Data) that are required for the modules from our standard library that map and store the final entities we want.

## Why are transactions at a minimum the right format for Post-Processing/Pre-Transformation data?
- Every transaction is initiated by an externally owned account (`EOA` --> transaction.from).
- It is the fundamental unit of interaction with a blockchain, and almost all, if not all, relevant changes to the state follow.
- Multiple events and call data are generated in singular transaction. This keeps them grouped together as a single interaction to be parsed as necessary.

## 4 Types of Data
- Pre-Processed Data
    - Raw blockchain.
- Post-Processed/Pre-Transformation Data
    - All relevant and pruned transaction data (+ optionally atomize transactions into event and call messages as well) (After Step 3).
    - The starting point for any new substream within a general and automated system.
- Post-Transformation Data
    - Data after it has been transformed into a ready state to be passed into generic modules.
- Final Data
    - Output of generic modules for mapping Post-Transformation data into final data (production/data apps input).

## Opportunities for code generation and automation in the mapping process:
- `Step 1` could be facilitated through a CLI tool.
- After you have identified data sources, or how to get them from a `Factory`, `Controller`, or some other means, and then identify all events/call data that interests you, you should be able to generate the substreams.yaml and code to do `Steps 2-3 automatically`.
- If you specify what final data you are interested in, the necessary types can be automatically generated (or in a common location to be referenced), and modules or `.spkgs` can be placed in the substreams.yaml.
- Essentially, the novelty of any particular substream is in how we go from Post-Processing/Pre-Transformation data to Post-Transformation data.

## Where is developer discretion necessary/most important?
- Identifying data sources and relevant events/call data.
    - Ad hoc data sources can be very important to keep track of for some protocols.
        - How to we handle and track add hoc data sources?
- What data abstractions do you work with or create in and after `Step 3` between Pre-Transformation and Post-Transformation data?
    - How do you get from A to B?
    - What are easy to use and efficient formats for a particular substream?
    - Do we always atomize the transaction messages into event and call messages?
    - Do we need to do anymore processing of raw data before transformation step?
- What should the Post-Transformation data look like and how do we map from Post-Transformational data into Final Data (working on a library)? 
