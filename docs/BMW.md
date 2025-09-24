# Notes on Reading/Writing BMW

I cannot find a guide online for this format (`.bww`) file extensions, used by
Bagpipe Music Writer Gold and Bagpipe Player. These notes I have taken are based
on looking through many `.bww` files so may be incomplete (such as with grace
notes/embellishments that I could not find in any files)

## Notes

A note contains a pitch, a duration, an optional dot, and an optional bit for
explaining the direction of the beam for connected 8th, 16th, etc.

### Pitch

Pitches for melody notes are in uppercase. They are: `LG`, `LA`, `B`, `C`, `D`,
`E`, `F`, `HG`, `HA`

Pitches used in dots and embellishments are the same but in lowercase. They are:
`lg`, `la`, `b`, `c`, `d`, `e`, `f`, `hg`, `ha`

### Duration

Durations are numbers based on note values: `1` (Whole note), `2` (half note),
`4` (quarter note), `8` (eighth note), `16` (sixteenth note), `32`
(thirty-second note). They are provided after the note (and optional beaming)
and an underscore.

Example: a quarter note on D: `D_4`

### Dots

Dots must be manually placed in the place corresponding to their note. They use
a `'` followed by the lowercase representation of the pitch.

Example: a dotted quarter on low a: `LA_4 'la`

### Ties

Ties must be placed manually in the place corresponding to their note. They use
a `^t` followed by the lowercase represenation of the pitch.

Example: a quarter note on D tied to an eighth note on D: `D_4 ^td D_8`

### Beaming

Beaming is given by an `l` or `r` to denote which direction the beam is facing.
It comes after the pitch and before the note duration. The first note of a group
will have an `r` to denote that its beam connects it to a note to its right, and
the rest of the beamed notes in a group will have `l`.

Example: an 8th note on D, followed by a 16th note on D, followed by a 16th note
on F: `Dr_8 Er_16 Fl_16`

## Grace Notes and Embellishments

These are unique in that they are all lowercase with no other punctuation.

### Single Gracenotes/Strikes

On the page, these grace notes and "strikes" (aka "taps") put a single grace
note on the intended space, and look identical. The difference is in the audio
output, since, for example, an e strike from f would sound different than an e
grace note from low a.

I plan to accept both of these when parsing the file, but
only output the grace note versions (for now), since the validation of knowing
when to use a grace note vs a strike will require a significant amount of logic.

| Note           | BMW Grace Note | BMW Strike |
| -------------- | -------------- | ---------- |
| low g          | unimplemented  | `strlg`    |
| low a          | `ag`           | `strla`    |
| b              | `bg`           | `strb`     |
| c              | `cg`           | `strc`     |
| d              | `dg`           | `strd`     |
| e              | `eg`           | `stre`     |
| f              | `fg`           | `strf`     |
| high g         | `gg`           | `strhg`    |
| high a (thumb) | `tg`           | `strha`    |

### Doublings

Standard doublings, low g - high a, are given with `db` followed by their
respective note.

Thumb doublings and half doublings similarly use `tdb` and
`hdb`.

Note that high g and high a half doublings and thumb doublings are
unimplemented since these could either be referred to interchangeably with their
standard doublings, or could be considered impossible to perform.

| Note   | Doubling | Thumb Doubling | Half Doubling |
| ------ | -------- | -------------- | ------------- |
| low g  | `dblg`   | `tdblg`        | `hdblg`       |
| low a  | `dbla`   | `tdbla`        | `hdbla`       |
| b      | `dbb`    | `tdbb`         | `hdbb`        |
| c      | `dbc`    | `tdbc`         | `hdbc`        |
| d      | `dbd`    | `tdbd`         | `hdbd`        |
| e      | `dbe`    | `tdbe`         | `hdbe`        |
| f      | `dbf`    | `tdbf`         | `hdbf`        |
| high g | `dbhg`   | unimplemented  | unimplemented |
| high a | `dbha`   | unimplemented  | unimplemented |

### Grace Note Strikes/Slurs/Shakes

BMW refers to "slurs" (sometimes also called shakes, for example a g grace note
to d followed by a tap on low g), as "grace note strikes." They use the format
`gstb` for "g strike on b."

The thumb version (with high a grace note instead of g grace note) is similarly
`tstb`, and the half version (with no leading grace note) is `hstb`.

Note: `lgstd` is for ***light*** g gracenote strike on d, ***not*** "low g
strike on d." The "heavy" version is simply `gstd`.

| Note      | Slur          | Thumb Slur    | Half Slur     |
| --------- | ------------- | ------------- | ------------- |
| low g     | unimplemented | unimplemented | unimplemented |
| low a     | `gstla`       | `tstla`       | `hstla`       |
| b         | `gstb`        | `tstb`        | `hstb`        |
| c         | `gstc`        | `tstc`        | `hstc`        |
| d (heavy) | `gstd`        | `tstd`        | `hstd`        |
| d (light) | `lgstd`       | `ltstd`       | `lhstd`       |
| e         | `gste`        | `tste`        | `hste`        |
| f         | `gstf`        | `tstf`        | `hstf`        |
| high h    | unimplemented | `tsthg`       | `hsthg`       |
| high a    | unimplemented | unimplemented | unimplemented |

### Hornpipe Shakes/ Peles

An example of this would be a D doubling followed by a low g strike strike, like
on C in pumpkin's fancy. Prepend `pel` with the appropriate lowercase pitch
note, like `pelb` for "pele on b."

For thumb version (with high a grace note instead of g grace note) is similarly
`tpelb`, and the half version (with no leading grace note) is `hpelb`

