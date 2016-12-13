# Universally Unique Lexicographically Sortable Identifier

[![Build Status](https://travis-ci.org/mmacedoeu/rulid.rs.svg?branch=master)](https://travis-ci.org/mmacedoeu/rulid.rs)
[![Language (Rust)](https://img.shields.io/badge/powered_by-Rust-blue.svg)](http://www.rust-lang.org/)

UUID can be suboptimal for many uses-cases because:

-   It isn't the most character efficient way of encoding 128 bits of randomness
-   The string format itself is apparently based on the original MAC & time version (UUIDv1 from Wikipedia)
-   It provides no other information than randomness

Instead, herein is proposed ULID:

-   128-bit compatibility with UUID
-   1.21e+24 unique ULIDs per millisecond
-   Lexicographically sortable!
-   Canonically encoded as a 26 character string, as opposed to the 36 character UUID
-   Uses Crockford's base32 for better efficiency and readability (5 bits per character)
-   Case insensitive
-   No special characters (URL safe)

## Implementations in other languages

From the community!

| Language | Author |
| -------- | ------ |
| [Javascript](https://github.com/alizain/ulid) | [alizain](https://github.com/alizain) |
| [Erlang](https://github.com/savonarola/ulid) | [savonarola](https://github.com/savonarola) |
| [Elixir](https://github.com/merongivian/ulid) | [merongivian](https://github.com/merongivian) |
| [Go](https://github.com/imdario/go-ulid) | [imdario](https://github.com/imdario/) |
| [Java](https://github.com/Lewiscowles1986/jULID) | [Lewiscowles1986](https://github.com/Lewiscowles1986) |
| [Julia](https://github.com/ararslan/ULID.jl) | [ararslan](https://github.com/ararslan) |
| [.NET](https://github.com/RobThree/NUlid) | [RobThree](https://github.com/RobThree)
| [.NET](https://github.com/fvilers/ulid.net) | [fvilers](https://github.com/fvilers)
| [PHP](https://github.com/Lewiscowles1986/ulid) | [Lewiscowles1986](https://github.com/Lewiscowles1986) |
| [Python](https://github.com/mdipierro/ulid) | [mdipierro](https://github.com/mdipierro) |
| [Ruby](https://github.com/rafaelsales/ulid) | [rafaelsales](https://github.com/rafaelsales) |

## Specification

Below is the current specification of ULID as implemented in this repository. *Note: the binary format has not been implemented.*

```
 01AN4Z07BY      79KA1307SR9X4MV3

|----------|    |----------------|
 Timestamp          Randomness
  10 chars           16 chars
   48bits             80bits
   base32             base32
```

### Components

**Timestamp**
-   48 bit integer
-   UNIX-time in milliseconds
-   Won't run out of space till the year 10895 AD.

**Randomness**
-   80 bits
-   Cryptographically secure source of randomness, if possible

### Sorting

The left-most character must be sorted first, and the right-most character sorted last (lexical order). The default ASCII character set must be used. Within the same millisecond, sort order is not guaranteed

### Encoding

Crockford's Base32 is used as shown. This alphabet excludes the letters I, L, O, and U to avoid confusion and abuse.

```
0123456789ABCDEFGHJKMNPQRSTVWXYZ
```

### Binary Layout and Byte Order

The components are encoded as 16 octets. Each component is encoded with the Most Significant Byte first (network byte order).

```
0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                      32_bit_uint_time_low                     |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     16_bit_uint_time_high     |       16_bit_uint_random      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       32_bit_uint_random                      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       32_bit_uint_random                      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### String Representation

```
ttttttttttrrrrrrrrrrrrrrrr

where
t is Timestamp
r is Randomness
```

## Prior Art

Partly inspired by:
-   [instagram-engineering](http://instagram-engineering.tumblr.com/post/10853187575/sharding-ids-at-instagram)
-   [firebase](https://firebase.googleblog.com/2015/02/the-2120-ways-to-ensure-unique_68.html)
