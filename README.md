# Bevy Rectray

[![Crates.io](https://img.shields.io/crates/v/bevy_rectray.svg)](https://crates.io/crates/bevy_rectray)
[![Docs](https://docs.rs/bevy_rectray/badge.svg)](https://docs.rs/bevy_rectray/latest/bevy_rectray/)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/book/plugin-development/)

Bevy Rectray is a native component based 2D and UI solution for the bevy engine.

## Getting Started

First add the `RectrayPlugin`:

```rust
app.add_plugins(RectrayPlugin)
```

Import the
[DSL prelude](https://docs.rs/bevy_rectray/latest/bevy_rectray/dsl/prelude/),
preferably inside the function scope.

```rust
fn spawn(mut commands: RCommands) {
    use bevy_rectray::dsl::prelude::*;
    ...
}
```

Create a sprite:

```rust
sprite!(commands {
    sprite: "Ferris.png",
    anchor: Left,
    offset: [40, 0],
    dimension: [200, 200],
})
```

This spawns a "Ferris.png" to the center left of the screen,
moved to the right by `40 px`, with dimension `200 px * 200 px`,
and returns an `Entity`.

Create a hierarchy:

```rust
vstack!(commands {
    font_size: em(2),
    child: text! {
        text: "Hello"
    },
    child: text! {
        text: "rust",
        child: text! {
            text: "and bevy!"
        },
    },
});
```

### Why a DSL?

`bevy_rectray`'s DSL is a very simple `macro_rules` macro that reorganizes arguments in a way to make
`commands` usable anywhere during the macro invocation, without the need to bend over backwards
to create children/bundles before parents. As a result we have a simple rust-like syntax.
that is intuitive and enjoyable to write.

If you don't like the DSL you can use our
[`bundles`](https://docs.rs/bevy_rectray/latest/bevy_rectray/bundles)
or
[`widgets`](https://docs.rs/bevy_rectray/latest/bevy_rectray/dsl/builders)
directly,

## How this works?

`bevy_rectray` is all about rectangles!

Each sprite is a rectangle, and placed relative to the parent
rectangle.

You might want to

```text
Place a sprite to the center right of the parent sprite,
move left by 10 px,
with 20% of parent's width as width
2x font size as height
and rotate by 45 degrees.
```

In `bevy_rectray` this is incredibly simple:

```rust
sprite!(commands {
    anchor: Right,
    offset: [-10, 0],
    dimension: size2!(20 %, 2 em),
    rotation: degrees(45),
    ...
})
```

Use `Transform2D` and `Dimension` to manipulate our widgets directly.

## What `bevy_rectray` provides

* Fine grained low level anchor-offset layout system.
* First class support for rotation and scaling.
* Simple and intuitive containers.
* Decentralized ECS components with no central state.
* Complete support of bevy's 2D primitives.
* Input handling system for mouse and cursor.
* Building blocks for most common widgets.
* Event handling through one-shot systems.
* Reactivity and animation through signals.
* `macro_rules` based DSL that annihilates boilerplate.
* Easy integration with third-party 2D crates.
* Easy migration to future bevy versions.

## What `bevy_rectray` is not

* Not a renderer.

    `bevy_rectray` has minimal rendering features and no third party bevy dependencies,
    this ensures maintainability and easy migration to future bevy versions,
    at the cost of not having out of the box widget styles.

* Not `bevy_ui` compatible.

    `bevy_rectray` is not dependent on `bevy_ui` in any way. This means `bevy_ui` exclusive
    features won't be available in `bevy_rectray` as is.

* No ui script or serialization.

    `bevy_rectray` uses rust closures for a lot of things, including events and reactivity,
    those are unfortunately not serializable.

* Limited reflection support.

    Limiting the scope of this project to the supported feature set of `Reflection` is not ideal of this project. Use reflect to some extent to
    debug is supported, but don't expect every reflect based feature to work with `bevy_rectray`.

* No styling

    Styling is outside the scope of this crate.

## Container

Anchor-Offset offers fine-grained control over the layout, but you can surrender
that control to
[containers](https://docs.rs/bevy_rectray/latest/bevy_rectray/layout) for ergonomics.

The `Container` is a very simple layout system that
only depends on insertion order of its children. You can find your
[`hstack`](https://docs.rs/bevy_rectray/latest/bevy_rectray/layout/struct.StackLayout.html),
[`grid`](https://docs.rs/bevy_rectray/latest/bevy_rectray/layout/struct.FixedGridLayout.html)
or
[`paragraph`](https://docs.rs/bevy_rectray/latest/bevy_rectray/layout/struct.ParagraphLayout) here.

You can implement [`Layout`](https://docs.rs/bevy_rectray/latest/bevy_rectray/layout/trait.Layout) yourself to create a custom layout.

## Widget Abstractions

Widget builders are used to empower our DSL.
Widget builders implements [`Widget`](https://docs.rs/bevy_rectray/latest/bevy_rectray/dsl/trait.Widget)
and `Default`.
They can be used in general like so:

```rust
FrameBuilder {
    offset: [121, 423].dinto(),
    anchor: Center.dinto(),
    color: color!(red).dinto()
    ..Default::default()
}.build(commands)
```

This returns an `Entity`.

`dinto` is implemented in `DslFrom` or `DslInto`.
which gives us nice conversion like `[i32; 2] -> Vec2`, which can save us a lot of typing!

When using the dsl macro, this becomes

```rust
frame! (commands {
    offset: [121, 423],
    anchor: Center,
    color: color!(red),
});
```

much cleaner, right?

## DSL Syntax

The DSL have a few special fields that makes it much more powerful than
a simple struct constructor.

### commands

At the root level, the DSL takes a
[`RCommands`](https://docs.rs/bevy_rectray/latest/bevy_rectray/dsl/struct.RCommands),
which is a combination of `Commands`, `AssetServer` and
[`SignalPool`](https://docs.rs/bevy_rectray/latest/bevy_rectray/signals/struct.SignalPool),

### child

`child:` is a special field that can be repeated, it accepts an `Entity`, `Option<Entity>`
or an iterator of `Entity`/`&Entity`,
and inserts it/them as a child/children.

```rust
frame! (commands {
    ...
    child: rectangle! {
        dimension: [40, 40]
    },
    child: text! {
        text: "Hello, World!!"
    },
    child: "Hello".chars().map(|c|
        text! (commands {
            text: c,
        })
    )
});
```

This syntax, notice the use of braces `{}`,

```rust
field: macro! { .. },
```

Will be automatically rewritten as

```rust
field: macro!(commands { .. }),
```

Which serves as context propagation.

### extra

Extra adds a component or a bundle to a widget,
which is the idiomatic pattern to compose behaviors.

```rust
// Example: Add dragging support to a `Sprite`.
sprite! (commands {
    ...
    extra: DragX,
    extra: DragConstraint,
    extra: DragSnapBack,
});
```

### entity

`entity` lets us fetch the `Entity`
directly from a nested macro invocation.

```rust
let sprite_entity: Entity;
sprite! (commands {
    child: sprite! {
        entity: sprite_entity,
    }
});
```

### `quote!` syntax

We have support for a syntax inspired by the `quote!` crate,
that can be used to repeat a child by an iterator.

```rust
vstack! (commands {
    child: #rectangle! {
        dimension: #dimensions,
        color: #colors,
    }
});
```

This zips `colors` and `dimensions` and
iterate through them to create multiple rectangles.

#### Note

The dsl normally functions on the field level, which is
performant and editor friendly, but using `quote!` syntax
requires running a `tt` muncher, which may cause editors to give up
or break your recursion limit. You can use

```rust
#![recursion_limit="256"]
```

to increase your recursion limit.

## Next Steps

Checkout our modules for more documentations and examples.

* [events](https://docs.rs/bevy_rectray/latest/bevy_rectray/events)
* [signals](https://docs.rs/bevy_rectray/latest/bevy_rectray/signals)
* [widgets](https://docs.rs/bevy_rectray/latest/bevy_rectray/widgets)
* [animation](https://docs.rs/bevy_rectray/latest/bevy_rectray/anim)

## License

License under either of

Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Contributions are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
