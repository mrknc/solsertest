## **1. Download the source code.**

```bash
$ cd some/path 
$ git clone https://github.com/solana-labs/solana.git
$ git clone https://github.com/mrknc/solsertest.git
```

In solana, change the last line in programs/vote/src/lib.rs to 
```
pub use solana_sdk::vote::program::*;
```
## **2. Build solana. **

## **3. cargo run in solsertest**