# bgpui-kit

A small shared UI layer for GPUI apps. The first prototype defines a theme token contract and three presentation primitives: a status badge, a section label and a surface card.

Each app keeps its own palette JSON, copy, state and actions. Components take theme values and translated text from the host:

```rust
use bgpui_kit::{Status, ThemeTokens, section_label, status_badge, surface_card};

let theme = ThemeTokens::parse(include_str!("../assets/themes/dark.json"))?;
let heading = section_label("Sessions", &theme);
let badge = status_badge("Working", Status::Working, &theme);
let card = surface_card(&theme).child(heading).child(badge);
```

The host must import GPUI's `ParentElement` trait for `.child(...)`. The strings above are placeholders; production apps provide their localized labels. Status badges pair text with color so their meaning remains visible without color.

The token shape matches the current Ginka, e1 and Kirikumo `assets/themes/{dark,light}.json` files. `duration_ms.fade` is optional because Kirikumo currently omits it. Validate palettes with:

```sh
cargo run --example check_themes -- path/to/dark.json path/to/light.json
```

## Sharing across apps

Use one `bgpui-kit` Git revision in each app's `Cargo.toml`. GPUI types from different Git sources or revisions cannot be mixed: the host's `gpui`, `gpui-component` and this crate must resolve to one GPUI instance. This prototype uses the `gpui-component` revision already selected by Ginka, and the same Zed Git source for `gpui`. Its lockfile selects Ginka's GPUI commit for standalone builds. A consuming app's lockfile must select the same commit. An app upgrading the toolkit should update these dependencies together.

Keep product-specific views in each app. Move a component here when its input and behavior can be stated without referring to an app's daemon, data model or navigation. Pedro has not yet been checked against this token contract.

This is an API and theme compatibility prototype. The three primitives have no click actions or focus behavior; interactive controls need keyboard and focus design before they are added.