Note: `lpeld` is for ***light*** g gracenote strike on d, ***not*** "low g
strike on d." The "heavy" version is simply `peld`.

| Note      | Hornpipe Shake | Thumb Hornpipe Shake | Half Hornpipe Shake |
| --------- | -------------- | -------------------- |
| low g     | unimplemented  | unimplemented        | unimplemented       |
| low a     | `pella`        | `tpella`             | `hpella`            |
| b         | `pelb`         | `tpelb`              | `hpelb`             |
| c         | `pelc`         | `tpelc`              | `hpelc`             |
| d (heavy) | `peld`         | `tpeld`              | `hpeld`             |
| d (light) | `lpeld`        | `ltpeld`             | `lhpeld`            |
| e         | `pele`         | `tpele`              | `hpele`             |
| f         | `pelf`         | `tpelf`              | `hpelf`             |
| high g    | unimplemented  | `tpelhg`             | `hpelhg`            |
| high a    | unimplemented  | unimplemented        | unimplemented       |

### Birls

| Embellishment      | BMW   |
| ------------------ | ----- |
| Birl w/ g g.n.     | `gbr` |
| Birl from a        | `brl` |
| Birl w/ low a      | `abr` |
| Birl w/ thumb g.n. | `tbr` |

### D throws

| Embellishment   | BMW      |
| --------------- | -------- |
| D throw (light) | `thrd`   |
| D throw (heavy) | `hvthrd` |

### Grips

| Embellishment  | BMW    |
| -------------- | ------ |
| Grip           | `grp`  |
| Grip w/ b g.n. | `grpb` |

### Catches (grace note grips)

A "catch" refers to a g grace note to a theme note, followed by a grip back to
that note. An example would be the g grace note to c followed by grip on c in
the third part of cabar feidh, which would be a "c catch". BMW uses `ggrpc` for
this, for a "g grace note grip on c."

For thumb version (with high a grace note instead of g grace note) is similarly
`tgrpc`, and the half version (with no leading grace note) is `hgrpc`.

Note that the `ggrpf`, `tgrpf`, and `hgrpf` all have an f grace note in between
the low g's, rather than a d grace note. The same holds for `tgrphg` (but not
`hgrphg`?)

| Note             | Catch         | Thumb Catch   | Half Catch    |
| ---------------- | ------------- | ------------- | ------------- |
| low g            | unimplemented | unimplemented | unimplemented |
| low a            | `ggrpla`      | `tgrpla`      | `hgrpla`      |
| b                | `ggrpb`       | `tgrpb`       | `hgrpb`       |
| c                | `ggrpc`       | `tgrpc`       | `hgrpc`       |
| d (d grace note) | `ggrpd`       | `tgrpd`       | `hgrpd`       |
| d (b grace note) | `ggrpdb`      | `tgrpdb`      | `hgrpdb`      |
| e                | `ggrpe`       | `tgrpe`       | `hgrpe`       |
| f                | `ggrpf`       | `tgrpf`       | `hgrpf`       |
| high g           | unimplemented | `tgrphg`      | `hgrphg`      |
| high a           | unimplemented | unimplemented | `hgrpha`      |

### Other Embellishments

| Embellishment       | BMW           |
| ------------------- | ------------- |
| Taorluath           | `taor`        |
| Taorluath w/ b g.n. | `taorb`       |
| Low g taorluath     | unimplemented |
| Crunluath           | `crunl`       |
| Crunluath w/ b g.n. | `crunlb`      |
| Heavy Crunluath     | unimplemented |
| Heavy B Crunluath   | unimplemented |
| Edre                | `edre`        |
| Dare                | `dare`        |
| Chedari             | unimplemented |
| Embari              | unimplemented |
| Darodo              | `bubly`       |
| Darodo (from low g) | `hbubly`      |

## Constructing a measure

Measures start with `!`, then have the various grace notes and theme notes.

### Pickup measures

There is no measure validation for correct # of beats, so you can just include a
measure of whatever length before the first full measure starts

## Constructing a line

Each line starts with `& sharpf sharpc` (with an optional time signature if it's
the first part). The `&` creates a treble clef ("g clef").

 and ends with either `!t` (line break?), `!I` (final bar line?),
or `''!I` (repeat).

### Time signatures

`2_4`, `6_8`, `4_4`, etc.

## Second Endings

First ending starts with `'1` and ends with `_'` Second ending simialrly starts
with `'2` and ends with `_'`

## Misc

`space` seems to be a keyword

## Constructing a tune

This block is at the top of every tune. Seems to be various metadata used for
display purposes.

```bww
Bagpipe Reader:1.0

MIDINoteMappings,(54,56,58,59,61,63,64,66,68,56,58,60,61,63,65,66,68,70,55,57,59,60,62,64,65,67,69)

FrequencyMappings,(370,415,466,494,554,622,659,740,831,415,466,523,554,622,699,740,831,932,392,440,494,523,587,659,699,784,880)

InstrumentMappings,(71,71,45,33,1000,60,70)

GracenoteDurations,(20,40,30,50,100,200,800,1200,250,250,250,500,200)

FontSizes,(100,100,65,70,300)

TuneTempo,60

TuneFormat,(1,1,F,L,500,500,500,500,L,0,0)

"Scotland The Brave",(T,L,0,0,Times New Roman,16,700,0,0,18,0,0,0)

"March",(Y,C,0,0,Times New Roman,14,400,0,0,18,0,0,0)

"Trad.",(M,R,0,0,Times New Roman,14,400,0,0,18,0,0,0)

"",(F,R,0,0,Times New Roman,10,400,0,0,18,0,0,0)

```
