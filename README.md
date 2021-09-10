# Liva specs

This module contains a parser, which is able to parse the liva programming language.

For now this means we generate a intermediate representation, generated with the help of `nom`,
 so this is kind of a mix between a lexer and parser.

Lua directly emits the opcodes to the VM while parsing. We dont do this cause we like the
decoupling of the parser from the VM. When perfomance become critical/present, this may change.


## What to parse

* [x] Comments
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
      

* [x] Statements:
    * [x] For loop
    * [x] While loop
    * [x] If(/Else) // TODO: ELSE
    * [x] Declarations:
        * [x] Classes
        * [x] Functions
        * [x] Assignment
    * [x] Return
    * [x] Import 

* [x] Keep track of line number and line span
