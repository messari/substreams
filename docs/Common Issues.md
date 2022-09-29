# Common Issues

**Unknown Proto Schema**

When you run into an error message like below when running a substream:

```
map_mint_event: message "messari.dex_amm.v1.MintEvents": (unknown proto schema) "\n\f\n\x011\x1a\x011\"\x011*\x011"
```

It's likely because the output type in your substreams manifest `substreams.yaml` does not match with the result type of your mapping module. 
