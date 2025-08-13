# 📜 Changelog of `oko-lang`
This file is a changelog containing all changes for each respective version of the **oko** programming language. Keep in mind that not all version have their own designated release in the **Releases/** page of this repository - some of the updates are indeed too minor and unimportant to create a separate release for them. 

## Version `1.0`
_(12th August)_
The initial release of **oko-lang**. Nothing of interest.

## Version `1.1`
_(12th August)_
Changes:
1. Fixed major scope bugs, which allows for recursion to actually work
2. `funct` keyword was renamed into `fun`
3. `feach` keyword was renamed into `for`

With the release of this version, the official VSCE also got updated.

## Version `1.2` 
_(13th August)_

Changes:
1. The `io::print` has temporarily been turned off, because Deno makes it very hard to flush the standart output, which is necessary for this module method to properly work. Until a solution will be found, this method will be unavailable.
2. Instead of JS's `NaN` `tu::toNumber()` now returns oko's `Nil` when a String is not numerical.
3. Added a new method to the `time` module: `time::sleep(ms)`. F.e., `time::sleep(2000)` will stop the execution of the program for 2 seconds.
4. Added a separate type of error for a JS-invoked error (No `28`).

## Version `1.21`
_(13th August)_

Changes:
1. Now, `\r` is accounted for on Windows systems. Before this change, oko did not launch on Windows machines.
2. Fixed a critical bug that made it so no `.oko` file could be executed.