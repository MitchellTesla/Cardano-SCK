# Cardano
This repository is for testing and development of solidity smart-contracts on the Cardano KEVM and Blockchain.

It includes a Cardano Node.
The core component used to participate in the Cardano decentralized blockchain.
This is an experimental repository under active research and development. Notice: (Some Missing files) 
Not currently Fully-Functional and a work in progress.

<p align="center">
  <img src="Cardanowallpaper4k_mTesla.jpg" alt="demo" />
</p>

Just like Bitcoin, Cardano uses the UTXO (unspent transaction output) model where the inputs are unspent outputs from previous transactions. However, the network has expanded upon this to employ an extended UTXO model (EUTXO) which offers unique advantages over other accounting models.

Without going too deep into the technicalities of it, the EUTXO model essentially allows the validity of transactions to be checked off-chain before the transaction is sent to the blockchain. This is in contrast to Ethereum Layer 1 which processes transactions all on-chain. Transaction execution costs can be also determined off-chain before transmission which is another unique feature.

Plutus Core will be used to define the parameters of these EUTXO transactions and compile the code developed for smart contracts. A Plutus Application Framework (PAF) will provide easy access to services and applications running on the network with full web browser interoperability.

Cardano launched native tokens on March 1st,2021 ‘Mary’ upgrade to allow users to create uniquely defined custom tokens and carry out transactions with them. Plutus will expand on current token capabilities, vastly improving minting policies which will be beneficial for NFTs which may need time locks.

Cardano Alonzo Upgrade in Q3*
Plutus is part of the Alonzo upgrade, a major upgrade stage on the Cardano roadmap which introduces smart contracts and the ability to build dapps. In an earlier blog post, the team hinted at a timeline:

“May and June will be a time for quality assurance and testing with users, which will be followed by a feature freeze lasting for four weeks. This will provide crypto exchanges and wallets with the time to upgrade and prepare for the Alonzo protocol update. We expect the Alonzo upgrade (hard fork) to happen in late summer,”
