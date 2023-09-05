# Parameters

This is the initial parameter for Selendra. Though, the values can be updated via on-chain governance in the future.

## Periods 

- Block: 6 seconds *(generally one block per slot, although see note below)
- Epoch: 4 hours (2_400 slots x 6 seconds)
- Session: 4 hours (Session and Epoch lengths are the same)
- Era: 24 hours (6 sessions per Era, 2_400 slots x 6 epochs x 6 seconds)

| **Selendra**	| **Time**	    | **Block***    | 
|---------------|---------------|---------------|
| Block	        | 6 seconds	    | 1             | 
| Epoch	        | 4 hours	    | 2_400         | 
| Session	    | 4 hours	    | 2_400         |
| Era	        | 24 hours	    | 14_40         | 

## Accounts and Identity

- The existential deposit is 0.1 SEL
- The deposit required to set an on-chain identity is 5.0000 SEL

## Staking, Validating, and Nominating

Maximum number of Validators that can be nominated by a nominator - 16

| **Selendra**	| **Time**	| **Slots**	| **Description** | 
|---------------|-----------|-----------|-----------------|
| Term duration	| 1 Day	| 14_400	| The time for which a validator is in the set after being elected. Note, this duration can be shortened in the case that a validator misbehaves.| 
| Nomination period	| 1 Day	| 14_400	| How often a new validator set is elected according to Phragmén's method.| 
| Bonding duration	| 28 days	| 403_200	| How long until your funds will be transferrable after unbonding. Note that the bonding duration is defined in eras, not directly by slots.| 
| Slash defer duration	| 28 days	| 403_200	| Prevents overslashing and validators "escaping" and getting their nominators slashed with no repercussions to themselves. Note that the bonding duration is defined in eras, not directly by slots.| 

## Treasury
| **Treasury**	| **Time**	| **Slots**	| **Description** | 
|---------------|-----------|-----------|-----------------|
|Periods between spends |	24 days	| 345_600 | When the treasury can spend again after spending previously.| 


## Precision
SEL have 18 decimals of precision. In other words, 10 ** 18 (1_000_000_000_000_000_000 or one thausand billion) Plancks make up a SEL.

