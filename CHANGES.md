# Library Inner Functionment Modification

* Commit ID is between parenthesis

## v0.1.1

* Implemntation of Tokio to make the server async (d759f0de2b069b1938b3920646c18aa6224d3191) 
* Client Request's Path is now forced to be lowercased (81645fc1f246436ed98c6a388aec44442ea42819)
* Implmentation of the Checkpoint Struct and CheckpointManager (066065d6811670fe2a9f577080d9e910736d5e00, ea50aebae7282e2e4788f441c40a189effcfda54)
* Global Library Architecture Modification (ea50aebae7282e2e4788f441c40a189effcfda54)
* Implementation of the Protocol Enum and Insertion in the flow (98884c91eb03437c88e567ce7450e5bd741a5f6c)
* Removing the env_logger::init() that could've caused crash on the end-user side (e83bcb733471b1c0a8ea131f1712057bd9734fe3)
* Update of Multiple StatusCode that needs Location, Upgrade and RetryAfter headers (3341168ef420c92d43c253ab55192158b526b184)
* Implementation of HEAD HTTP Method (eb78f29dfd17534aa5cccbed82cdaf21b217c05f)
