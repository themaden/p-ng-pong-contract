PingPongContract
This is a smart contract called PingPongContract. It allows users to perform "ping" and "pong" actions with a specific payment amount. The contract ensures that a user can only "pong" after they have previously "pinged" and a certain amount of time has passed.

Features
Ping Action (ping):

Users can perform the "ping" action by making a payment with a specified amount and token type.
The ping action can only be performed with the correct token and amount.
Once a user performs a ping, they cannot ping again.
Pong Action (pong):

Users can perform the "pong" action only after performing the "ping" action first.
The pong action can only be performed after a certain time duration has passed from the ping.
Once the pong action is successful, the payment amount is refunded to the user.
An event is triggered once the pong action is completed.
Management and State Data:

Ping Amount: The payment amount required for the ping action.
Token Type: Specifies which token is accepted for payments.
Duration: The waiting time between ping and pong actions.
User State: Tracks whether a user has pinged and stores their timestamp for ping actions.
Functions
1. init
Description: Initializes the smart contract and sets up the initial parameters.
Parameters:
ping_amount (BigUint): The amount required for the ping action.
duration_in_seconds (u64): The duration (in seconds) between ping and pong actions.
accepted_payment_token_id (TokenIdentifier): The identifier of the accepted payment token.
2. ping
Description: The user performs a ping action by making a payment with the correct token and amount.
Features:
The user can only perform the ping action with the correct token and payment amount.
Each user can only ping once.
When the ping action is successful, the timestamp of the user's ping is stored.
3. pong
Description: The user performs a pong action, but only if they have already pinged.
Features:
The user can only perform the pong action after waiting the specified duration from their ping.
When the pong action is successful, the user is refunded the amount they paid during the ping.
After the pong action, the user's data is cleared, and an event is triggered.
4. getPongEnableTimestamp
Description: Returns the timestamp when the user can perform the pong action.
Parameters:
user (ManagedAddress): The address of the user.
5. getTimeToPong
Description: Returns the remaining time for the user to perform the pong action.
Parameters:
user (ManagedAddress): The address of the user.
6. getAcceptedPaymentToken
Description: Returns the identifier of the accepted payment token.
7. getPingAmount
Description: Returns the required payment amount for the ping action.
8. getDurationTimestamp
Description: Returns the duration between the ping and pong actions.
Events
1. pong_event
Description: Triggered when the pong action is successfully completed.
Parameters:
user (ManagedAddress): The address of the user.
amount (BigUint): The amount that was refunded to the user.
Data Storage
The contract stores the following data:

accepted_payment_token_id: The identifier of the accepted payment token.
ping_amount: The required payment amount for the ping action.
duration_in_seconds: The duration between the ping and pong actions.
user_ping_timestamp: The timestamp of when the user performed the ping action.
did_user_ping: A state indicating whether the user has performed the ping action.
Usage
Initializing the Contract: When initializing the contract, the accepted payment token, ping amount, and duration must be specified.

rust
Kopyala
Düzenle
contract.init(BigUint::from(10u64), 60, TokenIdentifier::from("EGLD"));
Performing the Ping Action: A user performs the ping action by making the correct payment:

rust
Kopyala
Düzenle
contract.ping();
Performing the Pong Action: After waiting for the specified duration, the user can perform the pong action:

rust
Kopyala
Düzenle
contract.pong();
For more information and usage examples, please refer to the documentation.
