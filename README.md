# Initial Goals

Initially this was just my own implementation of a Pratt Parser that parses arithmetic expressions. Essentially a simple calculator.

# Switching goals

Now the Pratt Parser is now frankenstein-ed into a custom language parser.
The plan is to make this into a full on function compiler, that at least can run some complicated things.

The language spec is currently:

## Base Types

| num | Type | Underlying Type |
| :--- | :--- | :--- |
| 1 | char | char |
| 2 | uchar | unsigned char |
| 3 | short | int8_t |
| 4 | ushort | int16_t |
| 5 | int | int32_t |
| 6 | uint | uint32_t |
| 7 | float | float32 (using IEEE754) |
| 8 | i8 | int8_t |
| 9 | i16 | int16_t |
| 10 | i32 | int32_t |
| 11 | i64 | int64_t |
| 12 | i128 | int128_t |
| 13 | u8 | uint8_t |
| 15 | u16 | uint16_t |
| 16 | u32 | uint32_t |
| 17 | u64 | uint64_t |
| 18 | u128 | uint128_t |
| 19 | f32 | float32 (IEEE754) |
| 20 | f64 | float64 (IEEE754) |
| 21 | string | String |
| 22 | void | (None) |
| 23 | ? | Optional |

## Function Syntax

**Basic Syntax**
For simple functions, the syntax should look like this:

```go
fnc add(a: int, b: int) -> int:
  ret a + b
```

- The `fnc` initiates the function declaration
- The `-> int` declares the type returned by the function. Not explicitly typing this out, would make the function void by default.
- The `ret` is purely optional, which allows for something like this if you prefered:

```go
fnc add(a: int, b: int) -> int:
  a + b
```

## Variable

**Inferred and Explicitly Typed**
Variables can bot be inferred and explicitly written:
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

## Control Flow

**If / Else Flow**

Following a lot from python syntax:
```py
fnc some_function(N: i32) -> int:
  if N > 0:
    ret x
  else:
    ret -x
```
Or alternatively, you can write:
```py
fnc some_function(N: i32) -> int:
  if N > 0: ret x
  else: ret -x
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
foreach i: int; 0 .. 10 by -2:
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


## Compound Types
**Arrays**
You have fixed and dynamically-sized arrays, largely inspired by Dlang:
```rs
fixed_size: [int; 10] // Fixed size: Type then size
dynamic_size: [f32]   // Dynamically Sized: Just the type itself
```

**Tuples**
```go
some_x := 56
some_y: i8 = 104
vec2: (int, i8) = (some_x, some_y) // with assignment
some_tuple: (int, string) // without assignment
```

**Structs**
_Note: This part is still under consideration, and is not yet thought out thoroughly_
Structs have all their members access modifiers public, and they can hold functions inside of them, like this:
```go
struct Person:
  name: string
  age: i8
  money: f64

  fnc new(self, p_name: string, p_age: i8, p_money: f64) -> Self:
    Self(
      self.name = p_name
      self.age= p_age
      self.money = p_money
    )
```


**Classes**
_Note: This part is still under consideration, and is not yet thought out thoroughly_
```py
class Dog:
  public:
    name: string
    age: int

    fnc get_dog_name(self) -> string: // public function
      ret self.name

  private:
    owner: Person

    fnc increment_dog_age(self): // private function
      ret self.age++
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
    none // Directly returns none
  else: 
   x / y // Directly returns the value without Some
```

Do note that functions returning optional types must include at least one branch that returns `none` if a value might be absent.

Values can later be checked its nullable-ness by:
_Note: This part is still under consideration, and is not yet thought out thoroughly_
```zig
fnc main():
  result: f32? = divide_values(10.0, 2.0)

  if result != none:
    print(result)
  else:
    print("Division by Zero! Not allowed!")
```
