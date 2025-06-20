# tweet-terminal

## What is `pub fn`?

In Rust, `pub fn` declares a public function, making it accessible from outside its module. Use `pub fn` for functions you want to expose as part of your library or application’s API.

**Why use it?**

- To make functions accessible from other modules or crates.
- To define your public API.

**Example:**

```rust
pub fn print(&self) {
    // function body
}
```

## What is `impl`?

In Rust, `impl` is short for "implementation block." It is used to define methods (functions) and associated functions for a struct, enum, or trait. Methods defined inside an `impl` block can access the data of the struct and provide functionality related to that type.

**Why use it?**

- To organize code related to a specific type.
- To add methods that operate on the data of a struct or enum.
- To implement traits for custom behavior.

**Rust Example:**

```rust
struct MyStruct;

impl MyStruct {
    pub fn new() -> Self {
        MyStruct
    }
    fn private_method(&self) {
        // Only accessible inside this impl
    }
}
```

**If you know Python or JavaScript:**

- `impl` in Rust is like a class in Python or JavaScript, but for adding methods to a struct (which is like a simple object).
- You use `impl` to group functions that belong to a specific type, just like you put methods inside a class in Python or JavaScript.

**Python Example:**

```python
class MyClass:
    def my_method(self):
        pass
```

**JavaScript Example:**

```javascript
class MyClass {
  myMethod() {
    // method body
  }
}
```

## What is `enum` in Rust?

An `enum` (short for "enumeration") in Rust is a type that can be one of several defined variants. Enums are useful for representing a value that could be one of a few different options, each possibly with different data.

**Why use it?**

- To model data that can take on different forms or states.
- To make code safer and more expressive by handling all possible cases.
- To use with pattern matching for clear and concise control flow.

**Rust Example:**

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        }
    }
}
```

## Example: Creating a Token from Environment Variables

This Rust code creates a `Token` using values from environment variables:

```rust
let consumer = Token::new(
    &env::var("CONSUMER_KEY")?,
    &env::var("CONSUMER_SECRET")?,
);
```

**Explanation:**

- `env::var("CONSUMER_KEY")?` tries to get the value of the `CONSUMER_KEY` environment variable. The `?` operator returns an error if the variable is not set.
- `&` gets a reference to the string value, because `Token::new` expects references.
- `Token::new(...)` creates a new `Token` struct using the provided key and secret.
- The result is stored in the `consumer` variable.

**Why use this pattern?**

- It keeps your secrets out of your code and loads them securely from the environment.
- The `?` operator makes error handling concise.
- Using references avoids unnecessary copying of data.

## What does the `::` syntax mean in Rust?

The `::` syntax in Rust is called the "path separator." It is used to access items (like functions, types, constants, or modules) that are defined inside modules, structs, enums, or traits.

**Common uses:**

- To call an associated function or constant: `String::from("hi")`
- To access a type or function inside a module: `std::env::var("KEY")`
- To use an enum variant: `Option::Some(5)`

**Rust Example:**

```rust
let s = String::from("hello"); // Calls the 'from' function associated with String
let key = std::env::var("API_KEY"); // Calls the 'var' function in the 'env' module of 'std'
let value = Option::Some(10); // Refers to the 'Some' variant of the Option enum
```

**If you know Python or JavaScript:**

- In Python, it's like using `module.function()` or `Class.method()`:
  ```python
  import os
  key = os.environ.get("API_KEY")  # os.environ is like std::env, get is like var
  s = str("hello")  # str() is like String::from()
  ```
- In JavaScript, it's like using `Object.method()` or `Namespace.Function()`:
  ```javascript
  const s = String.from("hello"); // String.from is like String::from in Rust
  const key = process.env.API_KEY; // process.env is like std::env
  ```

**Why use it?**

- It keeps code organized and clear about where each item comes from.
- It allows you to use functions, types, and constants from other modules or crates.

## What does the `&` (ampersand) mean in Rust?

The `&` symbol in Rust is used to create a reference to a value, rather than moving or copying the value itself. This allows you to let a function or method use your data without taking ownership of it.

**In the example:**

```rust
let consumer = Token::new(
    &env::var("CONSUMER_KEY")?,
    &env::var("CONSUMER_SECRET")?,
);
```

- `env::var("CONSUMER_KEY")?` returns a `String`.
- `&env::var("CONSUMER_KEY")?` creates a reference to that `String` (a `&String`).
- `Token::new` expects references, not owned values, so you pass `&`.

**If you know Python or JavaScript:**

- In Python and JavaScript, variables are usually references by default, so you don't need to use `&`.
- In Rust, you must be explicit about borrowing (using `&`) to avoid moving ownership.

**Why use it?**

- To avoid moving or copying large data.
- To allow multiple parts of your code to read the same data safely.
- To follow Rust's ownership and borrowing rules for memory safety.

## How to Create a POST Request in Rust

This example shows how to create an async POST request using the `reqwest` crate in Rust.

**Example:**

```rust
use std::env;

