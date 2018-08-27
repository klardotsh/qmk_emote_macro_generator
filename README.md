# qmk_emote_macro_generator
Easily generate QMK macro/leader sequences for arbitrary unicode sequences. The
intention here is to be able to use emoticons and emojis in QMK macros (right
now only leader key macros are supported) without having to do all the hex
conversion and ibus config hackery yourself.

`config.toml` should include two dictionaries: emotes, and mappings. `emotes`
should be a key/value pair of a name for the emote without spaces (ex.
`TABLE_FLIP`) and the actual emote as a raw TOML string (single-quoted), ex.
`'(╯°□°）╯︵ ┻━┻')'`. `mappings` should be a key/value pair of one to five key
letter codes to be used as the leader sequence and one of the emote names
defined in `emotes`, ex. `fca = 'FLAG_CA'`.

## Running it
For now, you'll need to change config.toml in this directory to include whatever
you want, and then run `cargo build --release;
./target/release/qmk_emote_macro_generator > emotes.h`, copy `emotes.h` to your
QMK keymap directory, and `#include "emotes.h"` in your `keymap.c`. This is a
little hacky for now, but I'm working on it.

Docker images and AUR packages inbound... eventually.

## Example Config
```toml
[emotes]
# Emojis
BEER = "🍺"
BEER_TOAST = "🍻"
FACE_CUTE_SMILE = "😊"
FACE_HEART_EYES = "😍"
FACE_JOY = "😂"
FACE_SWEAT_SMILE = "😅"
FACE_THINKING = "🤔"
FIRE = "🔥"
FLAG_CA = "🇨🇦"
FLAG_US = "🇺🇸"
HAND_CLAP = "👏"
HAND_HORNS = "🤘"
HAND_OK = "👌"
HAND_THUMB_DOWN = "👎"
HAND_THUMB_UP = "👍"
HAND_WAVE = "👋"
HEART = "❤️"
MAPLE_LEAF = "🍁"
POOP = "💩"
TADA = "🎉"

# Emoticons, but fancier
ANGRY_TABLE_FLIP = "(ノಠ痊ಠ)ノ彡┻━┻"
CELEBRATORY_GLITTER = "+｡:.ﾟヽ(´∀｡)ﾉﾟ.:｡+ﾟﾟ+｡:.ﾟヽ(*´∀)ﾉﾟ.:｡+ﾟ"
SHRUGGIE = "¯\\_(ツ)_/¯"
TABLE_FLIP = "(╯°□°）╯︵ ┻━┻"

[mappings]
br = "BEER"
brt = "BEER_TOAST"

cel = "CELEBRATORY_GLITTER"

fcs = "FACE_CUTE_SMILE"
fhe = "FACE_HEART_EYES"
fj = "FACE_JOY"
fss = "FACE_SWEAT_SMILE"

fir = "FIRE"

fca = "FLAG_CA"
fus = "FLAG_US"

hcl = "HAND_CLAP"
hor = "HAND_HORNS"
ho = "HAND_OK"
htd = "HAND_THUMB_DOWN"
htu = "HAND_THUMB_UP"
hw = "HAND_WAVE"

he = "HEART"
ml = "MAPLE_LEAF"
poo = "POOP"
tad = "TADA"

atf = "ANGRY_TABLE_FLIP"
tf = "TABLE_FLIP"

shr = "SHRUGGIE"
```
