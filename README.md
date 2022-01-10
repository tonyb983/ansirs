# Ansirs (Like Answers I Guess?)
Simple and probably flawed little library to make simple usage of ansi color codes super easy when working with rust.

I tend to make a lot of shitty little terminal applications and I got sick of googling for ANSI color codes, or installing some huge or unweildy package to handle it, so I decided to make my own stupid and/or unweildy crate!

Usage is as simple as I could make it because I'm pretty dumb and I wanted to make this as easy as possible on future-me.

```rust
use ansirs::{Ansi, IntoAnsi, style_text};

let header_style = Ansi::new()
    .fg((25, 50, 250))      // Set foreground color to (25, 50, 250)
    .bg((255, 255, 255))    // Set background to black.
    .bold()                 // Set (toggle) bolded text.
    .underline();            // Set (toggle) underlined text.
let body_style = Ansi::new()
    .fg((50, 200, 50))
    .bg((255, 255, 255));
let mistake_style = Ansi::new()
    .fg((200, 25, 25))
    .bg((255, 255, 255))
    .strike()               // Set (toggle) strike-through.
    .italic();              // Set (toggle) italic text.

// Output can be saved (it is just a String)
let header = style_text("Ansirs Crate", header_style);
println!("{}", header);
// Or you can use style_text directly in println.
println!("{}", style_text("Simple and probably flawed library for dealing with ANSI color codes in rust!", body_style));
// You can also use the Ansi directly, but must remember to reset the style afterwards.
println!("{}Definitely an awesome crate.{}", mistake_style, Ansi::reset());
```

`style_text` can also take a lambda to generate styles on the fly. The lambda should match the function signature `Fn() -> Ansi`

```rust
// Same output as above, but without the locals variables. Keep in mind this makes reusing styles more difficult.
use ansirs::{Ansi, IntoAnsi, style_text};

println!("{}", style_text("Ansirs Crate", || Ansi::new()
    .fg((25, 50, 250))
    .bg((255, 255, 255))
    .bold()
    .underline());
println!("{}", style_text("Simple and probably flawed library for dealing with ANSI color codes in rust!", || Ansi::new()
    .fg((50, 200, 50))
    .bg((255, 255, 255))));
println!("{}", style_text("Definitely an awesome crate.", Ansi::new()
    .fg((200, 25, 25))
    .bg((255, 255, 255))
    .strike()
    .italic()));
```

## Todo
- [ ] Add coverage, lint, and maybe packaging / publishing gh actions
- [ ] Find whatever mistakes that exist in this crate.
- [ ] Expand tests
- [ ] Expand functionality?
- [ ] `Styled` trait for designating default styling for certain types. I'm thinking something along the lines of the std::fmt family of functions, i.e. a user-defined "builder" type function is written which has access to the instance and some sort of default styling.
- [ ] Along with above, could maybe move into "themes" as well?

<!--
 Copyright (c) 2022 Tony Barbitta
 
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->
