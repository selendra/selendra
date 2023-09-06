# Parameters

These are the initial parameters for Selendra. However, please note that these values can be updated through on-chain governance in the future.

## Periods

- Block: 6 seconds (generally one block per slot, although see note below)
- Epoch: 4 hours (2,400 slots x 6 seconds)
- Session: 4 hours (Session and Epoch lengths are the same)
- Era: 24 hours (6 sessions per Era, 2,400 slots x 6 epochs x 6 seconds)

| **Selendra** | **Time**    | **Blocks*** |
|--------------|-------------|-------------|
| Block        | 6 seconds   | 1           |
| Epoch        | 4 hours     | 2,400       |
| Session      | 4 hours     | 2,400       |
| Era          | 24 hours    | 14,400      |

## Accounts and Identity

- The existential deposit is 0.1 SEL.
- The deposit required to set an on-chain identity is 5.0000 SEL.

## Staking, Validating, and Nominating

The maximum number of Validators that can be nominated by a nominator is 16.

| **Selendra**      | **Time** | **Slots** | **Description**                                                   |
|-------------------|----------|-----------|-------------------------------------------------------------------|
| Term duration     | 1 Day    | 14,400    | The time for which a validator is in the set after being elected. Please note that this duration can be shortened in the case that a validator misbehaves. |
| Nomination period | 1 Day    | 14,400    | How often a new validator set is elected according to Phragmén's method. |
| Bonding duration  | 28 days  | 403,200   | How long until your funds will be transferable after unbonding. Please note that the bonding duration is defined in eras, not directly by slots. |
| Slash defer duration | 28 days | 403,200 | Prevents overslashing and validators from "escaping" and getting their nominators slashed with no repercussions to themselves. Please note that the bonding duration is defined in eras, not directly by slots. |

## Treasury

| **Treasury**           | **Time** | **Slots** | **Description**                               |
|------------------------|----------|-----------|-----------------------------------------------|
| Periods between spends | 24 days  | 345,600   | The time when the treasury can spend again after spending previously.

## Precision

SEL has 18 decimals of precision. In other words, 10^18 (1,000,000,000,000,000,000, or one quintillion) Plancks make up a SEL.
