# Standards

Standardizing where it makes sense and early pays dividends in terms of developer time in the long run. This file will explore where we standardize and why those decisions were made.

## Protobuf Definitions

Where the same data structures are used multiple times, it makes sense to make a [common](../common) protobuf definition. It shows the user this definition can be found in multiple places. It also makes the code base cleaner. You can find these under [`../common/proto`](../common/proto).

> You can learn more details about this [here](../common/README.md)
