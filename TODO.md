# 🧠 TODO MASTER — Matrix Crushed! (Remastered Edition)

## 🎯 0. Rebranding & Foundation
* [ ] Ubah nama project:
  * `Matrix Crush` → **Matrix Crushed!**
* [ ] Update semua referensi:

  * [ ] Title screen
  * [ ] Metadata (manifest.xml, config, save data)
  * [ ] Asset naming (folder, file)
* [ ] Refactor struktur project:

  * [ ] `/data/ui/`
  * [ ] `/data/effects/`
  * [ ] `/data/story/`
  * [ ] `/data/powerups/`
  * [ ] `/data/research/`

---

## 🎨 1. Visual Upgrade (HIGH PRIORITY)

### ✨ Particle & Effects
* [ ] Tambahkan sistem particle modular:

  * [ ] Explosion (match besar)
  * [ ] Combo streak glow
  * [ ] Power activation effect
  * [ ] Screen glitch effect
* [ ] Shader efek:

  * [ ] Chromatic aberration ringan
  * [ ] Glitch distortion (trigger saat combo tinggi)
* [ ] Optimize:

  * [ ] Batch rendering
  * [ ] Pooling particle system

---

## 🐞 2. Bug Fixes (CRITICAL)

* [ ] Fix bug:

  * ❌ Power gems aktif otomatis saat start
  * ✔️ Validasi:

    * [ ] Check initial state = `inactive`
    * [ ] Reset state saat board generate
* [ ] Tambahkan test:

  * [ ] Unit test power gem state
  * [ ] Edge case: load game / restart

---

## 🧩 3. UI & Localization Fix

* [ ] Perbaiki text rusak:

  * [ ] Encoding UTF-8 semua file
  * [ ] Replace hardcoded string → localization system
* [ ] Sistem bahasa:

  * [ ] `/data/lang/id.xml`
  * [ ] `/data/lang/en.xml`
* [ ] UI polish:

  * [ ] Responsive scaling
  * [ ] Button clarity (hand-drawn style tetap dipertahankan)

---

## 📖 4. Story Mode (CORE FEATURE)

### 🎭 Visual Novel Style
* [ ] Implement Story Mode:

  * [ ] 25 level
  * [ ] Dialog branching system

* [ ] Format data:

  ```xml
  <dialogue id="lvl01">
    <line char="AI">(0_0) Welcome to the system...</line>
    <choice>
      <option next="a">Trust system</option>
      <option next="b">Break system</option>
    </choice>
  </dialogue>
  ```

* [ ] 3 Ending:

  * [ ] Normal Ending
  * [ ] Glitch Ending
  * [ ] Void Ending (secret)

* [ ] Trigger system:

  * [ ] Berdasarkan pilihan + performa gameplay

---

## ♾️ 5. Endless Mode

* [ ] Unlock condition:

  * [ ] Selesaikan Story Mode
* [ ] Scaling:

  * [ ] Enemy speed meningkat
  * [ ] Spawn rate naik
* [ ] Hidden mechanics:

  * [ ] Rare power spawn
  * [ ] Secret bombs unlock

---

## 💣 6. New Power-Ups System

### 💥 Power Concepts
* [ ] Tambahkan power:

  * [ ] **Glitch Bomb**

    * Efek: random tile corruption + chain reaction
  * [ ] **Antimatter Bomb**

    * Efek: wipe area besar
  * [ ] **Void Bomb (secret)**

    * Efek: delete tile permanen

* [ ] Unlock logic:

  * [ ] Research tree
  * [ ] Endless secret drop

---

## 💰 7. Currency System

* [ ] Tambahkan:

  * **Data Core (DT)** → main currency
  * **Glitchy Entropy** → rare currency

* [ ] Drop logic:

  * [ ] DT: dari match & level
  * [ ] Entropy:

    * [ ] combo tinggi
    * [ ] glitch event
    * [ ] endless mode

---

## 🌳 8. Research Tree System

* [ ] Struktur:

  ```xml
  <research id="power_glitch">
    <cost dt="100" entropy="5"/>
    <unlock>glitch_bomb</unlock>
  </research>
  ```

* [ ] Node type:

  * [ ] Power unlock
  * [ ] Passive buff
  * [ ] Combo multiplier

* [ ] UI:

  * [ ] Node graph
  * [ ] Connection lines
  * [ ] Lock/Unlock state

---

## ⚔️ 9. Match Battle Mode

* [ ] PvE / PvAI:

  * [ ] Enemy board logic
* [ ] Features:

  * [ ] Attack by combo
  * [ ] Defense mechanic
* [ ] Future ready:

  * [ ] Multiplayer hook (optional)

---

## 🔄 10. Renaming System Update

* [ ] Rename:

  * `Bomb Gem` → **X Bomb**
  * `Sweep Gem` → **V Sweep**
* [ ] Update:

  * [ ] UI
  * [ ] Code enum
  * [ ] Localization

---

## 🧪 11. Testing & Balancing

* [ ] Playtest:

  * [ ] Story pacing
  * [ ] Power balancing
* [ ] Debug tools:

  * [ ] Spawn power manual
  * [ ] Skip level
* [ ] Metrics:

  * [ ] Avg combo
  * [ ] Currency gain rate

---

## 🚀 12. Polishing & Release

* [ ] Sound FX upgrade
* [ ] Background music (dynamic)
* [ ] Performance optimization
* [ ] Build final:

  * [ ] Windows
  * [ ] Web (optional)

---

# 🧩 BONUS (REKOMENDASI PRO LEVEL)

* [ ] Tambahkan **Glitch Event System**

  * Random dunia berubah (UI distort, rules berubah)
* [ ] Achievement system:

  * "Break the Matrix"
  * "Entropy Master"
  * "Void Whisperer"