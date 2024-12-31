# paracell

Data parallelism analyzer.

## Syntax

```rust
// Pure combinational logic.
// Time: 0 step.
fn ALU(operands: Array<Nat, 2>, op: Op) -> Nat {
    let a = operands[0];
    let b = operands[1];

    match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => a * b,
    }
}
```
