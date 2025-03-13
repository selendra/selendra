# Network Latency Requirements for Selendra Nodes

Network latency is a critical factor in blockchain performance, especially for networks like Selendra with fast block times and quick finality. This guide explains latency requirements, their impact on different node types, and how to optimize your network connection.

## Understanding Latency in Blockchain Networks

Latency refers to the time it takes for data to travel from one point to another in a network, typically measured in milliseconds (ms). In blockchain networks, low latency is crucial for:

- Rapid block propagation
- Timely consensus participation
- Consistent validator performance
- Smooth network synchronization

## Latency Requirements by Node Type

### Validator Nodes

Validators are responsible for block production and consensus, making them particularly sensitive to network latency:

| Latency Range | Performance Impact |
|---------------|-------------------|
| Under 30ms    | Optimal - excellent validator performance |
| 30-50ms       | Very good - minimal impact on operations |
| 50-100ms      | Good - acceptable for most validators |
| 100-150ms     | Acceptable - may occasionally affect performance |
| 150-200ms     | Marginal - may lead to missed blocks |
| Over 200ms    | Problematic - frequent missed blocks likely |

**Why validators are sensitive to latency:**
- With 1-second block times, validators need to receive, validate, and propagate blocks quickly
- AlephBFT consensus requires timely message exchange between validators
- High latency can result in missed blocks when it's your turn to produce

### RPC Nodes

Nodes that primarily serve API requests have more lenient latency requirements:

| Latency Range | Performance Impact |
|---------------|-------------------|
| Under 100ms   | Optimal for RPC services |
| 100-200ms     | Good - minimal impact on operations |
| 200-300ms     | Acceptable - slight delay in responses |
| Over 300ms    | May result in timeouts for some operations |

### Regular Full Nodes

Full nodes that sync and verify the blockchain but don't participate in consensus:

| Latency Range | Performance Impact |
|---------------|-------------------|
| Under 150ms   | Optimal synchronization |
| 150-250ms     | Good - stays in sync with minimal delay |
| 250-350ms     | Acceptable - may lag slightly behind chain head |
| Over 350ms    | May struggle to keep up with fast blocks |

## Impact of Selendra's Architecture on Latency Requirements

Selendra's design creates specific latency considerations:

1. **1-second block times**: Requires faster network communication than chains with longer block times
2. **AlephBFT consensus**: Achieves 2-3 second finality but requires prompt message exchange
3. **Validator slot rotation**: Validators must respond quickly when scheduled for block production
4. **Southeast Asia focus**: Network topology optimized for regional connectivity

## How to Measure Your Network Latency

To assess your connection's suitability for running a Selendra node:

1. **Basic latency test**: 
   ```bash
   ping selendra.org
   ```

2. **Test latency to known Selendra nodes**:
   ```bash
   ping boot-node.selendra.org
   ```

3. **Comprehensive network analysis**:
   ```bash
   mtr -r -c 100 selendra.org
   ```

4. **Testing validator-specific ports**:
   ```bash
   nc -zv boot-node.selendra.org 30333
   nc -zv boot-node.selendra.org 30343
   ```

## Optimizing Your Network Connection

If your latency is higher than recommended, consider these improvements:

### Physical Connection Improvements
- Use wired Ethernet instead of WiFi
- Upgrade to fiber optic internet if available
- Ensure your network interface supports Gigabit Ethernet

### Server Location Selection
- Choose data centers in Southeast Asia for optimal connection to Selendra
- Consider locations in Singapore, Vietnam, or Cambodia for best results
- Use traceroute to find optimal geographic positioning

### Network Configuration
- Implement Quality of Service (QoS) to prioritize Selendra traffic
- Use a dedicated connection for validator operations
- Consider BGP peering for more direct routing

### Provider Selection
- Choose ISPs with good peering agreements
- Business-grade internet connections often have better routing
- Look for providers with SLAs guaranteeing low latency

## Additional Network Considerations

While latency is important, also consider:

- **Bandwidth**: Recommended minimum 10 Mbps upload/download, 100+ Mbps preferred
- **Packet loss**: Should be under 0.1% for optimal performance
- **Jitter**: Variation in latency should be minimal (under 10ms)
- **Connection stability**: Consistent uptime is critical for validators

## Conclusion

Your network's latency has a direct impact on your node's performance in the Selendra network. While validators benefit most from very low latency connections, most node operations can function well with latencies under 200ms. When selecting hosting or connectivity options, prioritize low latency connections to ensure optimal participation in the network. 