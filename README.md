# WorkflowDemo

Demo showing how businesses can define common workflow, implement as upgradable ink! smart contracts on Substrate blockchains, and conduct transactions on blockchain transparently and trustlessly. 

Detailed requirements to be defined in separate file

Several technical components:

1. Serverless Workflow DSL (spec: https://github.com/serverlessworkflow/specification and examples https://github.com/serverlessworkflow/specification/tree/main/examples) 
2. ink! macros translate workflow DSL spec into ink! smart contracts including Rust macros for code simplicity and reuse  
3. Compile into Wasm bytecode and deploy auto generated smart contracts on Substrate blockchain and execute
4. When workflow changes, seamlessly upgrade smart contracts on Substrate blockchain
5. Benchmark gas fees & performance of smart contract implementations to optimize, and compare with Solidity / EVM based implementations
6. Documentations  

