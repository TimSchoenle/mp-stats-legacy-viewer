# Data Storage Architecture

## Directory Structure

```
/data
└── /[edition]
    ├── /meta/map.json
    ├── /dictionary/ids/[Chunk_ID].json
    ├── /names.tar.xz
    ├── /players/[Shard_ID].json.xz
    └── /leaderboards/[board]/[game]/[stat]/
        ├── /latest/chunk_XXXX.bin.xz
        └── history.tar.xz
```

---

## Binary & Schema Specifications

### 1. Leaderboard Binary Arrays (`chunk_XXXX.bin.xz`)

Leaderboard chunks are completely devoid of strings, JSON formatting, or file headers. They are raw memory buffers of integers, compressed using maximum-preset LZMA.

**Specifications:**
- **File Format:** Raw Binary Buffer (LZMA Compressed, `.xz`)
- **Data Type:** 64-bit Signed Integers (`int64_t`)
- **Byte Order:** Big Endianness
- **Chunk Limit:** 10,000 players

#### Byte Layout

The data is a strictly alternating sequence of `player_id` and `score`.

| Byte Offset | Size (Bytes) | Data Type | Value      | Description                  |
|-------------|--------------|-----------|------------|------------------------------|
| `0x00`      | 8            | `int64_t` | player_id  | The integer ID of the 1st player |
| `0x08`      | 8            | `int64_t` | score      | The score of the 1st player  |
| `0x10`      | 8            | `int64_t` | player_id  | The integer ID of the 2nd player |
| `0x18`      | 8            | `int64_t` | score      | The score of the 2nd player  |
| ...         | ...          | ...       | ...        | ...                          |

#### Implicit Ranking

Ranks are not stored in the binary buffer. Rank is calculated mathematically based on the chunk's filename index and the integer's position in the buffer.

**Formula:**
```
Rank = (Chunk_Index - 1) × 10,000 + (Array_Index ÷ 2) + 1
```

---

### 2. Player Profile Shards (`[Shard_ID].json.xz`)

Player profiles are LZMA-compressed minified JSON files. To prevent key duplication, the JSON uses a 1D flat integer array with a strict 7-integer stride.

**Specifications:**
- **File Format:** Minified JSON (LZMA Compressed, `.xz`)
- **Shard Math:** `Shard_ID = floor(player_id / 100,000)`
- **Schema:** `{"[player_id_string]": [int, int, int...]}`

#### The 7-Integer Stride Schema

Each stat entry within the flat array occupies 7 consecutive indices.

| Index Offset | Data Type | Value     | Resolution Map / Notes               |
|--------------|-----------|-----------|--------------------------------------|
| `i + 0`      | `int32`   | board_id  | Mapped via `/meta/map.json`          |
| `i + 1`      | `int32`   | game_id   | Mapped via `/meta/map.json`          |
| `i + 2`      | `int32`   | stat_id   | Mapped via `/meta/map.json`          |
| `i + 3`      | `int32`   | save_id   | Internal DB Save ID                  |
| `i + 4`      | `int64`   | score     | The player's score                   |
| `i + 5`      | `int32`   | rank      | The player's global rank for this stat |
| `i + 6`      | `int64`   | timestamp | Unix Epoch timestamp (in seconds)    |

---

### 3. Time-Series History Archives (`history.tar.xz`)

All historical saves for a specific leaderboard track are bundled into a single Solid LZMA Tarball.

**Specifications:**
- **File Format:** Standard POSIX Tarball (Solid LZMA Compressed, `.xz`)
- **Compression Strategy:** Solid Archive. Files inside the tarball are uncompressed. The LZMA algorithm compresses the entire tarball globally, deduplicating the time-series data streams perfectly.

#### Internal File Structure

```
[YYYY-MM-DD_HH-MM-SS]/_meta.json          (Uncompressed JSON)
[YYYY-MM-DD_HH-MM-SS]/chunk_XXXX.bin      (Uncompressed 64-bit Binary Buffer)
```

---

### 4. Name Search Index Archive (`names.tar.xz`)

To prevent inode exhaustion on the host file system, millions of search index pointers are bundled into a single solid archive.

**Specifications:**
- **File Format:** Standard POSIX Tarball (Solid LZMA Compressed, `.xz`)
- **Internal File Structure:** `names/[3-char-prefix]/[Sanitized_Name].csv`
- **CSV Schema:** `player_id,uuid` (Strictly 2 lines: the header, followed by the integer ID and 36-char string UUID)