pub async fn twitter_auth() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let auth_url = env::var("AUTH_URL").expect("AUTH_URL must be set");

    println!("Making POST request to: {}", auth_url);

    let client = reqwest::Client::new();
    let _res = client.post(auth_url)
        .body("the exact body that is sent")
        .send()
        .await?;

    Ok(())
}
```

**Step-by-step explanation:**

1. **Create a client**: `reqwest::Client::new()` creates a new HTTP client
2. **Make POST request**: `client.post(url)` creates a POST request to the specified URL
3. **Add body**: `.body("content")` adds the request body
4. **Send request**: `.send().await?` sends the request asynchronously
5. **Handle errors**: The `?` operator propagates errors up to the caller

**Requirements in Cargo.toml:**

```toml
[dependencies]
reqwest = { version = "0.12", features = ["blocking", "json"] }
tokio = { version = "1.0", features = ["full"] }
dotenv = "0.15"
```

**Main function setup:**

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    twitter_auth().await?;
    Ok(())
}
```

## What is `match` in Rust?

In Rust, `match` is a control flow construct that lets you compare a value against a series of patterns and execute code based on which pattern matches. It is similar to `switch` statements in other languages, but more powerful and exhaustive.

**Why use it?**

- To handle different possible values of enums, `Result`, `Option`, and other types.
- To ensure all possible cases are considered, making your code safer.

**Rust Example:**

```rust
let result = some_function();
match result {
    Ok(value) => println!("Success: {}", value),
    Err(e) => println!("Error: {}", e),
}
```

Here, `match` checks if `result` is `Ok` or `Err` and runs the corresponding code block.

**If you know Python or JavaScript:**

- In Python, it's like using `if/elif/else` for different cases.
- In JavaScript, it's similar to a `switch` statement, but with more pattern matching power.

**Python Example:**

```python
result = some_function()
if result.is_ok():
    print(f"Success: {result.value}")
else:
    print(f"Error: {result.error}")
```

**JavaScript Example:**

```javascript
switch (result.status) {
  case "ok":
    console.log("Success:", result.value);
    break;
  case "error":
    console.log("Error:", result.error);
    break;
}
```

## Multiple Case Matching and Switch-like Behavior in Rust

Rust's `match` can handle multiple patterns in a single arm using the `|` operator, similar to handling multiple cases in a `switch` statement in other languages.

**Rust Example: Multiple Patterns**

```rust
let number = 2;
match number {
    1 | 2 => println!("One or Two"),
    3 => println!("Three"),
    4..=10 => println!("Between Four and Ten"),
    _ => println!("Something else"),
}
```

- `1 | 2` matches if `number` is 1 or 2.
- `4..=10` matches any number from 4 to 10 (inclusive).
- `_` is the catch-all pattern (like `default` in switch).

**JavaScript Switch Example:**

```javascript
switch (number) {
  case 1:
  case 2:
    console.log("One or Two");
    break;
  case 3:
    console.log("Three");
    break;
  default:
    if (number >= 4 && number <= 10) {
      console.log("Between Four and Ten");
    } else {
      console.log("Something else");
    }
}
```

Rust's `match` is more flexible and can match ranges, multiple values, and even destructure data.
