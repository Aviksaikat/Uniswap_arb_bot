use ethers::{prelude::*, utils::parse_ether};
use revm::primitives::Address as rAddress;
use std::str::FromStr;

pub fn get_weth_address() -> Address {
    Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap()
}

pub fn tax_checker_starting_balance() -> U256 {
    parse_ether(420).unwrap()
}

pub fn tax_checker_address() -> rAddress {
    rAddress::from_str("00000000000000000000000000000000F3370000").unwrap()
}

// Holds constant value representing braindance caller
pub fn tax_checker_controller_address() -> rAddress {
    rAddress::from_str("000000000000000000000000000000000420BABE").unwrap()
}

pub fn get_tax_checker_code() -> Bytes {
    "608060405234801561001057600080fd5b506004361061002b5760003560e01c8063dab686f414610030575b600080fd5b61004a60048036038101906100459190610eb9565b610061565b604051610058929190610f2f565b60405180910390f35b600080600085905084600081905550836001819055506000808273ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156100c5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100e99190610fda565b506dffffffffffffffffffffffffffff1691506dffffffffffffffffffffffffffff1691508273ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa158015610159573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061017d9190611042565b73ffffffffffffffffffffffffffffffffffffffff168973ffffffffffffffffffffffffffffffffffffffff16036101ba57808280925081935050505b8873ffffffffffffffffffffffffffffffffffffffff166323b872dd84306064866101e591906110cd565b6040518463ffffffff1660e01b81526004016102039392919061110d565b6020604051808303816000875af1158015610222573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610246919061117c565b508273ffffffffffffffffffffffffffffffffffffffff1663fff6cae96040518163ffffffff1660e01b8152600401600060405180830381600087803b15801561028f57600080fd5b505af11580156102a3573d6000803e3d6000fd5b505050508273ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156102f2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103169190610fda565b826dffffffffffffffffffffffffffff169250816dffffffffffffffffffffffffffff1691505080925081935050506000899050600080600190506000808773ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa15801561039f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103c39190611042565b73ffffffffffffffffffffffffffffffffffffffff168e73ffffffffffffffffffffffffffffffffffffffff160361047957858780975081985050508773ffffffffffffffffffffffffffffffffffffffff16630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561044a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061046e9190611042565b9350600092506104eb565b8773ffffffffffffffffffffffffffffffffffffffff1663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156104c4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104e89190611042565b93505b60008e73ffffffffffffffffffffffffffffffffffffffff166370a08231306040518263ffffffff1660e01b815260040161052691906111a9565b602060405180830381865afa158015610543573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061056791906111d9565b90506000610576828a8a610dc8565b905060008190508773ffffffffffffffffffffffffffffffffffffffff1663a9059cbb8c856040518363ffffffff1660e01b81526004016105b8929190611206565b6020604051808303816000875af11580156105d7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105fb919061117c565b50828a8973ffffffffffffffffffffffffffffffffffffffff166370a082318e6040518263ffffffff1660e01b815260040161063791906111a9565b602060405180830381865afa158015610654573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061067891906111d9565b610682919061122f565b14610719576107168a8973ffffffffffffffffffffffffffffffffffffffff166370a082318e6040518263ffffffff1660e01b81526004016106c491906111a9565b602060405180830381865afa1580156106e1573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061070591906111d9565b61070f919061122f565b8b8b610dc8565b91505b600582610726919061122f565b915085156107b4578a73ffffffffffffffffffffffffffffffffffffffff1663022c0d9f60008430604051806020016040528060008152506040518563ffffffff1660e01b815260040161077d9493929190611341565b600060405180830381600087803b15801561079757600080fd5b505af11580156107ab573d6000803e3d6000fd5b50505050610836565b8a73ffffffffffffffffffffffffffffffffffffffff1663022c0d9f83600030604051806020016040528060008152506040518563ffffffff1660e01b8152600401610803949392919061138d565b600060405180830381600087803b15801561081d57600080fd5b505af1158015610831573d6000803e3d6000fd5b505050505b60008773ffffffffffffffffffffffffffffffffffffffff166370a08231306040518263ffffffff1660e01b815260040161087191906111a9565b602060405180830381865afa15801561088e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108b291906111d9565b826108bd919061122f565b9050600081036108d057600095506108ec565b81612710826108df91906113d9565b6108e991906110cd565b95505b505050508773ffffffffffffffffffffffffffffffffffffffff16630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa15801561093b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061095f9190610fda565b826dffffffffffffffffffffffffffff169250816dffffffffffffffffffffffffffff1691505080975081985050508261099e57858780975081985050505b60008473ffffffffffffffffffffffffffffffffffffffff166370a08231306040518263ffffffff1660e01b81526004016109d991906111a9565b602060405180830381865afa1580156109f6573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a1a91906111d9565b90506000610a2982898b610dc8565b905060008190508673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb8c856040518363ffffffff1660e01b8152600401610a6b929190611206565b6020604051808303816000875af1158015610a8a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610aae919061117c565b5082898873ffffffffffffffffffffffffffffffffffffffff166370a082318e6040518263ffffffff1660e01b8152600401610aea91906111a9565b602060405180830381865afa158015610b07573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b2b91906111d9565b610b35919061122f565b14610bcc57610bc9898873ffffffffffffffffffffffffffffffffffffffff166370a082318e6040518263ffffffff1660e01b8152600401610b7791906111a9565b602060405180830381865afa158015610b94573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bb891906111d9565b610bc2919061122f565b8a8c610dc8565b91505b600582610bd9919061122f565b91508515610c67578a73ffffffffffffffffffffffffffffffffffffffff1663022c0d9f83600030604051806020016040528060008152506040518563ffffffff1660e01b8152600401610c30949392919061138d565b600060405180830381600087803b158015610c4a57600080fd5b505af1158015610c5e573d6000803e3d6000fd5b50505050610ce9565b8a73ffffffffffffffffffffffffffffffffffffffff1663022c0d9f60008430604051806020016040528060008152506040518563ffffffff1660e01b8152600401610cb69493929190611341565b600060405180830381600087803b158015610cd057600080fd5b505af1158015610ce4573d6000803e3d6000fd5b505050505b60008873ffffffffffffffffffffffffffffffffffffffff166370a08231306040518263ffffffff1660e01b8152600401610d2491906111a9565b602060405180830381865afa158015610d41573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d6591906111d9565b82610d70919061122f565b905060008103610d835760009450610d9f565b8161271082610d9291906113d9565b610d9c91906110cd565b94505b5050505081818161ffff1691508061ffff16905099509950505050505050505094509492505050565b60008060015485610dd991906113d9565b905060018160005486610dec91906113d9565b610df69190611433565b8483610e0291906113d9565b610e0c91906110cd565b610e169190611433565b9150509392505050565b600080fd5b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b6000610e5082610e25565b9050919050565b610e6081610e45565b8114610e6b57600080fd5b50565b600081359050610e7d81610e57565b92915050565b6000819050919050565b610e9681610e83565b8114610ea157600080fd5b50565b600081359050610eb381610e8d565b92915050565b60008060008060808587031215610ed357610ed2610e20565b5b6000610ee187828801610e6e565b9450506020610ef287828801610e6e565b9350506040610f0387828801610ea4565b9250506060610f1487828801610ea4565b91505092959194509250565b610f2981610e83565b82525050565b6000604082019050610f446000830185610f20565b610f516020830184610f20565b9392505050565b60006dffffffffffffffffffffffffffff82169050919050565b610f7b81610f58565b8114610f8657600080fd5b50565b600081519050610f9881610f72565b92915050565b600063ffffffff82169050919050565b610fb781610f9e565b8114610fc257600080fd5b50565b600081519050610fd481610fae565b92915050565b600080600060608486031215610ff357610ff2610e20565b5b600061100186828701610f89565b935050602061101286828701610f89565b925050604061102386828701610fc5565b9150509250925092565b60008151905061103c81610e57565b92915050565b60006020828403121561105857611057610e20565b5b60006110668482850161102d565b91505092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60006110d882610e83565b91506110e383610e83565b9250826110f3576110f261106f565b5b828204905092915050565b61110781610e45565b82525050565b600060608201905061112260008301866110fe565b61112f60208301856110fe565b61113c6040830184610f20565b949350505050565b60008115159050919050565b61115981611144565b811461116457600080fd5b50565b60008151905061117681611150565b92915050565b60006020828403121561119257611191610e20565b5b60006111a084828501611167565b91505092915050565b60006020820190506111be60008301846110fe565b92915050565b6000815190506111d381610e8d565b92915050565b6000602082840312156111ef576111ee610e20565b5b60006111fd848285016111c4565b91505092915050565b600060408201905061121b60008301856110fe565b6112286020830184610f20565b9392505050565b600061123a82610e83565b915061124583610e83565b9250828210156112585761125761109e565b5b828203905092915050565b6000819050919050565b6000819050919050565b600061129261128d61128884611263565b61126d565b610e83565b9050919050565b6112a281611277565b82525050565b600081519050919050565b600082825260208201905092915050565b60005b838110156112e25780820151818401526020810190506112c7565b838111156112f1576000848401525b50505050565b6000601f19601f8301169050919050565b6000611313826112a8565b61131d81856112b3565b935061132d8185602086016112c4565b611336816112f7565b840191505092915050565b60006080820190506113566000830187611299565b6113636020830186610f20565b61137060408301856110fe565b81810360608301526113828184611308565b905095945050505050565b60006080820190506113a26000830187610f20565b6113af6020830186611299565b6113bc60408301856110fe565b81810360608301526113ce8184611308565b905095945050505050565b60006113e482610e83565b91506113ef83610e83565b9250817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff04831182151516156114285761142761109e565b5b828202905092915050565b600061143e82610e83565b915061144983610e83565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0382111561147e5761147d61109e565b5b82820190509291505056fea264697066735822122021d7fa1ece54bfb76448f1b8980f1d31b56d7b26c031edf3fa1a7e94e056e1cf64736f6c634300080d0033".parse().unwrap()
}
