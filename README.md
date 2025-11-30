# Initial Goals

Initially this was just my own implementation of a Pratt Parser that parses arithmetic expressions. Essentially a simple calculator.

# Switching goals

Now the Pratt Parser is now frankenstein-ed into a custom language parser.
The plan is to make this into a full on functional compiler, that at least can run some complicated things.

The language spec is currently:

## Base Types

| num | Type | Underlying Type |
| :--- | :--- | :--- |
| 1 | char | char |
| 2 | uchar | unsigned char |
| 3 | short | int16_t |
| 4 | ushort | uint16_t |
| 5 | int | int32_t |
| 6 | uint | uint32_t |
| 7 | float | f32 (using IEEE754, alias for f32) |
| 8 | i8 | int8_t |
| 9 | i16 | int16_t |
| 10 | i32 | int32_t |
| 11 | i64 | int64_t |
| 12 | i128 | int128_t |
| 13 | u8 | uint8_t |
| 14 | u16 | uint16_t |
| 15 | u32 | uint32_t |
| 16 | u64 | uint64_t |
| 17 | u128 | uint128_t |
| 18 | f32 | f32 (IEEE754) |
| 19 | f64 | f64 (IEEE754) |
| 20 | string | String |
| 21 | bool | boolean (true or false) |
| 22 | void | (None) |
| 23 | T? | Optional[T] (represents an optional value of type T) |

## Type System

### Type Inference Rules
- Integer literals default to `int` (i32): `x := 42`
- Float literals default to `f64`: `pi := 3.14`
- Type suffixes override defaults: `x := 42i64`, `pi := 3.14f32`
- String literals infer to `string`: `s := "hello"`
- Boolean literals infer to `bool`: `flag := true`

### Implicit Type Conversions
The following conversions happen automatically:
- Integer widening (same signedness): i8→i16→i32→i64→i128
- Unsigned widening: u8→u16→u32→u64→u128
- Float widening: f32→f64
- int→float (with precision warning for large values)
- T→T? (wrapping value in optional)

### Explicit Type Conversions
These conversions require the `as` keyword:
- Narrowing: `large: i64 = 1000; small: i8 = large as i8`
- Float to int: `x: float = 3.14; i: int = x as int`
- Signed ↔ Unsigned: `neg: i32 = -1; pos: u32 = neg as u32`

## Comments

```go
// Single-line comment

/*
  Multi-line
  comment
*/

fnc example():
  x := 42  // inline comment
  /* block comment */ y := 10
```

## Function Syntax

**Basic Syntax**
For simple functions, the syntax should look like this:

```go
fnc add(a: int, b: int) -> int:
  ret a + b
```

- The `fnc` initiates the function declaration
- The `-> int` declares the type returned by the function. Not explicitly typing this out, would make the function void by default.
- The `ret` keyword is required for returning values.

**Void Functions**
```go
fnc greet(name: string):  // void by default
  print("Hello, ${name}!")

fnc greet_explicit(name: string) -> void:  // explicit void (equivalent)
  print("Hello, ${name}!")
```

**Return Type Rules**
- No return type annotation = `void` (function returns nothing)
- Has return type annotation = must use `ret` keyword with appropriate value
- Can write `-> void` explicitly if desired (equivalent to omitting it)
- All code paths in non-void functions must return a value

## Variable

**Inferred and Explicitly Typed**
Variables can both be inferred and explicitly written:
```go
x := 420
y: int = 69
z: i64 = 67
```

**Type Literals**
You can set type literals to inferred values:
```go
pi := 3.14152965f32
e := 2.71828f64
```

**Variable States**
Following the a lot of how Rust is typed, every variable is immutable (const) by default. This means that in order for a variable to be mutable, it has to come with the `mut` keyword, like this:

```go
mut N: string = "Hello world!" // Explicit
mut M := 40 // Inferred 
```

## String Operations

**Concatenation**
```go
greeting := "Hello" + " " + "World"
```

**Interpolation**
Strings support interpolation using `${}` syntax:
```go
name := "Alice"
age := 25
msg := "Name: ${name}, Age: ${age}"

// Expression interpolation
result := "Sum: ${10 + 20}"  // "Sum: 30"
```

**Indexing**
```go
greeting := "Hello"
first_char := greeting[0]  // 'H'
```

**Mutability**
Strings are immutable by default. Modifying a string requires creating a new string.

## Control Flow

**If / Else Flow**

Following a lot from python syntax:
```py
fnc some_function(N: i32) -> int:
  if N > 0:
    ret N
  else:
    ret -N
```
Or alternatively, you can write:
```py
fnc some_function(N: i32) -> int:
  if N > 0: ret N
  else: ret -N
```

**Logical Operators**
```go
// Logical operators
x := true and false  // false
y := true or false   // true
z := not true        // false

// Short-circuit evaluation
if x > 0 and y < 10:
  // y < 10 only evaluated if x > 0 is true

// In conditions
foreach i; 0..10:
  if i % 2 == 0 or i % 3 == 0:
    print(i)
```

