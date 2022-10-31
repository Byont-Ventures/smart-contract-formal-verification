# Code report

## Slither

Compiled with solc

Number of lines: 474 (+ 0 in dependencies, + 0 in tests)

Number of assembly lines: 0

Number of contracts: 6 (+ 0 in dependencies, + 0 tests)

Number of optimization issues: 0

Number of informational issues: 5

Number of low issues: 1

Number of medium issues: 1

Number of high issues: 4

ERCs: ERC20

For more information about the detected items see the [Slither documentation](https://github.com/crytic/slither/wiki/Detector-Documentation).

### solc-version

- Impact: `Informational`
- Confidence: `High`

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

2 pragma solidity ^0.8.13;
```

### solc-version

- Impact: `Informational`
- Confidence: `High`

### solc-version

- Impact: `Informational`
- Confidence: `High`

```Solidity
// src/smart-contracts/interfaces/IVeriAuctionTokenForEth.sol

2 pragma solidity ^0.8.13;
```

### low-level-calls

- Impact: `Informational`
- Confidence: `High`

**In Function**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

96     function resignFromAuction() external override {
97         require(commited[msg.sender] > 0, "VeriAuctionTokenForEth (resignFromAuction): must have commited some ETH");
98
99         if (auctionFinalized()) {
100             // Gas savings
101             uint256 _unclaimableTokenAmount = unclaimableTokenAmount;
102             _unclaimableTokenAmount += calculateClaimableAmount();
103             require(
104                 _unclaimableTokenAmount <= amountToDistribute,
105                 "VeriAuctionTokenForEth (resignFromAuction): unclaimable amount would be larger than total tokens to distribute"
106             );
107             unclaimableTokenAmount = _unclaimableTokenAmount;
108         }
109
110         uint256 commitment = commited[msg.sender];
111         require(
112             getEthBalance() >= commitment,
113             "VeriAuctionTokenForEth (resignFromAuction): contract doesn't have enough ETH"
114         );
115
116         // In these three lines the re-entrancy attack happens.
117         (bool transferSuccess, ) = msg.sender.call{value: commitment}("");
118         require(transferSuccess, "VeriAuctionTokenForEth (resignFromAuction): failed to send ETH");
119
120         delete commited[msg.sender];
121     }
```

**Lines of relevance**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

117         (bool transferSuccess, ) = msg.sender.call{value: commitment}("");
```

### naming-convention

- Impact: `Informational`
- Confidence: `High`

### reentrancy-benign

- Impact: `Low`
- Confidence: `Medium`

**In Function**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

57     function depositAuctionTokens() external onlyOwner {
58         // The deployer of this auction contract should have approved the auction
59         // contract to transfer the auction token before deploying this contract.
60         auctionToken.transferFrom(msg.sender, address(this), amountToDistribute);
61         auctionStarted = true;
62     }
```

**Lines of relevance**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

60         auctionToken.transferFrom(msg.sender, address(this), amountToDistribute);
```

**Lines of relevance**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

60         auctionToken.transferFrom(msg.sender, address(this), amountToDistribute);
```

**Lines of relevance**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

61         auctionStarted = true;
```

### divide-before-multiply

- Impact: `Medium`
- Confidence: `Medium`

**In Function**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

196     function calculateClaimableAmount() public view returns (uint256 claimableAmount) {
197         require(auctionFinalized(), "VeriAuctionTokenForEth (calculateClaimableAmount): auction not finalized yet");
198
199         // Note that if we would have used getCurrentPrice() in the contract, then someone
200         // could send eth to the contract using the selfdestruct method and make it so that
201         // no one can claim tokens due to getCurrentPrice() reverting.
202         // Even though a lot of ETH should be sent to the contract, theoratically it is possible.
203         //
204         //  Example:
205         // claimableAmount = (commited[msg.sender] * 10**auctionTokenDecimals) / getCurrentPrice();
206
207         // We know that share will always be <= 1e18
208         // The constructor already takes care of preventing an overflow in (share * amountToDistribute)
209         uint256 share = (commited[msg.sender] * 1e18) / finalEthBalance;
210         claimableAmount = (share * amountToDistribute) / 1e18;
211     }
```

**Lines of relevance**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

209         uint256 share = (commited[msg.sender] * 1e18) / finalEthBalance;
```

**Lines of relevance**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

210         claimableAmount = (share * amountToDistribute) / 1e18;
```

### reentrancy-eth

- Impact: `High`
- Confidence: `Medium`

**In Function**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

96     function resignFromAuction() external override {
97         require(commited[msg.sender] > 0, "VeriAuctionTokenForEth (resignFromAuction): must have commited some ETH");
98
99         if (auctionFinalized()) {
100             // Gas savings
101             uint256 _unclaimableTokenAmount = unclaimableTokenAmount;
102             _unclaimableTokenAmount += calculateClaimableAmount();
103             require(
104                 _unclaimableTokenAmount <= amountToDistribute,
105                 "VeriAuctionTokenForEth (resignFromAuction): unclaimable amount would be larger than total tokens to distribute"
106             );
107             unclaimableTokenAmount = _unclaimableTokenAmount;
108         }
109
110         uint256 commitment = commited[msg.sender];
111         require(
112             getEthBalance() >= commitment,
113             "VeriAuctionTokenForEth (resignFromAuction): contract doesn't have enough ETH"
114         );
115
116         // In these three lines the re-entrancy attack happens.
117         (bool transferSuccess, ) = msg.sender.call{value: commitment}("");
118         require(transferSuccess, "VeriAuctionTokenForEth (resignFromAuction): failed to send ETH");
119
120         delete commited[msg.sender];
121     }
```

**Lines of relevance**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

117         (bool transferSuccess, ) = msg.sender.call{value: commitment}("");
```

**Lines of relevance**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

120         delete commited[msg.sender];
```

### unchecked-transfer

- Impact: `High`
- Confidence: `Medium`

**In Function**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

138     function claimTokens() external override {
139         require(auctionFinalized(), "VeriAuctionTokenForEth (claimTokens): Auction not finalized yet");
140
141         uint256 claimableAmount = calculateClaimableAmount();
142
143         // The writer of this functino might think that using the delete before the transfer prevents
144         // a re-entrancy attack. It actually succeedes in doing that. But it can still be part of another
145         // attack.
146         // These two lines can also be part of the re-entrancy attack in resignFromAuction().
147         // Than not only ETH was stolen, but the attacker would also get its share of the auction tokens.
148         delete commited[msg.sender];
149         auctionToken.transfer(msg.sender, claimableAmount);
150     }
```

**Lines of relevance**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

149         auctionToken.transfer(msg.sender, claimableAmount);
```

### unchecked-transfer

- Impact: `High`
- Confidence: `Medium`

**In Function**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

57     function depositAuctionTokens() external onlyOwner {
58         // The deployer of this auction contract should have approved the auction
59         // contract to transfer the auction token before deploying this contract.
60         auctionToken.transferFrom(msg.sender, address(this), amountToDistribute);
61         auctionStarted = true;
62     }
```

**Lines of relevance**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

60         auctionToken.transferFrom(msg.sender, address(this), amountToDistribute);
```

### unchecked-transfer

- Impact: `High`
- Confidence: `Medium`

**In Function**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

154     function claimUndistributedAuctionTokens() external onlyOwner {
155         uint256 tokensToSend = unclaimableTokenAmount;
156         delete unclaimableTokenAmount;
157
158         // The transfer will fail on insufficient balance
159         auctionToken.transfer(owner(), tokensToSend);
160     }
```

**Lines of relevance**

```Solidity
// src/smart-contracts/VeriAuctionTokenForEth_problems.sol

159         auctionToken.transfer(owner(), tokensToSend);
```

## SMTChecker

=================================================================

Running SMTChecker

=================================================================

Warning: Assertion checker does not yet implement this operator.

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:78:33:

|

78 | type(uint256).max / 10\*\*auctionTokenDecimals >= getEthBalance(),

| ^^^^^^^^^^^^^^^^^^^^^^^^

Warning: Assertion checker does not yet implement this operator.

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:187:33:

    |

187 | type(uint256).max / 10\*\*auctionTokenDecimals >= getEthBalance(),

    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^

Warning: Assertion checker does not yet implement this operator.

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:191:51:

    |

191 | pricePerAuctionToken = (getEthBalance() \* 10\*\*auctionTokenDecimals) / amountToDistribute;

    |                                                   ^^^^^^^^^^^^^^^^^^^^^^^^

Warning: Assertion checker does not yet implement this operator.

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:35:36:

|

35 | \_amountToDistribute >= 10\*\*auctionTokenDecimals,

| ^^^^^^^^^^^^^^^^^^^^^^^^

Warning: CHC: Division by zero happens here.

Counterexample:

auctionToken = 0, amountToDistribute = 0, auctionTokenDecimals = 0, auctionStarted = false, finalEthBalance = 0, unclaimableTokenAmount = 0, \_owner = 0x0

veriTokenAddress = 0x0

\_amountToDistribute = 0

Transaction trace:

VeriAuctionTokenForEth_problems.constructor(0x0, 0)

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:39:13:

|

39 | type(uint256).max / \_amountToDistribute >= 1e18,

| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Warning: CHC: Division by zero happens here.

Counterexample:

auctionToken = 0, amountToDistribute = 1, auctionTokenDecimals = 0, auctionStarted = true, finalEthBalance = 0, unclaimableTokenAmount = 0, \_owner = 0x0

Transaction trace:

VeriAuctionTokenForEth_problems.constructor(0x0, 1)

State: auctionToken = 0, amountToDistribute = 1, auctionTokenDecimals = 0, auctionStarted = false, finalEthBalance = 0, unclaimableTokenAmount = 0, \_owner = 0x0

VeriAuctionTokenForEth_problems.depositAuctionTokens(){ msg.sender: 0x0 }

    Ownable._checkOwner() -- internal call

        Ownable.owner() -- internal call

        Context._msgSender() -- internal call

    auctionToken.transferFrom(msg.sender, address(this), amountToDistribute) -- untrusted external call

State: auctionToken = 0, amountToDistribute = 1, auctionTokenDecimals = 0, auctionStarted = true, finalEthBalance = 0, unclaimableTokenAmount = 0, \_owner = 0x0

VeriAuctionTokenForEth_problems.depositAuctionTokens(){ msg.sender: 0x0 }

    Ownable._checkOwner() -- internal call

        Ownable.owner() -- internal call

        Context._msgSender() -- internal call

    auctionToken.transferFrom(msg.sender, address(this), amountToDistribute) -- untrusted external call, synthesized as:

        VeriAuctionTokenForEth_problems.commitEth(){ msg.sender: 0x0, msg.value: 1 } -- reentrant call

            VeriAuctionTokenForEth_problems.auctionFinalized() -- internal call

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:78:13:

|

78 | type(uint256).max / 10\*\*auctionTokenDecimals >= getEthBalance(),

| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Warning: CHC: Division by zero happens here.

Counterexample:

auctionToken = 0, amountToDistribute = 1, auctionTokenDecimals = 0, auctionStarted = false, finalEthBalance = 0, unclaimableTokenAmount = 0, \_owner = 0x0

pricePerAuctionToken = 0

Transaction trace:

VeriAuctionTokenForEth_problems.constructor(0x0, 1)

State: auctionToken = 0, amountToDistribute = 1, auctionTokenDecimals = 0, auctionStarted = false, finalEthBalance = 0, unclaimableTokenAmount = 0, \_owner = 0x0

VeriAuctionTokenForEth_problems.getCurrentPrice()

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:187:13:

    |

187 | type(uint256).max / 10\*\*auctionTokenDecimals >= getEthBalance(),

    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Warning: CHC: Overflow (resulting value larger than 2\*\*256 - 1) happens here.

Counterexample:

auctionToken = 0, amountToDistribute = 1, auctionTokenDecimals = 0, auctionStarted = false, finalEthBalance = 0, unclaimableTokenAmount = 0, \_owner = 0x0

pricePerAuctionToken = 0

Transaction trace:

VeriAuctionTokenForEth_problems.constructor(0x0, 1)

State: auctionToken = 0, amountToDistribute = 1, auctionTokenDecimals = 0, auctionStarted = false, finalEthBalance = 0, unclaimableTokenAmount = 0, \_owner = 0x0

VeriAuctionTokenForEth_problems.getCurrentPrice()

    VeriAuctionTokenForEth_problems.getEthBalance() -- internal call

    VeriAuctionTokenForEth_problems.getEthBalance() -- internal call

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:191:33:

    |

191 | pricePerAuctionToken = (getEthBalance() \* 10\*\*auctionTokenDecimals) / amountToDistribute;

    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Warning: CHC: 9 verification condition(s) could not be proved. Enable the model checker option "show unproved" to see all of them. Consider choosing a specific contract to be verified in order to reduce the solving problems. Consider increasing the timeout per query.

Warning: BMC: Overflow (resulting value larger than 2\*\*256 - 1) happens here.

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:82:41:

|

82 | type(uint256).max / 1e18 >= commited[msg.sender] + msg.value,

| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Note: Counterexample:

= 1

<result> = 2\*\*256

\_owner = 0

address(this).balance = 1

amountToDistribute = 0

auctionStarted = true

auctionToken = 0

auctionTokenDecimals = 0

commited[msg.sender] = 2\*\*256 - 1

finalEthBalance = 0

finialized = false

this = 0

unclaimableTokenAmount = 0

Note: Callstack:

Note:

Note that array aliasing is not supported, therefore all mapping information is erased after a mapping local variable/parameter is assigned.

You can re-introduce information using require().

Warning: BMC: Overflow (resulting value larger than 2\*\*256 - 1) happens here.

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:209:26:

    |

209 | uint256 share = (commited[msg.sender] \* 1e18) / finalEthBalance;

    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^

Note: Counterexample:

<result> = 0x0100000000000000000000000000000000000000000000000005C5e69957480000

\_owner = 0

\_unclaimableTokenAmount = 0

amountToDistribute = 0

auctionToken = 0

auctionTokenDecimals = 0

claimableAmount = 0

commited[msg.sender] = 0x12725Dd1d243ABa0e75FE645cc4873f9e65AFE688c928E1f22

commitment = 0

finalEthBalance = 1

finialized = true

share = 0

transferSuccess = false

unclaimableTokenAmount = 0

Note: Callstack:

Note:

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:102:40:

    |

102 | \_unclaimableTokenAmount += calculateClaimableAmount();

    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^

Note:

Note that array aliasing is not supported, therefore all mapping information is erased after a mapping local variable/parameter is assigned.

You can re-introduce information using require().

Note that external function calls are not inlined, even if the source code of the function is available. This is due to the possibility that the actual called contract has the same ABI but implements the function differently.

Warning: BMC: Overflow (resulting value larger than 2\*\*256 - 1) happens here.

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:210:28:

    |

210 | claimableAmount = (share \* amountToDistribute) / 1e18;

    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^

Note: Counterexample:

<result> = 2\*\*256

\_owner = 0

\_unclaimableTokenAmount = 0

amountToDistribute = 0x80 \* 2\*\*248

auctionToken = 0

auctionTokenDecimals = 0

claimableAmount = 0

commited[msg.sender] = 1

commitment = 0

finalEthBalance = 0x06F05b59D3B20000

finialized = true

share = 2

transferSuccess = false

unclaimableTokenAmount = 0

Note: Callstack:

Note:

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:102:40:

    |

102 | \_unclaimableTokenAmount += calculateClaimableAmount();

    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^

Note:

Note that array aliasing is not supported, therefore all mapping information is erased after a mapping local variable/parameter is assigned.

You can re-introduce information using require().

Note that external function calls are not inlined, even if the source code of the function is available. This is due to the possibility that the actual called contract has the same ABI but implements the function differently.

Warning: BMC: Overflow (resulting value larger than 2\*\*256 - 1) happens here.

--> src/smart-contracts/VeriAuctionTokenForEth_problems.sol:102:13:

    |

102 | \_unclaimableTokenAmount += calculateClaimableAmount();

    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Note: Counterexample:

<result> = 2\*\*256

\_owner = 0

\_unclaimableTokenAmount = 0xFFFFffffFFFFffed8DA22e2dBC545f18A019ba33B78C0619A50197736D71e0df

amountToDistribute = 2\*\*256 - 1

auctionToken = 0

auctionTokenDecimals = 2\*\*256 - 1

claimableAmount = 0x12725Dd1d243ABa0e75FE645cc4873f9e65AFE688c928E1f21

commited[msg.sender] = 0x12725Dd1d243ABa0e75FE645cc4873f9e65AFE688c928E1f21

commitment = 0

finalEthBalance = 0xFFFFffffFFFFffffFFFFffffFFFFffffFFFFffffFFFFffffF7E52fe5AFE40000

finialized = true

share = 1

transferSuccess = false

unclaimableTokenAmount = 0xFFFFffffFFFFffed8DA22e2dBC545f18A019ba33B78C0619A50197736D71e0df

Note: Callstack:

Note:

Note that array aliasing is not supported, therefore all mapping information is erased after a mapping local variable/parameter is assigned.

You can re-introduce information using require().

Note that external function calls are not inlined, even if the source code of the function is available. This is due to the possibility that the actual called contract has the same ABI but implements the function differently.

## Mythril

=================================================================

Flatten the contract to be verified

=================================================================

Flattened file written at /prj/./security-scans/flattened/VeriAuctionTokenForEth_problems-flat.sol

=================================================================

Run Mythril

=================================================================

mythril.mythril.mythril_config [INFO]: Creating mythril data directory

mythril.mythril.mythril_config [INFO]: No config file found. Creating default: /root/.mythril/config.ini

mythril.mythril.mythril_config [INFO]: Infura key not provided, so onchain access is disabled. Use --infura-id <INFURA_ID> or set it in the environment variable INFURA_ID or in the ~/.mythril/config.ini file

mythril.support.signatures [INFO]: Using signature database at /root/.mythril/signatures.db

solcx [INFO]: Downloading from https://solc-bin.ethereum.org/linux-amd64/solc-linux-amd64-v0.8.17+commit.8df45f5f

solcx [INFO]: solc 0.8.17 successfully installed at: /root/.solcx/solc-v0.8.17

solcx [INFO]: Using solc version 0.8.17

mythril.laser.ethereum.svm [INFO]: LASER EVM initialized with dynamic loader: <mythril.support.loader.DynLoader object at 0x7f018a205790>

mythril.laser.ethereum.strategy.extensions.bounded_loops [INFO]: Loaded search strategy extension: Loop bounds (limit = 3)

mythril.laser.plugin.loader [INFO]: Loading laser plugin: coverage

mythril.laser.plugin.loader [INFO]: Loading laser plugin: mutation-pruner

mythril.laser.plugin.loader [INFO]: Loading laser plugin: call-depth-limit

mythril.laser.plugin.loader [INFO]: Loading laser plugin: instruction-profiler

mythril.laser.plugin.loader [INFO]: Loading laser plugin: dependency-pruner

mythril.laser.plugin.loader [INFO]: Instrumenting symbolic vm with plugin: coverage

mythril.laser.plugin.loader [INFO]: Instrumenting symbolic vm with plugin: mutation-pruner

mythril.laser.plugin.loader [INFO]: Instrumenting symbolic vm with plugin: call-depth-limit

mythril.laser.plugin.loader [INFO]: Instrumenting symbolic vm with plugin: instruction-profiler

mythril.laser.plugin.loader [INFO]: Instrumenting symbolic vm with plugin: dependency-pruner

mythril.laser.ethereum.svm [INFO]: Starting contract creation transaction

mythril.laser.ethereum.svm [INFO]: Finished contract creation, found 0 open states

mythril.laser.ethereum.svm [WARNING]: No contract was created during the execution of contract creation Increase the resources for creation execution (--max-depth or --create-timeout) Check whether the bytecode is indeed the creation code, otherwise use the --bin-runtime flag

mythril.laser.ethereum.svm [INFO]: Finished symbolic execution

mythril.laser.plugin.plugins.coverage.coverage_plugin [INFO]: Achieved 18.11% coverage for code: 60e06040523480156200001157600080fd5b506040516200296c3803806200296c833981810160405281019062000037919062000385565b620000576200004b6200021460201b60201c565b6200021c60201b60201c565b8173ffffffffffffffffffffffffffffffffffffffff1660808173ffffffffffffffffffffffffffffffffffffffff168152505060805173ffffffffffffffffffffffffffffffffffffffff1663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000d9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000ff91906200040a565b60ff1660c0818152505060c051600a6200011a9190620005bf565b8110156200015f576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016200015690620006bd565b60405180910390fd5b670de0b6b3a7640000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6200019691906200070e565b1015620001da576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401620001d19062000808565b60405180910390fd5b8060a0818152505060008060146101000a81548160ff0219169083151502179055506000600181905550600060038190555050506200082a565b600033905090565b60008060009054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050816000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b600080fd5b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b60006200031282620002e5565b9050919050565b620003248162000305565b81146200033057600080fd5b50565b600081519050620003448162000319565b92915050565b6000819050919050565b6200035f816200034a565b81146200036b57600080fd5b50565b6000815190506200037f8162000354565b92915050565b600080604083850312156200039f576200039e620002e0565b5b6000620003af8582860162000333565b9250506020620003c2858286016200036e565b9150509250929050565b600060ff82169050919050565b620003e481620003cc565b8114620003f057600080fd5b50565b6000815190506200040481620003d9565b92915050565b600060208284031215620004235762000422620002e0565b5b60006200043384828501620003f3565b91505092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60008160011c9050919050565b6000808291508390505b6001851115620004ca57808604811115620004a257620004a16200043c565b5b6001851615620004b25780820291505b8081029050620004c2856200046b565b945062000482565b94509492505050565b600082620004e55760019050620005b8565b81620004f55760009050620005b8565b81600181146200050e576002811462000519576200054f565b6001915050620005b8565b60ff8411156200052e576200052d6200043c565b5b8360020a9150848211156200054857620005476200043c565b5b50620005b8565b5060208310610133831016604e8410600b8410161715620005895782820a9050838111156200058357620005826200043c565b5b620005b8565b62000598848484600162000478565b92509050818404811115620005b257620005b16200043c565b5b81810290505b9392505050565b6000620005cc826200034a565b9150620005d9836200034a565b9250620006087fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8484620004d3565b905092915050565b600082825260208201905092915050565b7f5665726941756374696f6e546f6b656e466f724574682028636f6e737472756360008201527f746f72293a206d7573742064697374726962757465206174206c65617374206f60208201527f6e652077686f6c6520746f6b656e000000000000000000000000000000000000604082015250565b6000620006a5604e8362000610565b9150620006b28262000621565b606082019050919050565b60006020820190508181036000830152620006d88162000696565b9050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b60006200071b826200034a565b915062000728836200034a565b9250826200073b576200073a620006df565b5b828204905092915050565b7f5665726941756374696f6e546f6b656e466f724574682028636f6e737472756360008201527f746f72293a2073697a65206966205f616d6f756e74546f44697374726962757460208201527f6520636f756c6420636175736520616e206f766572666c6f77207768656e206360408201527f6c61696d696e6720746f6b656e73000000000000000000000000000000000000606082015250565b6000620007f0606e8362000610565b9150620007fd8262000746565b608082019050919050565b600060208201905081810360008301526200082381620007e1565b9050919050565b60805160a05160c0516120cc620008a0600039600081816104cc01528181610db90152610e7301526000818161093201528181610a2d01528181610cad01528181610d5001528181610e520152610ec10152600081816103410152818161077301528181610c430152610c6f01526120cc6000f3fe6080604052600436106100e85760003560e01c8063715018a61161008a578063b1111f1911610059578063b1111f1914610241578063eb91d37e14610258578063ed4460ef14610283578063f2fde38b146102ae576100e8565b8063715018a6146101bd57806388463329146101d45780638da5cb5b146101eb57806399fdb32014610216576100e8565b806345dc13f3116100c657806345dc13f31461015a57806348c54b9d146101645780634bb278f31461017b5780635054f45814610192576100e8565b80630bd8a1d0146100ed5780632a79c611146101185780633ff62dd114610143575b600080fd5b3480156100f957600080fd5b506101026102d7565b60405161010f91906110d3565b60405180910390f35b34801561012457600080fd5b5061012d6102e3565b60405161013a9190611107565b60405180910390f35b34801561014f57600080fd5b5061015861032a565b005b6101626103e8565b005b34801561017057600080fd5b506101796106db565b005b34801561018757600080fd5b50610190610813565b005b34801561019e57600080fd5b506101a761087a565b6040516101b49190611107565b60405180910390f35b3480156101c957600080fd5b506101d261096c565b005b3480156101e057600080fd5b506101e9610980565b005b3480156101f757600080fd5b50610200610c18565b60405161020d9190611163565b60405180910390f35b34801561022257600080fd5b5061022b610c41565b60405161023891906111dd565b60405180910390f35b34801561024d57600080fd5b50610256610c65565b005b34801561026457600080fd5b5061026d610d4b565b60405161027a9190611107565b60405180910390f35b34801561028f57600080fd5b50610298610ebf565b6040516102a59190611107565b60405180910390f35b3480156102ba57600080fd5b506102d560048036038101906102d09190611229565b610ee3565b005b60008060015411905090565b6000600260003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054905090565b610332610f66565b600060035490506003600090557f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a9059cbb610383610c18565b836040518363ffffffff1660e01b81526004016103a1929190611256565b6020604051808303816000875af11580156103c0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103e491906112ab565b5050565b600060149054906101000a900460ff16610437576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161042e9061135b565b60405180910390fd5b61043f6102d7565b1561047f576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610476906113ed565b60405180910390fd5b600034116104c2576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104b9906114a5565b60405180910390fd5b6104ca610fe4565b7f0000000000000000000000000000000000000000000000000000000000000000600a6104f79190611627565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61052291906116a1565b1015610563576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161055a9061176a565b60405180910390fd5b34600260003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546105ae919061178a565b670de0b6b3a76400007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6105e291906116a1565b1015610623576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161061a90611856565b60405180910390fd5b34600260003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000206000828254610672919061178a565b925050819055503373ffffffffffffffffffffffffffffffffffffffff167f9cff2c0a4d5be51d8a883d8301a9be115ec08ce14e5fe70c21c3f5f507cca33f346106ba610fe4565b6106c2610d4b565b6040516106d193929190611876565b60405180910390a2565b6106e36102d7565b610722576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016107199061191f565b60405180910390fd5b600061072c61087a565b9050600260003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600090557f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a9059cbb33836040518363ffffffff1660e01b81526004016107cc929190611256565b6020604051808303816000875af11580156107eb573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061080f91906112ab565b5050565b61081b610f66565b600060149054906101000a900460ff1661086a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610861906119b1565b60405180910390fd5b610872610fe4565b600181905550565b60006108846102d7565b6108c3576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016108ba90611a69565b60405180910390fd5b6000600154670de0b6b3a7640000600260003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205461091b9190611a89565b61092591906116a1565b9050670de0b6b3a76400007f00000000000000000000000000000000000000000000000000000000000000008261095c9190611a89565b61096691906116a1565b91505090565b610974610f66565b61097e6000610fec565b565b6000600260003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205411610a02576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016109f990611b63565b60405180910390fd5b610a0a6102d7565b15610a975760006003549050610a1e61087a565b81610a29919061178a565b90507f0000000000000000000000000000000000000000000000000000000000000000811115610a8e576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a8590611c41565b60405180910390fd5b80600381905550505b6000600260003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054905080610ae4610fe4565b1015610b25576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610b1c90611cf9565b60405180910390fd5b60003373ffffffffffffffffffffffffffffffffffffffff1682604051610b4b90611d4a565b60006040518083038185875af1925050503d8060008114610b88576040519150601f19603f3d011682016040523d82523d6000602084013e610b8d565b606091505b5050905080610bd1576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610bc890611dd1565b60405180910390fd5b600260003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600090555050565b60008060009054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b7f000000000000000000000000000000000000000000000000000000000000000081565b610c6d610f66565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166323b872dd33307f00000000000000000000000000000000000000000000000000000000000000006040518463ffffffff1660e01b8152600401610cea93929190611df1565b6020604051808303816000875af1158015610d09573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d2d91906112ab565b506001600060146101000a81548160ff021916908315150217905550565b6000807f000000000000000000000000000000000000000000000000000000000000000011610daf576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610da690611ec0565b60405180910390fd5b610db7610fe4565b7f0000000000000000000000000000000000000000000000000000000000000000600a610de49190611627565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff610e0f91906116a1565b1015610e50576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e4790611f78565b60405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000007f0000000000000000000000000000000000000000000000000000000000000000600a610e9e9190611627565b610ea6610fe4565b610eb09190611a89565b610eba91906116a1565b905090565b7f000000000000000000000000000000000000000000000000000000000000000081565b610eeb610f66565b600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610f5a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610f519061200a565b60405180910390fd5b610f6381610fec565b50565b610f6e6110b0565b73ffffffffffffffffffffffffffffffffffffffff16610f8c610c18565b73ffffffffffffffffffffffffffffffffffffffff1614610fe2576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610fd990612076565b60405180910390fd5b565b600047905090565b60008060009054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050816000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b600033905090565b60008115159050919050565b6110cd816110b8565b82525050565b60006020820190506110e860008301846110c4565b92915050565b6000819050919050565b611101816110ee565b82525050565b600060208201905061111c60008301846110f8565b92915050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b600061114d82611122565b9050919050565b61115d81611142565b82525050565b60006020820190506111786000830184611154565b92915050565b6000819050919050565b60006111a361119e61119984611122565b61117e565b611122565b9050919050565b60006111b582611188565b9050919050565b60006111c7826111aa565b9050919050565b6111d7816111bc565b82525050565b60006020820190506111f260008301846111ce565b92915050565b600080fd5b61120681611142565b811461121157600080fd5b50565b600081359050611223816111fd565b92915050565b60006020828403121561123f5761123e6111f8565b5b600061124d84828501611214565b91505092915050565b600060408201905061126b6000830185611154565b61127860208301846110f8565b9392505050565b611288816110b8565b811461129357600080fd5b50565b6000815190506112a58161127f565b92915050565b6000602082840312156112c1576112c06111f8565b5b60006112cf84828501611296565b91505092915050565b600082825260208201905092915050565b7f5665726941756374696f6e546f6b656e466f724574682028636f6d6d6974457460008201527f68293a2041756374696f6e206e6f742073746172746564000000000000000000602082015250565b60006113456037836112d8565b9150611350826112e9565b604082019050919050565b6000602082019050818103600083015261137481611338565b9050919050565b7f5665726941756374696f6e546f6b656e466f724574682028636f6d6d6974457460008201527f68293a2041756374696f6e2069732066696e616c697a65640000000000000000602082015250565b60006113d76038836112d8565b91506113e28261137b565b604082019050919050565b60006020820190508181036000830152611406816113ca565b9050919050565b7f5665726941756374696f6e546f6b656e466f724574682028636f6d6d6974457460008201527f68293a20636f6d6d69746d656e74206d7573742062652067726561746572207460208201527f68616e2030000000000000000000000000000000000000000000000000000000604082015250565b600061148f6045836112d8565b915061149a8261140d565b606082019050919050565b600060208201905081810360008301526114be81611482565b9050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60008160011c9050919050565b6000808291508390505b600185111561154b57808604811115611527576115266114c5565b5b60018516156115365780820291505b8081029050611544856114f4565b945061150b565b94509492505050565b6000826115645760019050611620565b816115725760009050611620565b81600181146115885760028114611592576115c1565b6001915050611620565b60ff8411156115a4576115a36114c5565b5b8360020a9150848211156115bb576115ba6114c5565b5b50611620565b5060208310610133831016604e8410600b84101617156115f65782820a9050838111156115f1576115f06114c5565b5b611620565b6116038484846001611501565b9250905081840481111561161a576116196114c5565b5b81810290505b9392505050565b6000611632826110ee565b915061163d836110ee565b925061166a7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8484611554565b905092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b60006116ac826110ee565b91506116b7836110ee565b9250826116c7576116c6611672565b5b828204905092915050565b7f5665726941756374696f6e546f6b656e466f724574682028636f6d6d6974457460008201527f68293a20776f756c6420726573756c7420696e20746f6f206d7563682045544860208201527f20696e207468652061756374696f6e0000000000000000000000000000000000604082015250565b6000611754604f836112d8565b915061175f826116d2565b606082019050919050565b6000602082019050818103600083015261178381611747565b9050919050565b6000611795826110ee565b91506117a0836110ee565b92508282019050808211156117b8576117b76114c5565b5b92915050565b7f5665726941756374696f6e546f6b656e466f724574682028636f6d6d6974457460008201527f68293a20776f756c6420726573756c74206973206120746f6f206c617267652060208201527f636f6d6d69746d656e7400000000000000000000000000000000000000000000604082015250565b6000611840604a836112d8565b915061184b826117be565b606082019050919050565b6000602082019050818103600083015261186f81611833565b9050919050565b600060608201905061188b60008301866110f8565b61189860208301856110f8565b6118a560408301846110f8565b949350505050565b7f5665726941756374696f6e546f6b656e466f724574682028636c61696d546f6b60008201527f656e73293a2041756374696f6e206e6f742066696e616c697a65642079657400602082015250565b6000611909603f836112d8565b9150611914826118ad565b604082019050919050565b60006020820190508181036000830152611938816118fc565b9050919050565b7f5665726941756374696f6e546f6b656e466f72457468202866696e616c697a6560008201527f293a2041756374696f6e206e6f74207374617274656400000000000000000000602082015250565b600061199b6036836112d8565b91506119a68261193f565b604082019050919050565b600060208201905081810360008301526119ca8161198e565b9050919050565b7f5665726941756374696f6e546f6b656e466f72457468202863616c63756c617460008201527f65436c61696d61626c65416d6f756e74293a2061756374696f6e206e6f74206660208201527f696e616c697a6564207965740000000000000000000000000000000000000000604082015250565b6000611a53604c836112d8565b9150611a5e826119d1565b606082019050919050565b60006020820190508181036000830152611a8281611a46565b9050919050565b6000611a94826110ee565b9150611a9f836110ee565b9250828202611aad816110ee565b91508282048414831517611ac457611ac36114c5565b5b5092915050565b7f5665726941756374696f6e546f6b656e466f72457468202872657369676e467260008201527f6f6d41756374696f6e293a206d757374206861766520636f6d6d69746564207360208201527f6f6d652045544800000000000000000000000000000000000000000000000000604082015250565b6000611b4d6047836112d8565b9150611b5882611acb565b606082019050919050565b60006020820190508181036000830152611b7c81611b40565b9050919050565b7f5665726941756374696f6e546f6b656e466f72457468202872657369676e467260008201527f6f6d41756374696f6e293a20756e636c61696d61626c6520616d6f756e74207760208201527f6f756c64206265206c6172676572207468616e20746f74616c20746f6b656e7360408201527f20746f2064697374726962757465000000000000000000000000000000000000606082015250565b6000611c2b606e836112d8565b9150611c3682611b83565b608082019050919050565b60006020820190508181036000830152611c5a81611c1e565b9050919050565b7f5665726941756374696f6e546f6b656e466f72457468202872657369676e467260008201527f6f6d41756374696f6e293a20636f6e747261637420646f65736e27742068617660208201527f6520656e6f756768204554480000000000000000000000000000000000000000604082015250565b6000611ce3604c836112d8565b9150611cee82611c61565b606082019050919050565b60006020820190508181036000830152611d1281611cd6565b9050919050565b600081905092915050565b50565b6000611d34600083611d19565b9150611d3f82611d24565b600082019050919050565b6000611d5582611d27565b9150819050919050565b7f5665726941756374696f6e546f6b656e466f72457468202872657369676e467260008201527f6f6d41756374696f6e293a206661696c656420746f2073656e64204554480000602082015250565b6000611dbb603e836112d8565b9150611dc682611d5f565b604082019050919050565b60006020820190508181036000830152611dea81611dae565b9050919050565b6000606082019050611e066000830186611154565b611e136020830185611154565b611e2060408301846110f8565b949350505050565b7f5665726941756374696f6e546f6b656e466f724574682028676574437572726560008201527f6e745072696365293a206e6f20746f6b656e7320746f2064697374726962757460208201527f6500000000000000000000000000000000000000000000000000000000000000604082015250565b6000611eaa6041836112d8565b9150611eb582611e28565b606082019050919050565b60006020820190508181036000830152611ed981611e9d565b9050919050565b7f5665726941756374696f6e546f6b656e466f724574682028676574437572726560008201527f6e745072696365293a20746f6f206d7563682045544820696e2074686520636f60208201527f6e74726163740000000000000000000000000000000000000000000000000000604082015250565b6000611f626046836112d8565b9150611f6d82611ee0565b606082019050919050565b60006020820190508181036000830152611f9181611f55565b9050919050565b7f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160008201527f6464726573730000000000000000000000000000000000000000000000000000602082015250565b6000611ff46026836112d8565b9150611fff82611f98565b604082019050919050565b6000602082019050818103600083015261202381611fe7565b9050919050565b7f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572600082015250565b60006120606020836112d8565b915061206b8261202a565b602082019050919050565b6000602082019050818103600083015261208f81612053565b905091905056fea2646970667358221220f5f5ec4eb3ff36b062abd404955d7bcc2ceeacecb0e3295d8c231bcb36b1796964736f6c63430008110033

mythril.laser.plugin.plugins.instruction_profiler [INFO]: Total: 19.43353533744812 s

[ADD ] 0.2301 %, nr 43, total 0.0447 s, avg 0.0010 s, min 0.0009 s, max 0.0013 s

[AND ] 0.4092 %, nr 78, total 0.0795 s, avg 0.0010 s, min 0.0009 s, max 0.0024 s

[CALLER ] 0.0045 %, nr 1, total 0.0009 s, avg 0.0009 s, min 0.0009 s, max 0.0009 s

[CALLVALUE ] 0.0048 %, nr 1, total 0.0009 s, avg 0.0009 s, min 0.0009 s, max 0.0009 s

[CODECOPY ] 1.3134 %, nr 1, total 0.2552 s, avg 0.2552 s, min 0.2552 s, max 0.2552 s

[CODESIZE ] 0.0050 %, nr 1, total 0.0010 s, avg 0.0010 s, min 0.0010 s, max 0.0010 s

[DIV ] 1.1559 %, nr 180, total 0.2246 s, avg 0.0012 s, min 0.0012 s, max 0.0028 s

[DUP1 ] 1.7080 %, nr 335, total 0.3319 s, avg 0.0010 s, min 0.0009 s, max 0.0025 s

[DUP2 ] 2.7435 %, nr 540, total 0.5332 s, avg 0.0010 s, min 0.0009 s, max 0.0027 s

[DUP3 ] 0.6238 %, nr 123, total 0.1212 s, avg 0.0010 s, min 0.0009 s, max 0.0024 s

[DUP4 ] 0.1098 %, nr 19, total 0.0213 s, avg 0.0011 s, min 0.0009 s, max 0.0040 s

[DUP5 ] 1.0936 %, nr 59, total 0.2125 s, avg 0.0036 s, min 0.0009 s, max 0.1537 s

[DUP6 ] 1.6363 %, nr 320, total 0.3180 s, avg 0.0010 s, min 0.0009 s, max 0.0026 s

[DUP7 ] 0.6766 %, nr 130, total 0.1315 s, avg 0.0010 s, min 0.0009 s, max 0.0028 s

[EQ ] 0.0297 %, nr 5, total 0.0058 s, avg 0.0012 s, min 0.0011 s, max 0.0013 s

[EXP ] 0.0212 %, nr 3, total 0.0041 s, avg 0.0014 s, min 0.0013 s, max 0.0014 s

[GAS ] 0.0056 %, nr 1, total 0.0011 s, avg 0.0011 s, min 0.0011 s, max 0.0011 s

[GT ] 2.7977 %, nr 305, total 0.5437 s, avg 0.0018 s, min 0.0011 s, max 0.1624 s

[ISZERO ] 3.9689 %, nr 378, total 0.7713 s, avg 0.0020 s, min 0.0011 s, max 0.1378 s

[JUMP ] 7.8262 %, nr 629, total 1.5209 s, avg 0.0024 s, min 0.0017 s, max 0.1179 s

[JUMPDEST ] 6.6864 %, nr 884, total 1.2994 s, avg 0.0015 s, min 0.0009 s, max 0.1961 s

[JUMPI ] 6.3824 %, nr 386, total 1.2403 s, avg 0.0032 s, min 0.0020 s, max 0.1436 s

[LOG3 ] 0.0049 %, nr 1, total 0.0009 s, avg 0.0009 s, min 0.0009 s, max 0.0009 s

[LT ] 0.0501 %, nr 8, total 0.0097 s, avg 0.0012 s, min 0.0011 s, max 0.0014 s

[MLOAD ] 2.9229 %, nr 19, total 0.5680 s, avg 0.0299 s, min 0.0179 s, max 0.0360 s

[MSTORE ] 30.4365 %, nr 260, total 5.9149 s, avg 0.0227 s, min 0.0111 s, max 0.0273 s

[MUL ] 1.6099 %, nr 191, total 0.3129 s, avg 0.0016 s, min 0.0010 s, max 0.1089 s

[NOT ] 0.0098 %, nr 2, total 0.0019 s, avg 0.0010 s, min 0.0009 s, max 0.0010 s

[OR ] 0.0099 %, nr 2, total 0.0019 s, avg 0.0010 s, min 0.0009 s, max 0.0010 s

[POP ] 5.1251 %, nr 993, total 0.9960 s, avg 0.0010 s, min 0.0009 s, max 0.0039 s

[PUSH1 ] 6.7860 %, nr 1138, total 1.3188 s, avg 0.0012 s, min 0.0009 s, max 0.1537 s

[PUSH2 ] 0.0151 %, nr 3, total 0.0029 s, avg 0.0010 s, min 0.0009 s, max 0.0011 s

[PUSH20 ] 0.0456 %, nr 9, total 0.0089 s, avg 0.0010 s, min 0.0009 s, max 0.0011 s

[PUSH3 ] 5.6634 %, nr 1081, total 1.1006 s, avg 0.0010 s, min 0.0009 s, max 0.0033 s

[PUSH32 ] 0.7217 %, nr 136, total 0.1402 s, avg 0.0010 s, min 0.0010 s, max 0.0026 s

[PUSH4 ] 0.0105 %, nr 2, total 0.0020 s, avg 0.0010 s, min 0.0010 s, max 0.0011 s

[PUSH8 ] 0.0103 %, nr 2, total 0.0020 s, avg 0.0010 s, min 0.0010 s, max 0.0010 s

[RETURNDATACOPY] 0.0053 %, nr 1, total 0.0010 s, avg 0.0010 s, min 0.0010 s, max 0.0010 s

[RETURNDATASIZE] 0.0161 %, nr 3, total 0.0031 s, avg 0.0010 s, min 0.0009 s, max 0.0012 s

[SHL ] 0.0149 %, nr 3, total 0.0029 s, avg 0.0010 s, min 0.0009 s, max 0.0010 s

[SHR ] 0.7217 %, nr 128, total 0.1403 s, avg 0.0011 s, min 0.0010 s, max 0.0026 s

[SLOAD ] 0.0122 %, nr 2, total 0.0024 s, avg 0.0012 s, min 0.0012 s, max 0.0012 s

[SLT ] 0.0117 %, nr 2, total 0.0023 s, avg 0.0011 s, min 0.0011 s, max 0.0012 s

[SSTORE ] 0.0053 %, nr 1, total 0.0010 s, avg 0.0010 s, min 0.0010 s, max 0.0010 s

[STATICCALL ] 0.2805 %, nr 1, total 0.0545 s, avg 0.0545 s, min 0.0545 s, max 0.0545 s

[SUB ] 0.0688 %, nr 13, total 0.0134 s, avg 0.0010 s, min 0.0010 s, max 0.0012 s

[SWAP1 ] 2.6260 %, nr 507, total 0.5103 s, avg 0.0010 s, min 0.0009 s, max 0.0036 s

[SWAP2 ] 1.6265 %, nr 239, total 0.3161 s, avg 0.0013 s, min 0.0009 s, max 0.0749 s

[SWAP3 ] 0.5983 %, nr 120, total 0.1163 s, avg 0.0010 s, min 0.0009 s, max 0.0012 s

[SWAP4 ] 0.0096 %, nr 2, total 0.0019 s, avg 0.0009 s, min 0.0009 s, max 0.0009 s

[SWAP5 ] 1.1488 %, nr 226, total 0.2232 s, avg 0.0010 s, min 0.0009 s, max 0.0024 s

mythril.analysis.security [INFO]: Starting analysis

mythril.mythril.mythril_analyzer [INFO]: Solver statistics:

Query count: 407

Solver time: 50.4525671005249

The analysis was completed successfully. No issues were detected.