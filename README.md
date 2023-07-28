this error was generated calling `insert` function
```
Call was rejected:
Request ID: b1740d7b45a346d2024e4be5d467975a8d6a4eee117f978c29f8fefe6794175c
Reject code: 5
Reject text: Canister bkyz2-fmaaa-aaaaa-qaaaq-cai trapped explicitly: Panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:524:5
```

here is my log:
```
2023-07-28 21:01:58.587654 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] 0
2023-07-28 21:01:58.587654 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] 0
2023-07-28 21:01:58.587654 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] 1
2023-07-28 21:01:58.587654 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] 0
2023-07-28 21:01:58.587654 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] 1
2023-07-28 21:01:58.587654 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] Panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:524:5
```