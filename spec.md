# Specifications for Blockchain project

## Desired outcome
- Users should be able to store text files on blockchain network
- Users need to buy storage on network in exchange of coins
- Each file should have some unique ID so that users can share the file with others
- Miners are the ones who provide their storage to store networks' files and get incentive in coins

## Possible malicious activities
- Users may double spend and buy extra storage
- Miners can get incentive and then discard the stored data
- Monopoly of a single miner and other miners not getting a chance due to one of the miner having large storage available
- Miners can access private data of users on network

## Possible malfunctionings
- Miner on which user's data is stored goes down
- User want to store some data but any single miner does NOT have that much amount of empty space, although collectively there is enough space. (this is can be fixed by dividing files into chunks and then storing but it's out of the scope of this project atleast for now)

We want that in worst case only malicious nodes have malicious ledger, but all clean nodes have correct ledger

## Things we keep hardcoded (fixed):
- charges for storing some amount of data over some number of validations
- incentive on successfully storing and proving that you stored some data for 1 validation

## Block structure
- Hash (with **PoW**)
- Previous Hash
- Data

## Transactions
- File stored at some miner's address
- Miner receiving reward
- User asking for some file (signed by user's signature) => fixed cost

-> user requests to store some file -> goes to mempool.
-> miner decides to mine (store the reqested file) from mempool once stored makes a block representing the transaction of file storage and propogates.

## The Plan:
- we just broadcast whole chain over network (accept the one having longest length and mine on top)
    - chance of a attack when real chain is idle for long time
- also all nodes just verify whole chain before accepting
- miners store data based on current chain state
- users add their request to mempool from which miners mine the blocks
- how to validate that specified nodes are storing?

## Validation 
- some nodes are validator nodes
- these nodes ask random nodes for storage proof by asking for hash 
- validation is basically agreements from k2 distinct random (randomness maintained due to PoW) nodes
- we store in ledger that each storage nodes has receive x agreements till now
- once x reaches k2 => give incentive according to fixed amounts per validation and reset x to 0
- here validators cannot do fake validations because even if some node gets lucky and proposes a storage node to be valid, most of other nodes will detect it's reality as out of k2 atleast 51% will be valid assuming noone has (51% or more power in network and PoW ensures randomness)

## Things to maintain on blockchain
1. Balances for all nodes
2. File storage mapping registry (which k nodes are responsible for storing a specific file)

## Things to propogate (mostly some gossip algorithm right?)
1. Blockchain itself
2. mempool

## Extras
- now we want to retrieve our file, we already have the mapping showing which nodes store our file, somehow we need to query them and get the file then decode to get the data. (**EASY**)
- when some nodes fail the verification, some files will no longer have k storaing nodes, so it needs to be readded in mempool so other nodes can store it. this should happend every epoch, each node checks it's register and adds required things to mempool, also nodes should verify the mempool transactions with their local ledge as malicious nodes can propogate malicious mempool, we should always maintain k storing nodes for each file on network. (**HARD**)
