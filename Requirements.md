# Project highlevel requirements

## Project Objectives

1. We'd like to share the learning with smart contract developers that ink! / Wasm based ones on the Substrate based blockchain communities, either Polkadot / Kusama parachians, or standalone Substrate blockchains, is a vilable alternative option to develope, deploy, upgrade, execute smart contracts, with simplified coding, lower costs, better performance and maintainability.
2. We'd like to show enterprises the possibilities of trustlessly transacting with business partners by leveraging blockchain's transparency and workflow automation via smart contracts, generated from Workflow specifications, interact and transact efficiently without intermediaries across border / enterprise boundaries. 

## Technical Objectives 
In this project, we want to compare ink / Wasm based smart contract development, with that of Solidity / EVM, to demonstrate some of its comparative benefits in terms of:

1. Code simplicity and reuse via ink! macros
2. Upgradability without cumbersome data migraiton and hard folks 
3. Lower gas fees, higher performance and smaller bytecode footprints via custom pallets, Wasm and Rust optimizations  
4. In addition, evalute some relevant tool deatures such as those from OpenZepplin, Brushfam's Sol2ink, templates, etc.  

## Milestones
### Milestone1 
* Define use cases with Serverless Workflow DSL (spec: https://github.com/serverlessworkflow/specification and examples https://github.com/serverlessworkflow/specification/tree/main/examples)
* Implement in both ink! and Solidity. 
* Define ink! macros translating workflow DSL spec into ink! smart contracts for code simplicity, modularization and reuse. Iterate for optimal balance among macos, smart contract code and custom pallet to reduce gas fees / simplcity as compared to Solidity
* End-to-end integrations, compile into Wasm bytecode and deploy auto generated smart contracts on Substrate blockchain and execute
* Quality documentation and comparison analysis 

### Milestone2
* Upon workflow changes, seamlessly upgrade smart contracts on Substrate blockchain
* smart contract related tool evaluations 
* Quality documentation and comparison analysis

### Milestone3
* Benchmark gas fees & performance of smart contract implementations to optimize, and compare with Solidity / EVM based implementations
* Documentations and analysis
* Open source & publish 

# Workflow DSL
[Serverless Workflow DSL ]( https://github.com/serverlessworkflow/specification ) is a low-code, event-driven workflow orchestration domain specific lanuage that is open source and vendor neutral. [Synapse](https://github.com/serverlessworkflow/synapse) is a Workflow Management System comes with it. There are some nice [examples](https://github.com/serverlessworkflow/specification/tree/main/examples). We chose it for this project as it is open source, general purposed, relative simple as compared to other full blown ones such as BPMN (comparison article [here](https://blog.automatiko.io/2022/05/15/serverless-vs-bpmn.html)). 

## Prototype Example
1. A very simple online food ordering system (adapted from the original example) with Order, Delivery and Payment services, by three businesses indepedently: restaurant (In & Out Burger), delivery (like Doordash or Uber Eats), and Payment Processor (like Stripe, Paypal), yet orchestrated via the same workflow, to serve online orders.
2. We'll use YAML files to specify the workflow (interchangeable with JSON format)

### Workflow Event Definition

``` yaml
# file://orderevents.yml
events:
- name: Food Order Event
  source: "/orders/"
  type: org.orders
  correlation:
  - contextAttributeName: orderid
- name: ETA Deadline Event
  source: "/ordersETA"
  type: org.orders.eta
  correlation:
  - contextAttributeName: orderid
- name: Order Picked Up Event
  source: "/ordersPickup"
  type: org.orders.delivery
  correlation:
  - contextAttributeName: orderid
- name: Order Delievered Event
  source: "/orderDelivery"
  type: org.orders.delivery
  correlation:
  - contextAttributeName: orderid
```

### Workflow Function Definition

``` yaml
# file://orderfunctions.yml
functions:
- name: Submit Order Function
  operation: http://OrderService.org/orders.json#submit
- name: Get Order ETA Function
#Random between 1-3 hours
  operation: http://OrderService.org/orders.json#orderETA 
- name: Dispatch Courrier Function
  operation: http://DeliveryService.org/deliveries.json#dispatch
- name: Deliver Order Function
  operation: http://DeliveryService.org/deliveries.json#deliver
- name: Charge For Order Function
#Add dollar amount
  operation: http://PaymentService.org/payments.proto#PaymentService#ChargeUser 
```

### Main Workflow Definition

With the function and event definitions above here's the main workflow definition:

```yaml
# This is the main workflow for this prototype. Needs ot be adapted to smart contracts, instead of the current API calls 
# Waiting should not be part of smart contract but rather in-between triggering events. 
id: FoodOrderWorkflow
name: Food Order Workflow
version: '1.0.0'
specVersion: '0.8'
start: Place Order
functions: file://orderfunctions.yml
events: file://orderevents.yml
states:
- name: Place Order
  type: operation
  actions:
  - subFlowRef: PlaceOrderWorkflow
  transition: Wait for ETA Deadline
- name: Wait for ETA Deadline
  type: event
  onEvents:
  - eventRefs:
    - ETA Deadline Event
    eventDataFilter:
      data: "${ .results.status }"
      toStateData: "${ .status }"
  transition: Deliver Order
- name: Deliver Order
  type: operation
  actions:
  - subFlowRef: DeliverOrderWorkflow
  transition: Charge For Order
- name: Charge For Order
  type: operation
  actions:
  - functionRef:
      refName: Charge For Order Function
      arguments:
        order: "${ .order.id }"
    actionDataFilter:
      results: "${ .outcome.status }"
      toStateData: "${ .status }"
  stateDataFilter:
    output: '${ . | {"orderid": .id, "orderstatus": .status} | .orderstatus += ["Order
      Completed"] }'
  end: true
```

#### Place Order Sub-Workflow

```yaml
# this irder taking subflow can be implemented as smart contrac assuming ETA estimates with random time between 1-3 hours or failure
id: PlaceOrderWorkflow
name: Place Order Workflow
version: '1.0.0'
specVersion: '0.8'
start: Submit Order
states:
- name: Submit Order
  type: event
  onEvents:
  - eventRefs:
    - Food Order Event
    actions:
    - functionRef:
        refName: Submit Order Function
        arguments:
          order: "${ .order }"
      actionDataFilter:
        results: "${ .results.status }"
        toStateData: "${ .status }"
    - functionRef:
        refName: Get Order ETA Function
        arguments:
          customer: "${ .customerId }"
          restaurantid: "${ .order.restaurantId }"
          delivery: " ${ .delivery }"
      actionDataFilter:
        results: "${ .results.status }"
        toStateData: "${ .status }"
  end: true
```

#### Deliver Order Sub-Workflow

```yaml
# Dispatch and Delivery can be implemented as smart contracts respectively, with timeout failure handling.
# Waiting should not be part of smart contract but rather in-between triggering events. 
id: DeliverOrderWorkflow
name: Deliver Order Workflow
version: '1.0.0'
specVersion: '0.8'
start: Dispatch Courier
states:
- name: Dispatch Courier
  type: operation
  actions:
  - functionRef: Dispatch Courrier Function
  transition: Wait for Order Pickup
- name: Wait for Order Pickup
  type: event
  onEvents:
  - eventRefs:
    - Order Picked Up Event
    eventDataFilter:
      data: "${ .data.status }"
      toStateData: "${ .status }"
    actions:
    - functionRef: Deliver Order Function
  transition: Wait for Delivery Confirmation
- name: Wait for Delivery Confirmation
  type: event
  onEvents:
  - eventRefs:
    - Order Delievered Event
    eventDataFilter:
      data: "${ .data.status }"
      toStateData: "${ .status }"
  end: true
```

### Customer orders food 

The following can be converted to a transaction sent to the on-chain smart contract to trigger the workflow

```json
{
   "specversion": "1.0",
   "type": "org.orders",
   "source": "/orders/",
   "subject": "Food Order",
   "id": "A0101",
   "time": "2023-06-01T17:31:00Z",
   "orderid": "ORDER-100",
   "data": {
      "id": "ORDER-100",
      "customerId": "CUSTOMER-123",
      "status": [],
      "order": {
         "restaurantId": "RESTAURANT-456",
         "items": [
            {
               "itemId": "ITEM-789",
               "amount": 1,
               "addons": ""
            }
         ]
      },
      "delivery":{
         "address": "123 Main St, San Francisco, CA",
         "type": "contactless",
         "requestedTime": "ASAP",
         "location": "Front door",
         "instructions": ""
      }
   }
}
```

### Workflow happy path output

For the example order event, the workflow output for a successful completion would look like below. Other workflow execution paths have different outputs. These are supposed to be looged on the chain as multiple immutable records with timestamps, that can be scanned and reported on later. 

```json
{
  "orderid": "ORDER-100",
  "orderstatus": [
    "Order Submitted",
    "Order ETA Received",
    "Order Picked up",
    "Order Delievered",
    "Order Charged",
    "Order Completed"
  ]
}
```

## Reuse functions and events

This example shows how function and event definitions can be declared independently and referenced by workflow definitions. This is useful when you would like to reuse event and function definitions across multiple workflows. In those scenarios it allows you to make changed/updates to these definitions in a single place without having to modify multiple workflows.

For the example we have two files, namely our "functiondefs.json" and "eventdefs.yml" (to show that they can be expressed in either JSON or YAML). These hold our function and event definitions which then can be referenced by multiple workflows.

functiondefs.json
```json
{
  "functions": [
      {
        "name": "checkFundsAvailability",
        "operation": "file://myapis/billingapis.json#checkFunds"
      },
      {
        "name": "sendSuccessEmail",
        "operation": "file://myapis/emailapis.json#paymentSuccess"
      },
      {
        "name": "sendInsufficientFundsEmail",
        "operation": "file://myapis/emailapis.json#paymentInsufficientFunds"
      }
    ]
}
```

eventdefs.yml

```yaml
events:
- name: PaymentReceivedEvent
  type: payment.receive
  source: paymentEventSource
  correlation:
  - contextAttributeName: accountId
- name: ConfirmationCompletedEvent
  type: payment.confirmation
  kind: produced
```

In our workflow definition then we can reference these files rather than defining function and events in-line.

```yaml
id: paymentconfirmation
version: '1.0.0'
specVersion: '0.8'
name: Payment Confirmation Workflow
description: Performs Payment Confirmation
functions: functiondefs.json
events: eventdefs.yml
states:
- name: PaymentReceived
  type: event
  onEvents:
  - eventRefs:
    - PaymentReceivedEvent
    actions:
    - name: checkfunds
      functionRef:
        refName: checkFundsAvailability
        arguments:
          account: "${ .accountId }"
          paymentamount: "${ .payment.amount }"
  transition: ConfirmBasedOnFunds
- name: ConfirmBasedOnFunds
  type: switch
  dataConditions:
  - condition: "${ .funds | .available == \"true\" }"
    transition: SendPaymentSuccess
  - condition: "${ .funds | .available == \"false\" }"
    transition: SendInsufficientResults
  defaultCondition:
    transition: SendPaymentSuccess
- name: SendPaymentSuccess
  type: operation
  actions:
  - functionRef:
      refName: sendSuccessEmail
      arguments:
        applicant: "${ .customer }"
  end:
    produceEvents:
    - eventRef: ConfirmationCompletedEvent
      data: "${ .payment }"
- name: SendInsufficientResults
  type: operation
  actions:
  - functionRef:
      refName: sendInsufficientFundsEmail
      arguments:
        applicant: "${ .customer }"
  end:
    produceEvents:
    - eventRef: ConfirmationCompletedEvent
      data: "${ .payment }"
```      
  
## Merge 
Combine code from these two examples (food ordering and payment) to contrive a single use case demonstrating ink macros to simply smart contract coding, code reuse and lower gas fees.


# ink! Macros
ink! macros serves as a bridge connecting business users who may or may not be literate on smart contract / blockchain coding, with the actual implementation. This layer is expected to analyze the workdlow specification and target ink! code, to come up with a list of reusable macros to encapsulate general business logic, simplify coding, maxmize reuse and parameterize workflow steps, etc. 

The macros should aim to encapsulate the most reusable components of the business logic so customers only need to add a macro with parameters in ink! contract to make the code as simple and efficient as possible. Consider moving some frequently used logic into custom pallets as part of the runtime to save gas feeds. The distribution of business logic among among macros, ink! contract and custom pallets is a fine balance, which can be aided by tangible benchmark metrics comapring various implementation options balanced for optimal outcome. 


# ink! Smart Constracts
End-to-end integration from Workflow spec, to expand macros to ink! code, compile into Wasm bytecodes, test locally, then deploy smart contracts on-chain on Substrate blockchain 
testnet and sending transactions to trigger the workflow execution, log events and deliver intended outcome. Provide Dockerfile(s) that can be used to test all the functionality, easily shared and reproduced, plus online sandbox / playground for non-technical users to toy with.    

# Upgrade Smart Contracts
One key differentiatos is smart contract upgradability, as comoared to Solidity / EVM based ones. Given the above end-to-end set up, it is desirable that upon workflow changes (business logic changes), seamlessly upgrade smart contracts on Substrate blockchain in an automated fashion, 

We can leverage ink! built-in tools / libraries to manage contract upgrades: the "Dispatcher" module, routing of functional calls during upgrade, the "Upgradeable" trait, old vs new contract instances, contract data migration, etc. 

Possible business logic changes that could cause smart contract revisions / upgredes:

* Add a max waiting time to the order taking. Cancel the order and notify customer if ETA of food preparation, or delivery, or sum of both > max waiting time.
* Add a promotional discount if order total is > $100
* Impose minimal order amount for free delivery, or charge delivery fee customer consents to
* Variable delivery rate proportional to the distance between the restaurant and delivery address

Will defer the scenario after the baseline implemented to best demonstrate end-to-end smart conract revision via macros and auto upgrade. 

# Benchmarking & Comparisons 

Prepare a test / benchmark suit. Use substrate runtime benchmarking tool / ink-bench tool to measure the gas costs of varias implementations, benchmark gas fees & performance of smart contract implementations, potentially expose some weakness such as the String type tends to bloat code, best if a solution to reduce footprint. 

Optimize overall prototype implementation by balancing logic among macros, ink contract, custom pallets for optimal metrics. Finally compare with Solidity / EVM based implementations with tangible metrics for analysis. 


# Documentations 

Create a website summarizing this prototype with technical documentations, concete results and collaterals to share the learning. Explore difference between OpenZepplin and OpenBrush in this context and share learning. Publish articles to share the experiment with the community. 





