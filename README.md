# `vdom-rsjs`

A pair of Rust crate and npm package that allows rendering to a virtual DOM tree
in Rust, then sending this to JavaScript to update the real DOM.

The Rust crate includes:

 * An equivalent set of structs and enums to the [`virtual-dom`][] npm package.
 * (Very soon) The same diff algorithm as `virtual-dom` to generate minimal
   patches in Rust to send to JS.
 * (Maybe at some point) The ability to render to an HTML string.

The npm package includes:

 * Some conversion functions that take in the objects that the Rust crate
   generates via `serde_json` and creates the `virtual-dom` equivalents.

## Actions

Rather than allowing function handlers to be attached to nodes the `vdom-rsjs`
types are generic over an action type , this should be a JSON serializable type.
Any event property (e.g. `onclick`) can have a `VProperty::Action` specified
with an instance of this action type, then on the JS side you can pass a
function in when converting the objects to `virtual-dom` which will be called on
the event with the action instance.

See the examples for more details.

## Examples

### WebSocket

This is the canonical virtual DOM example of a counter with increment and
decrement buttons. When the buttons are clicked these push actions over a
websocket to the backend, then the state change is handled server side and the
updated virtual DOM tree is sent back to be rendered to the screen (soon this
will just send the patches back).

This is implemented as a separate frontend and backend for build process
simplicity, to build and start the backend run

```sh
$ cargo run --example websocket
```

then to build and start the frontend run

```sh
$ cd examples/websocket
$ npm install
$ npm start
```

[`virtual-dom`]: https://npmjs.com/package/virtual-dom
