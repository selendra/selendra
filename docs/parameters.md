# Parameters

These are the initial parameters for Selendra. However, these values can be updated via on-chain governance in the future.

## Periods 

- Block: 6 seconds (generally one block per slot, although see the note below)
- Epoch: 4 hours (2,400 slots x 6 seconds)
- Session: 4 hours (Session and Epoch lengths are the same)
- Era: 24 hours (6 sessions per Era, 2,400 slots x 6 epochs x 6 seconds)

| **Selendra**	| **Time**	    | **Blocks**    | 
|---------------|---------------|---------------|
| Block	        | 6 seconds	    | 1             | 
| Epoch	        | 4 hours	    | 2,400         | 
| Session	    | 4 hours	    | 2,400         |
| Era	        | 24 hours	    | 14,400        | 

## Accounts and Identity

- The existential deposit is 0.1 SEL.
- The deposit required to set an on-chain identity is 5,000 SEL.

## Staking, Validating, and Nominating

A nominator can select up to 16 validators, but can only nominate one validator at a time.

A validator can accept an unlimited number of nominators, but only a certain number of nominators will be selected to join block authoring per era.

| **Selendra**	| **Duration**	| **Slots**	| **Description** | 
|---------------|-----------|-----------|-----------------|
| Term duration	| 1 Day	| 14,400	| The time for which a validator is in the set after being elected. Note that this duration can be shortened in the case that a validator misbehaves.| 
| Nomination period	| 1 Day	| 14,400	| How often a new validator set is elected according to Phragmén's method.| 
| Bonding duration	| 28 days	| 403,200	| How long until your funds will be transferrable after unbonding. Note that the bonding duration is defined in eras, not directly by slots.| 
| Slash defer duration	| 28 days	| 403,200	| Prevents overslashing and validators from "escaping" and getting their nominators slashed with no repercussions to themselves. Note that the bonding duration is defined in eras, not directly by slots.| 

## Treasury

| **Treasury**	| **Duration**	| **Slots**	| **Description** | 
|---------------|-----------|-----------|-----------------|
| Periods between spends | 24 days	| 345,600 | The intervals at which the treasury can spend again after spending previously.| 

## Precision

SEL has 18 decimal places of precision. In other words, 10^18 (1,000,000,000,000,000,000 or one quintillion) Plancks make up one SEL.
