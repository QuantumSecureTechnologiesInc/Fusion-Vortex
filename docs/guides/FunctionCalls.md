# Function Calls in Fusion

Fusion supports first-class function calls. Functions are defined using the `fn` keyword and can accept arguments and return values.

## Declaration

```fusion
fn add(a: Int, b: Int) : Int {
    return a + b;
}
```

## Compilation & Execution

1. **Compilation**:
   - Each `FunctionDecl` is compiled into a `Function` object containing a `Chunk` of bytecode.
   - Global functions are stored and resolved by name.
   - Using a function name in an expression emits an `OpCode::Constant` with the `Function` object.

2. **Invocation**:
   - `OpCode::Call(argc)` expects the callee (Function) to be on the stack below the arguments.
   - Arguments are validated against the function's arity.

3. **VM Execution**:
   - A `CallFrame` is created for the called function, initialized with an Instruction Pointer (`ip`) of 0.
   - `base_pointer` is set to point to the start of the function's arguments on the execution stack.
   - Locals (and arguments) are accessed relative to this `base_pointer`.
   - `OpCode::Return` pops the return value, clears the stack frame (popping args and locals), and pushes the result back to the caller's stack.

## Example Flow

```fusion
let x = add(10, 20);
```

1. Main pushes `add` (Function).
2. Main pushes `10`.
3. Main pushes `20`.
4. `Call(2)`.
5. VM creates `CallFrame` for `add`. `BP` points to `add`. `locals[0]` is `10`, `locals[1]` is `20`.
6. `add` executes `GetLocal(0)` (10), `GetLocal(1)` (20), `Add`.
7. `add` executes `Return`. Pops 30 (result). Pops 2 args + func. Pushes 30.
8. Main resumes. `30` is on stack.
