# A Rusty MOP

An experiment in creating a flexible dynamic object system with a
clear metaobject protocol. Right now this is in an extremely
early/experimental state. I'm attempting to translate the design and
code from the [_The Art of the Metaobject Protocol_][mop-book]. As that
code was designed for and written in Common Lisp, the differences
are... substantial. This project is similar in spirit to the GObject
and GObject Introspection projects. Eventually may be a great basis
for providing a tool that can generate high-quality bindings to Rust
code for higher-level languages.


[mop-book]: https://mitpress.mit.edu/books/art-metaobject-protocol

# License

Copyright 2017-2018 Geoff Shannon

This file is part of Rust MOP.

Rust MOP is free software: you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the
Free Software Foundation, either version 3 of the License, or (at your
option) any later version.

Rust MOP is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
