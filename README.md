To reproduce bug:
```
git revert 8e132cf5e25ce59a80c1b743aa0c065b6ee60db5
cargo run
```

Bugged version output:
```
{"p2_key1": "p2_val1"}
```

To run fixed version
```
git checkout origin/master
cargo run
```

With fix output: 
```
{"p1_key2": "p1_val2", "p1_key1": "p1_val1", "p2_key1": "p2_val1"}
```