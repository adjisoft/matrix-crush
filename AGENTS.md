## 🎯 TUJUAN UTAMA

Refactor struktur proyek agar:

* Konsisten dan modular
* Mudah dipahami dan dikembangkan
* File tidak terlalu besar (≤ 500 LOC ideal)
* Tidak mengubah gameplay, logic, atau behavior

---

## 🧱 ARSITEKTUR WAJIB

Gunakan 3 layer utama:

### 1. CORE (opsional)

Reusable systems (audio, save, dll)

### 2. GAME (LOGIC MURNI)

Tidak boleh tergantung UI / rendering

### 3. APP (SCENES / UI)

Menangani:

* rendering
* input
* transisi scene

---

## 📂 STRUKTUR TARGET

```
src/
├── main.rs
├── bootstrap.rs
├── app.rs
│
├── scenes/
│   ├── menu/
│   ├── gameplay/
│   └── story/
│
├── systems/
│   ├── audio.rs
│   ├── save.rs
│   ├── localization.rs
│   └── effects.rs
│
game/
├── board/
├── entities/
├── progression/
└── story/
```

---

## 🔒 ATURAN KERAS (MANDATORY)

### 1. ❌ DILARANG MENGUBAH GAMEPLAY

* Tidak boleh mengubah:

  * mekanik match
  * damage / score
  * level logic
* Hanya refactor struktur & organisasi kode

---

### 2. 📏 BATAS UKURAN FILE

| Tipe File | Maksimal             |
| --------- | -------------------- |
| Normal    | 300–500 LOC          |
| Kompleks  | 700 LOC (hard limit) |

Jika melebihi:
➡️ WAJIB dipecah

---

### 3. 🧩 SINGLE RESPONSIBILITY

Setiap file hanya boleh:

* 1 tanggung jawab utama

Contoh:

* ❌ `level_selection.rs` (UI + logic + state)
* ✅ pecah jadi:

  * `ui.rs`
  * `state.rs`
  * `input.rs`

---

### 4. 🚫 DILARANG CAMPUR LAYER

| Layer   | Boleh | Tidak boleh |
| ------- | ----- | ----------- |
| game    | logic | rendering   |
| scenes  | UI    | core logic  |
| systems | util  | gameplay    |

---

## 🔄 STRATEGI REFACTOR

### STEP 1 — Pindahkan Logic ke `game/`

Contoh:

* `matrix_match/` → `game/board/`
* `level.rs` → `game/progression/`

---

### STEP 2 — Ubah `layout/` → `scenes/`

Mapping:

| Lama                | Baru                     |
| ------------------- | ------------------------ |
| layout/main_menu.rs | scenes/menu/main_menu.rs |
| layout/board        | scenes/gameplay          |

---

### STEP 3 — Pecah File Besar

#### Contoh: board logic

```
board/
├── matcher.rs
├── resolver.rs
├── cascade.rs
└── spawn.rs
```

---

### STEP 4 — Buat `systems/`

Semua util masuk sini:

* audio
* savegame
* i18n
* effects global

---

### STEP 5 — Sederhanakan `main.rs`

```rust
fn main() {
    bootstrap::run();
}
```

---

## 🧠 POLA CODING WAJIB

### 1. STATE TERPISAH

```rust
struct LevelSelectState {
    selected: usize,
}
```

---

### 2. UI HANYA RENDER

```rust
fn render(state: &State) {}
```

---

### 3. LOGIC PURE FUNCTION

```rust
fn match_tiles(board: &Board) -> Matches
```

---

## 🧹 CLEANUP WAJIB

### 1. HAPUS DUPLIKASI ASSET

❌ DILARANG:

```
assets/
games/.../assets/
```

✅ GUNAKAN:

```
assets/ (single source of truth)
```

---

### 2. NAMA FILE HARUS JELAS

| Buruk    | Baik       |
| -------- | ---------- |
| logic.rs | matcher.rs |
| utils.rs | save.rs    |
| stuff.rs | audio.rs   |

---

### 3. MOD.RS HARUS MINIMAL

Hanya re-export:

```rust
pub mod matcher;
pub mod resolver;
```

---

## ⚙️ BUILD OPTIMIZATION (WAJIB)

Tambahkan:

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
```

---

## 🧪 VALIDASI SETELAH REFACTOR

Agent WAJIB memastikan:

* [ ] Game tetap bisa dijalankan
* [ ] Tidak ada perubahan gameplay
* [ ] Tidak ada panic/error baru
* [ ] Semua module terhubung benar
* [ ] Tidak ada file > 700 LOC

---

## 🚀 GOAL AKHIR

Setelah refactor:

* Struktur mudah dipahami dalam < 5 menit
* AI agent bisa navigasi tanpa kebingungan
* Mudah ditambah:

  * level baru
  * UI baru
  * fitur baru

---

## 🧭 PRIORITAS EKSEKUSI

1. Pecah file besar
2. Pisahkan game logic
3. Rapikan scenes
4. Bersihkan assets
5. Optimasi build

---

## ⚠️ CATATAN

* Refactor ≠ rewrite
* Jangan over-engineering
* Fokus ke keterbacaan & konsistensi

---