# Kuro (Kuro-lang)

**Kuro** je minimalistický, experimentální programovací jazyk a virtuální stroj (VM) napsaný v Rustu. Původně vznikl jako osobní projekt a kombinuje jednoduchou syntaxi inspirovanou Pythonem s nekompromisní rychlostí kompilovaného jazyka.

## ✨ Hlavní funkce

- **Dva režimy spouštění:**
  - **JIT (Run):** Okamžité spuštění skriptu přes vlastní stack-based Virtual Machine. Ideální pro rychlé testování.
  - **AOT (Build):** Překlad kódu Kuro přímo do nativního Rust kódu a následná kompilace do rychlé, samostatné binárky pomocí `rustc -O`.
- **Jednoduchá syntaxe:** Čistý kód bez zbytečných závorek, odsazování bloků (indentation), zabudované operace pro vstup/výstup a formátování stringů.
- **Lehký VM:** Stack-based (zásobníkový) interpret s izolovanou pamětí pro čísla (`mem_int`) a texty (`mem_str`).

## 🚀 Ukázka kódu (`test.kr`)

```python
# Tohle je globalni komentar
<import src="math.kr" />

<fn main()>:
    let user_input = input("Zadej sve jmeno: ") # Program ted POCKA!
    let mut sum = 0

    # Kdyz zmacknes Enter, pusti se pocitani...
    for i in 1..20000000:
        sum += 1

    print("\n>>> Dopsal jsi! Cyklus se spocital a jedeme dal.")
    print(f"Ahoj {user_input}, tajne cislo je {sum}")
🛠️ Instalace a sestavení
Kuro je napsaný v Rustu, takže pro sestavení budeš potřebovat Cargo.

# Naklonování repozitáře
git clone https://github.com/grulgalus/Kuro-lang.git
cd Kuro-lang

# Sestavení samotného enginu/kompilátoru
cargo build --release
🎮 Použití
Kompilátor přijímá z terminálu dva hlavní příkazy. (Předpokládáme, že binárka se jmenuje kuro).

# 1. Spuštění kódu v interpretu (JIT)
./target/release/kuro run skript.kr

# 2. Zkompilování do nativní binárky (AOT)
./target/release/kuro build skript.kr
Při použití příkazu build Kuro vygeneruje dočasný soubor .rs, automaticky zavolá rustc -O a vytvoří super-rychlý spustitelný soubor (bez nutnosti instalovat Kuro na cílovém PC)!

🧠 Architektura projektu
Projekt je rozdělený do 4 hlavních modulů:

compiler.rs – Překládá zdrojový kód do bytecode instrukcí (řeší i for cykly a scope přes stack jumpy).
bytecode.rs – Definuje samotný set instrukcí (např. Push, Load, Store, Add, JmpIfGt).
vm.rs – Běhové prostředí (Virtual Machine), které krokuje bytecode a provádí operace na zásobníku.
main.rs – CLI rozhraní a chytrý generátor kódu pro AOT kompilaci.
```
# 👥 Autoři
Grydldiček (Discord) & Jandysanko (Minecraft) & kenaj1125/Demon_JohnDoe (Roblox) & grulgalus (GitHub) - Tvůrce a hlavní designér.
Opera AI - Spolupráce na generování kódu a návrhu architektury.
Cloude - Rozšiřuje projekt.

# 📄 Licence
Tento projekt je licencován pod licencí MIT.
