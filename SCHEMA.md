# Block
A single block part of the chain. Each authority must ensure the chain never forks, the earliest signed block takes precedence,
the other is discarded. Each block must be signed by 35% of the network.

Name|Type|Description
-|-|-
`id`|Id|The ID of this block
`previous`|Id|The ID of the previous block
`transactions`|Transaction[]|All the transactions happening inside this block, limited to 64
`signatures`|Signature[]|The authority signatures needed to validate this block

# Transaction
A single transaction happening inside a block. Every available action is described below.
Some action types may require a certain percentage of approval, stored inside the `approvals` field.
The `signatures` field only contains account signatures of the involved identities.

Name|Type|Description
-|-|-
`action`|String|The machine readable action of this transaction
`data`|T|The content of this transaction, specific to each type
`signatures`|Signature[]|The account signatures needed to validate this block
`approvals`|Signature[]|The authority signatures needed to approve this block

# Signature
The signature uses RSASSA-PKCS1-v1_5 ([RFC8017, Section 8.2](https://datatracker.ietf.org/doc/html/rfc8017#section-8.2)).
In the case of a transaction, the whole `data` block is used for creating the hash, while the `transactions` block is used for individual blocks.
This block is serialized using the JSON specs with as little whitespaces as possible, and the timestamp is appened.

Name|Type|Description
-|-|-
`signer_type`|String|One of `AUTHORITY` or `ACCOUNT`, depending on what key is used
`signer`|Id|The ID of the identity who owns the key
`timestamp`|ISO8601Timestamp|When the signature was performed
`signature`|String|The Base64 signature of the transaction data or of the whole block and the timestamp

# Transaction Types
## Transfer

Action|Description
-|-
`TRANSFER`|Transfer coins from one account to the other
`EXTRACT`|Extract the coins recieved by an authority to an account
`CREDIT`|Add credits to a single account. Require 90% approval.

Name|Type|Description
-|-|-
`from`|Id?|The account or authority sending coins, null if `CREDIT`
`to`|Id|The account receiving coins
`amount`|Integer|The amount of coins being transfered
`reward`|Integer|The amount of coins given to the authorities signing this transfer. Must be at least one.

Signature|Author
-|-
`SENDER`|Key of the `from` account or authority, only if not `CREDIT`

## Identity

Action|Description
-|-
`CREATE_ACCOUNT`|Push the new account onto the chain
`CREATE_AUTHORITY`|Push a new authority onto the chain and accept it inside the network. Requires 51% approval.

Name|Type|Description
-|-|-
`id`|Id|The hash of the `key` field
`key`|String|The ASCII-armored key tied to this identity
`name`|String|Human readable name for accounts, endpoint for authorities, limited to 79 characters

Signature|Author
-|-
`IDENTITY_KEY`|Key inside the `key` field
