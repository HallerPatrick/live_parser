# Liva specs

This module contains a parser, which is able to parse the liva programming language.

For now this means we generate a intermediate representation, generated with the help of `nom`,
 so this is kind of a mix between a lexer and parser.

Lua directly emits the opcodes to the VM while parsing. We dont do this cause we like the
decoupling of the parser from the VM. When perfomance become critical/present, this may change.


## What to parse

* [x] Literals:
    * [x] Num
    * [x] String
    * [x] Bool
    * [x] Array
    * [x] Map
    * [x] Nil
    * [x] Variable Names

* [x] Expressions: Are getting lexed, while literals are being parsed.
        -> Complete parsing will be done at compile time
      
* [ ] Declarations:
    * [ ] Classes
    * [ ] Functions
    * [ ] Scopes?

* [ ] Statements:
    * [ ] For loop
    * [ ] While loop
    * [ ] If(/Else)
    * [ ] Assignment
    * [ ] Call
    * [ ] Return
    * [ ] Case-Switch?

* [ ] Keep track of line number and line span