**Loops**
Loops are written with the `foreach` keyword. The iterator can be implicitly casted into an int if not assigned to a value, or assigned within the foreach line, like this:
```d
foreach i; 0..10: // this is implicitly casted to an int
  print(i)
```

With explicitly assigning a type:
```d
foreach i: int; x ..= y: // inclusive
  print(i)
```

With user-specified jumps:
```c#
foreach i: int; 0 .. 10 by 2:
  if i % 2 == 0:
    print(i)
```

Descending ranges are internally handled, without having to explicitly write specified jump values:
```c#
foreach i: int; 10 ..= 0:
  if i % 2 != 0:
    print(i)
```
Do note that loops are **NOT** inclusive by default.

**Loop Control**
```go
// Break: exit loop early
foreach i; 0..10:
  if i == 5:
    break
  print(i)  // Prints 0, 1, 2, 3, 4

// Continue: skip to next iteration
foreach i; 0..10:
  if i % 2 == 0:
    continue
  print(i)  // Prints 1, 3, 5, 7, 9

// Nested loops
foreach i; 0..5:
  foreach j; 0..5:
    if i == j:
      break  // Only breaks inner loop
    print("${i}, ${j}")
```

## Compound Types

**Arrays**
You have fixed and dynamically-sized arrays, largely inspired by Dlang:
```rs
fixed_size: [int; 10] // Fixed size: Type then size
dynamic_size: [f32]   // Dynamically Sized: Just the type itself

// Array initialization
arr1: [int; 5] = [1, 2, 3, 4, 5]  // Explicit all elements
arr2: [int] = [1, 2, 3]  // Dynamic array
```

**Tuples**
```go
some_x := 56
some_y: i8 = 104
vec2: (int, i8) = (some_x, some_y) // with assignment
some_tuple: (int, string) // without assignment
```

**Structs**

Structs have all their members access modifiers public, and they can hold functions inside of them:
```go
struct Person:
  name: string
  age: i8
  money: f64

  // Constructor (Rust-style)
  fnc new(name: string, age: i8, money: f64) -> Person:
    ret Person { name, age, money }  // Shorthand when names match

  // Or explicit field names
  fnc new_explicit(p_name: string, p_age: i8, p_money: f64) -> Person:
    ret Person {
      name: p_name,
      age: p_age,
      money: p_money
    }

  // Method
  fnc get_name(self) -> string:
    ret self.name

// Usage
person := Person::new("Alice", 25, 1000.50)
name := person.get_name()

// Direct instantiation
bob := Person { name: "Bob", age: 30, money: 500.0 }
```

**Classes**

_Note: This part is still under consideration, and is not yet thought out thoroughly._
_Specifically for classes, we might just not have classes entirely._

```go
class Dog:
  public:
    name: string
    age: int

    fnc get_dog_name(self) -> string: // public function
      ret self.name

  private:
    owner: Person

    fnc increment_dog_age(self): // private function
      self.age = self.age + 1
```

**Optional / Nullable Types**
These types allow a variable or a function to either hold a value of a specified type or represent the absence of a value with `none`. This is similar to Rust's `Option` type functionality, but simplified, as values can be returned directly from functions without needing to wrap the value in Some.

```zig
x: int? = none
y: float? = 2.71
z: Person? = none
```

```zig
fnc divide_values(x: f32, y: f64) -> f32?:
  if y == 0.0:
    ret none // Directly returns none
  else: 
    ret x / y // Directly returns the value without Some
```

Do note that functions returning optional types must include at least one branch that returns `none` if a value might be absent.

**Optional Type Unwrapping**

_Note: This part is still under consideration, and is not yet thought out thoroughly_

Values can be checked for nullable-ness by:

```zig
fnc main():
  result: f32? = divide_values(10.0, 2.0)

  if result != none:
    print(result)
  else:
    print("Division by Zero! Not allowed!")
```

## Pending Design Decisions

The following features are still under consideration and have not been finalized:

### String Mutability Details
- Whether strings can be indexed and modified
- Memory model for string operations

### Array Initialization Syntax
- Default initialization syntax (e.g., `[0; 5]` for arrays of zeros)
- Dynamic array growth semantics

### Struct Field Access Control
- Whether to support private fields in structs
- Syntax for access modifiers if supported

### Function Parameters
- Pass by value vs pass by reference semantics
- How `mut` interacts with function parameters

### Method Call Syntax
- Support for both dot notation (`obj.method()`) and static style (`Type::method(obj)`)

### Loop Labels
- Support for labeled break/continue for nested loops

### Optional Type Unwrapping Syntax
- Final syntax for safely unwrapping optional types
- Whether to support force unwrap (`!`), default unwrap (`??`), or pattern matching (`if let`)
