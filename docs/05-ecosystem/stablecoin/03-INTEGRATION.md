# KHRt Integration Guide

## Important Notice

This integration guide outlines the proposed methods for integrating with the KHRt stablecoin system. All APIs, endpoints, and methods described are subject to change as we develop the system and receive regulatory approval.

## Integration Options

### 1. Direct Smart Contract Integration

```solidity
// KHRt Token Interface
interface IKHRt {
    function transfer(address to, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
    function approve(address spender, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
    function allowance(address owner, address spender) external view returns (uint256);
}

// Example Integration Contract
contract KHRtIntegration {
    IKHRt public khrt;
    
    constructor(address khrtAddress) {
        khrt = IKHRt(khrtAddress);
    }
    
    function processPayment(address from, uint256 amount) external {
        require(
            khrt.transferFrom(from, address(this), amount),
            "Payment failed"
        );
        // Process payment logic
    }
}
```

### 2. SDK Integration

```typescript
// Node.js SDK Example
import { KHRtSDK } from '@selendra/khrt-sdk';

const sdk = new KHRtSDK({
    network: 'mainnet', // or 'testnet'
    apiKey: 'YOUR_API_KEY'
});

// Process payment
async function processPayment(amount: number, fromAddress: string) {
    try {
        const payment = await sdk.payments.create({
            amount: amount,
            from: fromAddress,
            currency: 'KHRt'
        });
        return payment;
    } catch (error) {
        console.error('Payment failed:', error);
    }
}

// Monitor balance
sdk.accounts.onBalanceChange(address, (balance) => {
    console.log('New balance:', balance);
});
```

### 3. REST API Integration

```typescript
// API Endpoints (Proposed)
const API_ENDPOINTS = {
    // Account Management
    createAccount: '/v1/accounts',
    getBalance: '/v1/accounts/{address}/balance',
    getTransactions: '/v1/accounts/{address}/transactions',
    
    // Payments
    createPayment: '/v1/payments',
    getPaymentStatus: '/v1/payments/{id}',
    refundPayment: '/v1/payments/{id}/refund',
    
    // Webhooks
    registerWebhook: '/v1/webhooks',
    listWebhooks: '/v1/webhooks',
    deleteWebhook: '/v1/webhooks/{id}'
};

// Example API Call
async function getAccountBalance(address: string): Promise<number> {
    const response = await fetch(
        `${API_BASE}${API_ENDPOINTS.getBalance.replace('{address}', address)}`,
        {
            headers: {
                'Authorization': `Bearer ${API_KEY}`,
                'Content-Type': 'application/json'
            }
        }
    );
    const data = await response.json();
    return data.balance;
}
```

## Integration Scenarios

### 1. E-commerce Integration

```javascript
// Example using Express.js
const express = require('express');
const { KHRtSDK } = require('@selendra/khrt-sdk');

const app = express();
const khrt = new KHRtSDK({ /* config */ });

app.post('/checkout', async (req, res) => {
    try {
        const payment = await khrt.payments.create({
            amount: req.body.amount,
            currency: 'KHRt',
            metadata: {
                orderId: req.body.orderId,
                customer: req.body.customerId
            }
        });
        
        res.json({ paymentUrl: payment.checkoutUrl });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.post('/webhook/khrt', async (req, res) => {
    const event = khrt.webhooks.constructEvent(
        req.body,
        req.headers['khrt-signature']
    );
    
    switch (event.type) {
        case 'payment.succeeded':
            await fulfillOrder(event.data.orderId);
            break;
        case 'payment.failed':
            await cancelOrder(event.data.orderId);
            break;
    }
    
    res.json({ received: true });
});
```

### 2. Mobile App Integration

```swift
// iOS Swift Example
import KHRtSDK

class PaymentViewController: UIViewController {
    let khrt = KHRtSDK(apiKey: "YOUR_API_KEY")
    
    func initiatePayment(amount: Decimal) {
        khrt.createPayment(
            amount: amount,
            currency: "KHRt"
        ) { result in
            switch result {
            case .success(let payment):
                self.showPaymentSheet(payment)
            case .failure(let error):
                self.showError(error)
            }
        }
    }
}

// Android Kotlin Example
import org.selendra.khrt.sdk.*

class PaymentActivity : AppCompatActivity() {
    private val khrt = KHRtSDK("YOUR_API_KEY")
    
    fun initiatePayment(amount: BigDecimal) {
        lifecycleScope.launch {
            try {
                val payment = khrt.createPayment(
                    amount = amount,
                    currency = "KHRt"
                )
                showPaymentSheet(payment)
            } catch (e: Exception) {
                showError(e)
            }
        }
    }
}
```

### 3. Point of Sale Integration

```typescript
// React POS Example
import { KHRtTerminal } from '@selendra/khrt-terminal';

const POS: React.FC = () => {
    const terminal = new KHRtTerminal({
        apiKey: 'YOUR_API_KEY',
        deviceId: 'POS_001'
    });
    
    const handlePayment = async (amount: number) => {
        try {
            const payment = await terminal.collectPayment({
                amount,
                currency: 'KHRt',
                metadata: {
                    posId: 'POS_001',
                    cashierId: 'CASHIER_001'
                }
            });
            
            if (payment.status === 'succeeded') {
                printReceipt(payment);
            }
        } catch (error) {
            showError(error);
        }
    };
    
    return (
        // POS UI components
    );
};
```

## Best Practices

### 1. Security

- Store API keys securely
- Validate all webhook signatures
- Implement rate limiting
- Use HTTPS for all API calls
- Regular security audits

### 2. Error Handling

```typescript
// Error Handling Example
try {
    const payment = await khrt.payments.create({
        amount: amount,
        currency: 'KHRt'
    });
} catch (error) {
    if (error instanceof KHRtValidationError) {
        // Handle validation errors
    } else if (error instanceof KHRtApiError) {
        // Handle API errors
    } else if (error instanceof KHRtNetworkError) {
        // Handle network errors
    } else {
        // Handle unknown errors
    }
}
```

### 3. Testing

```typescript
// Test Environment Setup
const khrt = new KHRtSDK({
    network: 'testnet',
    apiKey: 'TEST_API_KEY'
});

// Test Account
const TEST_ACCOUNT = {
    address: '0xtest...',
    privateKey: 'test_private_key'
};

// Test Helper
async function createTestPayment(amount: number) {
    return await khrt.payments.create({
        amount,
        currency: 'KHRt',
        testMode: true
    });
}
```

## Integration Checklist

1. **Setup**
   - [ ] Register for API access
   - [ ] Generate API keys
   - [ ] Configure webhook endpoints
   - [ ] Set up test environment

2. **Implementation**
   - [ ] Implement basic integration
   - [ ] Add error handling
   - [ ] Set up webhooks
   - [ ] Implement logging

3. **Testing**
   - [ ] Test with test accounts
   - [ ] Verify webhook handling
   - [ ] Test error scenarios
   - [ ] Performance testing

4. **Production**
   - [ ] Security review
   - [ ] Switch to production API keys
   - [ ] Monitor transactions
   - [ ] Set up alerts

## Support

- Developer Portal: [dev.selendra.org](https://dev.selendra.org)
- API Documentation: [docs.selendra.org/api](https://docs.selendra.org/api)
- Support Email: dev@selendra.org
- Developer Chat: [@selendra_dev](https://t.me/selendra_dev)

## Legal Notice

This integration guide is provided for informational purposes only. All integrations must comply with applicable regulations and obtain necessary approvals before going live.
