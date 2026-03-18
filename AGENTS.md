# Matrix Crushed! (Remaster Upgrade)

## 🎯 OBJECTIVE

Upgrade game lama **Matrix Crush** menjadi **Matrix Crushed! (Remastered Edition)** dengan:

* Visual modern (particle, glitch effect)
* Story Mode (25 level + branching)
* Endless Mode
* Power-up system baru
* Currency & research tree
* Bug fixes & UI improvements

⚠️ PRIORITY:

> Deliver **playable build secepat mungkin**, lalu iterate.

---

## 🧱 EXISTING STRUCTURE (DO NOT BREAK)

```
src/
│   audio.rs
│   level.rs
│   level_selection.rs
│   main.rs
│   savegame.rs
│
├── effects/
├── layout/
│   └── board/
└── matrix_match/
```

👉 Gunakan struktur ini sebagai base, **hindari rewrite total**.

---

## ⚙️ GLOBAL RULES

* Gunakan Rust + Macroquad
* Jangan overengineering
* Semua fitur harus:

  * modular
  * data-driven (prefer JSON/XML-like struct)
* Gunakan naming baru:

  * Bomb Gem → X Bomb
  * Sweep Gem → V Sweep

---

## 🚀 PHASE 1 — CRITICAL FIX & FOUNDATION

### 🐞 Bug Fix

* Fix power gem auto-trigger saat start:

  * validasi state awal = inactive
  * reset state saat board generate

### 🧼 Cleanup

* Rapikan module:

  * pisahkan logic vs render
* Tambahkan logging sederhana

---

## 🎨 PHASE 2 — VISUAL SYSTEM

### ✨ Particle System (effects/particles.rs)

Implement:

* Explosion effect
* Combo streak glow
* Power activation
* Glitch distortion

Requirements:

* Gunakan pooling (no allocation tiap frame)
* Bisa dipanggil dari board logic

---

### ⚡ Screen Effects (effects/)

Tambahkan:

* screen shake (sudah ada → improve)
* glitch shader (simple)
* flash effect saat combo tinggi

---

## 🧩 PHASE 3 — CORE GAME EXPANSION

### 💣 Power System Upgrade (matrix_match/gem.rs)

Tambahkan:

* Glitch Bomb
* Antimatter Bomb
* Void Bomb (secret)

Refactor:

* gunakan enum:

```rust
enum PowerType {
    XBomb,
    VSweep,
    GlitchBomb,
    AntimatterBomb,
    VoidBomb,
}
```

---

## 💰 PHASE 4 — CURRENCY SYSTEM

Tambahkan:

```rust
struct Currency {
    data_core: u32,
    entropy: u32,
}
```

Logic:

* Data Core → dari match biasa
* Entropy → dari combo tinggi / glitch event

Integrasi ke:

* savegame.rs
* reward system

---

## 🌳 PHASE 5 — RESEARCH TREE

Buat module baru:

```
src/research/
    mod.rs
    tree.rs
```

Fitur:

* unlock power baru
* upgrade multiplier
* cost DT + Entropy

---

## 📖 PHASE 6 — STORY MODE (IMPORTANT)

Buat module baru:

```
src/story/
    mod.rs
    dialogue.rs
    runner.rs
```

### Format data:

Gunakan struktur sederhana:

```rust
struct Dialogue {
    text: String,
    choices: Vec<Choice>,
}
```

### Implement:

* 25 level story
* branching choice
* 3 ending:

  * Stability
  * Glitch
  * Void

### Visual:

* gunakan ASCII face:

  * (0_0)
  * (x_x)
  * (   )

---

## ♾️ PHASE 7 — ENDLESS MODE

Unlock condition:

* selesai story

Fitur:

* scaling difficulty
* random power spawn
* entropy farming

---

## ⚔️ PHASE 8 — MATCH BATTLE

Tambahkan mode baru:

```
src/battle/
```

Fitur:

* player vs AI
* combo → damage
* basic AI logic

---

## 🧩 PHASE 9 — UI IMPROVEMENT

Perbaiki:

* text rusak (UTF-8)
* layout scaling
* button clarity

Tambahkan:

* currency display
* research tree UI
* story dialogue UI

---

## 🔄 PHASE 10 — RENAMING

Update seluruh code:

* Bomb → X Bomb
* Sweep → V Sweep
* Game title → Matrix Crushed!

---

## 🧪 PHASE 11 — TESTING

Tambahkan:

* debug tools:

  * spawn power
  * skip level
* basic assertions

---

## 🧠 AI EXECUTION STRATEGY

Kerjakan berurutan:

1. Fix bug + stabilize
2. Tambah particle & efek
3. Refactor power system
4. Tambah currency
5. Implement story (minimal dulu)
6. Endless mode
7. Polish UI

⚠️ Jangan implement semua sekaligus.

---

## ✅ DEFINITION OF DONE

Game dianggap selesai jika:

* Bisa dimainkan dari start → story → ending
* Endless mode unlock
* Semua power bekerja
* Tidak ada crash
* Visual terasa “hidup”

---

## 🧩 OPTIONAL (IF TIME)

* glitch event system
* achievement system
* shader lebih advanced

---

## 💬 FINAL NOTE

Fokus:

> “Make it playable first, perfect later.”

Jika ragu:

* pilih solusi paling sederhana
* hindari abstraction berlebihan

---

END.