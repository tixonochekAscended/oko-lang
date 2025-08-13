<div align="center">
  <img style="height: 350px;" src="https://github.com/tixonochekAscended/oko-lang/blob/main/ffOkoBannerRadiused.png?raw=true">
</div>

------
# oko-lang – A programming language
**oko-lang** (or simply **oko**) is an interpreted, non-esoteric programming language the official implementation of which is powered by the Deno JS runtime. Features most of the things that you can find in other languages - variables, complex data structures, functions, program flow control, loops, rich standart library (built-ins) and more.

## ⚠️ Warning (& Contribution)
At the moment, oko-lang is in its early stage of development. Expect ground-breaking changes, no backward compatability, significant performance and development experience improvements and much more. If you would like to contribute to the language by any means, [we have a discord server.](https://discord.gg/NSK7YJ2R6j) There you can contact me - the main developer - and other people who are willing to help us build **oko**.

## 🔌 **VSCode Extension**
If you happen to be using __Visual Studio Code__ as your primary code editor / IDE, there is an [official extension for the oko programming language.]() Some of the features include:
1. Syntax Highlighting
2. Code Snippets
3. Keyword Snippets
4. File icon(s)

## 📂 Installation
There are __2 main ways__ to install the **oko** programming language.

1️⃣ **First Method**: Head into the **Releases/** page of this repository. Afterwards, download the version you are interested in for your respective OS and architecture. This is by far the easiest way to get the oko-lang executable.

2️⃣ **Second Method**: For this method, you need to have the **Deno JS runtime** installed on your machine. Download the source code of **oko**, and then use Deno to compile **oko** by using a command similar to this one: `deno compile --allow-all oko.js`

## ⚙️ Usage
Once you got the **oko-lang** executable, you can easily execute `.oko` files:
```
oko <Filename>.<FileExtension>
```
If you would like to see some information regarding **oko-lang**'s license, use `oko license`.

## 📜 Guide regarding oko-lang
The syntax of **oko** is dead simple. Every statement (besides the ones that end in ` { blocks } `) should have a trailing **semicolon** `;`. Example:
```js
io::println("Hello, World!");
```
To define variables, you use the `:=` assignment operator. There also are other ones, like `=`, `+=`, `-=`, `*=` and `/=`. The difference between `:=` and `=` is that the `=` operator can by definition only mutate an existing variable - it can't be used to change a variable's type or to set a value to a variable that hasn't been declared and initialized yet.
```js
var := 13;
var += 13;
var -= 13;
var *= 13;
var /= 13;
var = 13;
```
Just so you know, there are only single-line comments in **oko**, which use the following syntax:
```js
// this is a comment
```
Here is an example regarding variables that will throw an error:
```js
msg := "Hello, World!";
msg = -0.13 // can't change the variable's type via `=` from String to Number
```

Now let's get to operators. Here's a complete list of them (this doesn't include the assignment ones): `+`, `-`, `*`, `/`, `^`, `%`, `>`, `<`, `==`, `>=`, `<=`, `&&`, `||`. All of the operators listed were binary (infix). The latest version of **oko** features only one unary (prefix) operator: `!`. All of these work as expected and work only with certain data types. Usually, when `Nil` is either on the left or the right side of the expression the result is also `Nil` - but there are exceptions to that rule.

Before we continue, you must know that the only 4 data types in **oko** are: `String`, `Number`, `Array` and `Nil`. This doesn't mean the language isn't capable though.

You can easily manipulate program flow via `if`, `elif` (`else if`) and `else` statements. As you already know there are no booleans, so numbers are usually used instead of them. An empty array `[]`, zero `0`, an empty string `""` and nil are all falsy - others are truthful. Here is a code example that will output `Omega`:
```js
import io;
import tu;

if ("") {
  io::println("Steak");
} elif ([]) {
  io::println("Trophy");
} elif ( tu::getNil() ) {
  io::println("Cat");
} elif (1) {
  io::println("Omega");
} else {
  io::println("Princess");
}
```

At the moment, **oko** features 2 types of loops: `while` and `for` (which is a `forEach`). Here's an example for you:
```js
import io;

j := 1;
while (j <= 5) {
  io::println(j);
  j += 1;
}

items := ["Apple", "Banana", "Pear"];
for (el) (items) {
  io::println(el);
}
```
The output will be as following:
```
1
2
3
4
5
Apple
Banana
Pear
```

As you may have already noticed, we `import` a lot of things to get things done. The `import` keyword allows us to import **built-in modules of oko**. Here is a list of all modules and the respective functions they expose:
- `prog` (Program Control)
  - `exit(code)`: Exits the program with the specified numeric exit code
  - `throw(msg)`: Throws an error with the given string message

- `time` (Time Utilities)
  - `now()`: Returns the current timestamp in milliseconds since the Unix epoch
  - `sleep(ms)`: Stops the program execution for a certain amount of milliseconds.

- `tu` (Type Utilities)
  - `getNil()`: Returns `nil` (no value)
  - `toNumber(value)`: Converts a value to a number, if a string can not be converted to a number returns Nil, errors if value is an array
  - `toString(value)`: Converts a value to its string representation
  - `typeOf(value)`: Returns the type of the given value as a string

- `io` (Input/Output)
  - `input()`: Prompts the user for input, returns a string or nil if empty
  - ~~`print(...args)`: Prints values without newline, joined by space~~ **🔴 Does not work at the moment because of the issues caused by Deno and the inability to easily flush the standart output.**
  - `println(...args)`: Prints values with newline, joined by space
  - `readTextFile(path)`: Reads the contents of a text file at the given path. Returns the contents on success and Nil on failure.
  - `writeTextFile(path, content)`: Writes the content string to a file at the given path, returns 1 on success and Nil on failure.

- `math` (Mathematical Functions)
  - `sqrt(num)`: Returns the square root of a number or nil if invalid
  - `abs(num)`: Returns the absolute value of a number
  - `round(num)`: Returns the floor of a number (rounded down)
  - `ceil(num)`: Returns the ceiling of a number (rounded up)
  - `floor(num)`: Returns the floor of a number (rounded down)
  - `sin(num)`: Returns the sine of a number (radians)
  - `cos(num)`: Returns the cosine of a number (radians)
  - `tan(num)`: Returns the tangent of a number (radians)
  - `log(num, base)`: Returns the logarithm of a number with specified base
  - `random()`: Returns a random floating-point number between 0 and 1

- `stru` (String Utilities)
  - `len(str)`: Returns the length of a string
  - `at(str, index)`: Returns the character at the given index of a string
  - `sub(str, start, end)`: Returns a substring from start to end indices
  - `split(str, delim)`: Splits a string by the delimiter into an array of strings
  - `lower(str)`: Converts a string to lowercase
  - `upper(str)`: Converts a string to uppercase
  - `trim(str)`: Trims whitespace from both ends of a string
  - `replace(str, find, replace)`: Replaces all occurrences of `find` in the string with `replace`

- `arru` (Array Utilities)
  - `len(arr)`: Returns the length of an array
  - `at(arr, index)`: Returns the element at the given index in an array
  - `push(arr, val)`: Adds a value to the end of an array
  - `remove(arr, index)`: Removes and returns the element at the given index in an array
  - `join(arr, separator)`: Joins array elements into a string separated by the given separator, replacing nil elements with "Nil"

To use any of the described functions, you need to firstly `import` the module you want to use, f.e.:
```js
import io;
```
And then call any function from that specific module via the **mod access operator** `mod::func`:
```js
io::println("msg");
```
It's that simple.

At last, here's how you can create your own functions. You need to use the `fun` keyword to do so. Here are some code examples:
```js
import io;

fun sum(a, b) {
  return a + b;
}

fun thisReturnsNil(msg) {
  io::println(msg);
}
```
If a function doesn't feature a `return` statement, it returns `Nil` - which by the way doesn't have its own specific keyword. The easiest way to get Nil is by using `tu::getNil()` module method.
You also are allowed to nest function definitions:
```js
import io;

fun outer(a, b) {
  fun inner(c) {
    return a + b + c;
  }

  return inner(5);
}

io::println(outer(2, 3));
```
This outputs `10`.

## 📖 Naming
The name `oko-lang` (or rather `oko`) was chosen for no specific reason by me, tixonochek. Whether it was the best possible choice at the time or not, the name `oko` allowed me to create a fitting logotype and slogan for the language.

## 🧑‍⚖️ License
**oko-lang** uses the **GNU General Public License version 3**. For more information, check the `LICENSE` file of this repository or use the `oko license` command (the first method is preffered).
