ได้เลยครับ ถ้าคุณใช้ Rust จริงจัง โดยเฉพาะแนวที่คุณกำลังทำอยู่ เช่น C++/Rust, image processing, ONNX/YOLO, performance สูง ๆ — `cargo` เป็นเครื่องมือที่ควรรู้ค่อนข้างเยอะเลย

คุณใช้:

```powershell
cargo 1.97.1
```

ซึ่งเป็น Cargo รุ่นใหม่มากแล้ว ณ วันที่ 15 ส.ค. 2026

Cargo ไม่ได้มีแค่ `build / run / test` แต่มี command สำหรับ dependency, workspace, profiling, packaging, publishing, documentation, debugging dependency tree ฯลฯ โดยเอกสารทางการแบ่งระบบของ Cargo ครอบคลุม Manifest, Workspaces, Dependencies, Features, Profiles, Configuration, Build Scripts, Registries และอื่น ๆ ด้วย ([Rust Documentation][1])

ผมจะแบ่งให้เป็นกลุ่มตามการใช้งานจริงนะครับ

## 1. Command พื้นฐานที่ต้องใช้ทุกวัน

| Command        | ใช้ทำอะไร                                       |
| -------------- | ----------------------------------------------- |
| `cargo new`    | สร้าง project ใหม่                              |
| `cargo init`   | เปลี่ยน directory ที่มีอยู่ให้เป็น Rust project |
| `cargo build`  | Compile project                                 |
| `cargo run`    | Compile + Run                                   |
| `cargo check`  | ตรวจ compile โดยไม่สร้าง executable เต็ม        |
| `cargo test`   | Run tests                                       |
| `cargo bench`  | Run benchmark                                   |
| `cargo clean`  | ลบ build artifacts                              |
| `cargo fmt`    | Format code                                     |
| `cargo clippy` | ตรวจ code quality / lint                        |
| `cargo doc`    | Generate documentation                          |
| `cargo tree`   | ดู dependency tree                              |
| `cargo update` | Update dependencies                             |
| `cargo fetch`  | Download dependencies                           |

สำหรับการพัฒนาแต่ละวัน ผมมองว่า 10 ตัวที่ควรจำที่สุดคือ:

```powershell
cargo new
cargo build
cargo check
cargo run
cargo test
cargo fmt
cargo clippy
cargo tree
cargo update
cargo clean
```

---

# 2. สร้าง Project

### `cargo new`

สร้าง Rust package ใหม่

```powershell
cargo new myapp
```

ได้ประมาณ:

```text
myapp/
├── Cargo.toml
└── src/
    └── main.rs
```

สร้าง binary:

```powershell
cargo new myapp
```

สร้าง library:

```powershell
cargo new mylib --lib
```

สร้าง project ใน directory ปัจจุบัน:

```powershell
cargo init
```

หรือ library:

```powershell
cargo init --lib
```

---

# 3. Compile

### `cargo build`

Compile แบบ development

```powershell
cargo build
```

ผลลัพธ์อยู่ประมาณ:

```text
target/debug/
```

ถ้าต้องการ Release:

```powershell
cargo build --release
```

ผลลัพธ์:

```text
target/release/
```

สำหรับงาน performance เช่น image processing / YOLO / inference ให้สนใจ:

```powershell
cargo build --release
```

มากกว่า debug build

---

# 4. `cargo check`

อันนี้สำคัญมาก

```powershell
cargo check
```

มันตรวจว่า code compile ได้หรือไม่ แต่ไม่จำเป็นต้องสร้าง executable เต็ม

ดังนั้นเวลาพัฒนา:

```powershell
cargo check
```

มักเร็วกว่า:

```powershell
cargo build
```

Workflow ที่ดี:

```powershell
cargo check
cargo test
cargo build --release
```

---

# 5. Run

```powershell
cargo run
```

ส่ง argument ให้ program:

```powershell
cargo run -- hello world
```

Rust program จะได้รับ:

```text
hello
world
```

Release:

```powershell
cargo run --release
```

กำหนด binary:

```powershell
cargo run --bin myapp
```

กำหนด example:

```powershell
cargo run --example demo
```

Cargo รองรับ `--bin`, `--example`, feature selection และ release/profile options ใน `cargo run` โดยตรง ([Rust Documentation][2])

---

# 6. Build แบบ Release

ตัวนี้คุณควรจำให้ดี:

```powershell
cargo build --release
```

เพราะ:

```text
Debug
↓
compile เร็ว
↓
runtime performance ต่ำกว่า
```

ส่วน:

```text
Release
↓
optimization
↓
runtime performance สูงกว่า
```

สามารถเลือก profile:

```powershell
cargo build --profile release
```

หรือ custom profile:

```powershell
cargo build --profile production
```

ถ้าคุณทำระบบ performance สูง ๆ เรื่อง `[profile.release]` ใน `Cargo.toml` จะสำคัญมาก

เช่น:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

---

# 7. Test

Run test:

```powershell
cargo test
```

Run test แบบเฉพาะชื่อ:

```powershell
cargo test test_name
```

แสดง output:

```powershell
cargo test -- --nocapture
```

Run test เฉพาะ package:

```powershell
cargo test -p mypackage
```

Run integration test:

```powershell
cargo test --test integration_test
```

---

# 8. Benchmark

```powershell
cargo bench
```

โดยปกติจะใช้ร่วมกับ benchmark framework เช่น Criterion

ตัวอย่าง:

```powershell
cargo bench
```

เหมาะกับสิ่งที่คุณสนใจมาก เช่น

```text
Rust vs C++
OpenCV processing
YOLO preprocessing
image resize
JPEG decode
memory copy
Kafka processing
RabbitMQ processing
```

---

# 9. Format

```powershell
cargo fmt
```

ตรวจว่า format ถูกต้องหรือไม่:

```powershell
cargo fmt --check
```

ใน CI นิยม:

```powershell
cargo fmt --check
```

---

# 10. Clippy

```powershell
cargo clippy
```

เข้มขึ้น:

```powershell
cargo clippy -- -D warnings
```

อันนี้ดีมากสำหรับ production project

เช่น CI:

```powershell
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

---

# 11. Dependency Management

นี่เป็นส่วนที่คุณน่าจะใช้เยอะ เพราะก่อนหน้านี้คุณใช้ Conan กับ C++

ดู dependency:

```powershell
cargo tree
```

ตัวอย่าง:

```text
myapp
├── opencv
│   ├── ...
│   └── ...
├── tokio
└── serde
```

ดู dependency แบบกลับด้าน:

```powershell
cargo tree -i serde
```

ดู features:

```powershell
cargo tree -e features
```

Cargo ระบุไว้โดยตรงว่า `cargo tree -e features` ใช้ดูว่า feature ไหนถูกเปิดโดย package ไหน และ `-i` ใช้ดู dependency แบบ inverse ได้ ([Rust Documentation][3])

---

# 12. Update Dependency

```powershell
cargo update
```

ดูว่า package ไหน update:

```powershell
cargo update
```

Update package เดียว:

```powershell
cargo update -p serde
```

---

# 13. Download Dependency

```powershell
cargo fetch
```

มีประโยชน์มากเวลาเตรียม build environment หรือ CI

เช่น:

```powershell
cargo fetch
cargo build --offline
```

---

# 14. Offline Mode

ถ้า dependency ถูก download แล้ว:

```powershell
cargo build --offline
```

ถ้าต้องการบังคับทั้ง lock และ offline:

```powershell
cargo build --frozen
```

`--frozen` เทียบเท่ากับ:

```text
--locked + --offline
```

ตาม Cargo documentation ([Rust Documentation][2])

---

# 15. `Cargo.lock`

สำหรับ application:

```text
Cargo.toml
Cargo.lock
```

สำคัญมาก

ดู dependency ที่ถูก lock:

```powershell
cargo tree
```

ตรวจว่า lock file ไม่เปลี่ยน:

```powershell
cargo build --locked
```

ถ้า dependency resolution ทำให้ `Cargo.lock` ต้องเปลี่ยน command จะ fail

เหมาะกับ CI/CD

```powershell
cargo build --release --locked
```

---

# 16. เพิ่ม Dependency

Cargo รุ่นใหม่รองรับ:

```powershell
cargo add serde
```

เช่น:

```powershell
cargo add tokio
```

พร้อม feature:

```powershell
cargo add tokio --features full
```

กำหนด version:

```powershell
cargo add serde@1
```

ลบ:

```powershell
cargo remove serde
```

อันนี้เทียบได้ประมาณกับ package management ใน ecosystem อื่น ๆ

---

# 17. Features

Feature เป็นเรื่องสำคัญมากใน Rust

เปิด feature:

```powershell
cargo build --features foo
```

หลาย feature:

```powershell
cargo build --features "foo bar"
```

เปิดทั้งหมด:

```powershell
cargo build --all-features
```

ปิด default:

```powershell
cargo build --no-default-features
```

Feature ของ workspace package:

```powershell
cargo build -p mypackage --features mypackage/feature1
```

Cargo รองรับ `--features`, `--all-features`, และ `--no-default-features` โดยตรง ([Rust Documentation][3])

---

# 18. Workspace

ถ้าคุณทำระบบใหญ่ ผมแนะนำให้รู้ Workspace ตั้งแต่แรก

ตัวอย่าง:

```text
my-system/
├── Cargo.toml
├── core/
│   └── Cargo.toml
├── image/
│   └── Cargo.toml
├── inference/
│   └── Cargo.toml
└── server/
    └── Cargo.toml
```

Build ทั้ง workspace:

```powershell
cargo build --workspace
```

Test:

```powershell
cargo test --workspace
```

Build package เดียว:

```powershell
cargo build -p inference
```

Test package:

```powershell
cargo test -p inference
```

Cargo ใช้ `-p/--package` และ `--workspace` สำหรับเลือก package ใน workspace ([Rust Documentation][4])

---

# 19. Documentation

สร้าง documentation:

```powershell
cargo doc
```

และเปิด:

```powershell
cargo doc --open
```

รวม dependencies:

```powershell
cargo doc --no-deps
```

Release:

```powershell
cargo doc --release
```

Cargo สามารถ generate documentation สำหรับ target architecture ที่ระบุด้วย `--target` ได้ด้วย ([Rust Documentation][5])

---

# 20. Package

ตรวจสอบ package:

```powershell
cargo package
```

ดูว่า package มีอะไรบ้าง:

```powershell
cargo package --list
```

ตรวจ package:

```powershell
cargo package --allow-dirty
```

ไฟล์ที่ได้:

```text
target/package/
```

---

# 21. Publish

ถ้าเป็น crate ที่ต้องการปล่อยไป crates.io:

```powershell
cargo publish
```

ตรวจสอบก่อน:

```powershell
cargo publish --dry-run
```

นี่เป็น command ที่ควรใช้ระวัง เพราะ `publish` เป็นการเผยแพร่ package จริง

---

# 22. Install CLI Application

ถ้า crate เป็น command-line application:

```powershell
cargo install ripgrep
```

ตัวอย่าง:

```powershell
cargo install cargo-watch
```

Update:

```powershell
cargo install ripgrep --force
```

ติดตั้งจาก local project:

```powershell
cargo install --path .
```

---

# 23. `cargo search`

ค้นหา crate:

```powershell
cargo search opencv
```

เช่น:

```powershell
cargo search tokio
cargo search serde
cargo search kafka
```

---

# 24. `cargo info`

ดูข้อมูล crate:

```powershell
cargo info tokio
```

เหมาะสำหรับดู:

```text
version
features
dependencies
license
repository
```

---

# 25. Dependency Tree ขั้นสูง

ดู tree:

```powershell
cargo tree
```

ดู duplicates:

```powershell
cargo tree -d
```

ดู dependencies:

```powershell
cargo tree --depth 1
```

ดู feature:

```powershell
cargo tree -e features
```

inverse dependency:

```powershell
cargo tree -i openssl
```

เช่นคุณสงสัยว่า:

```text
ทำไม openssl ถึงถูกติดตั้ง?
```

ใช้:

```powershell
cargo tree -i openssl
```

อันนี้มีประโยชน์มากเวลาเจอ dependency conflict

---

# 26. `cargo metadata`

อันนี้สำหรับ tooling / automation

```powershell
cargo metadata
```

output เป็น JSON

นิยมใช้ใน:

```text
IDE
Build system
CI/CD
Scripts
Dependency analysis
Code generation
```

ตัวอย่าง:

```powershell
cargo metadata --format-version 1
```

ถ้าคุณจะเขียน tooling ด้วย Rust เอง command นี้มีประโยชน์มาก

---

# 27. `cargo locate-project`

หา `Cargo.toml`

```powershell
cargo locate-project
```

เช่น:

```text
C:\Users\papop\projects\rust-standard\Cargo.toml
```

มีประโยชน์กับ script/tooling

---

# 28. `cargo version`

คุณใช้อยู่แล้ว:

```powershell
cargo version
```

แบบ verbose:

```powershell
cargo version --verbose
```

จะเห็นข้อมูลเพิ่มเติมเกี่ยวกับ Cargo build

---

# 29. `cargo help`

ดู command ทั้งหมด:

```powershell
cargo help
```

ดู help ของ command:

```powershell
cargo help build
```

หรือ:

```powershell
cargo build --help
```

Cargo รองรับ `cargo help <command>` และ `--help` โดยตรง ([Rust Documentation][6])

ถ้าจะดู command ที่เครื่องคุณรองรับจริง ๆ ผมแนะนำ:

```powershell
cargo --list
```

เพราะมันสะท้อน Cargo version ที่คุณติดตั้งอยู่

---

# 30. Build Timing

อันนี้น่าสนใจมากสำหรับคุณ

```powershell
cargo build --timings
```

Cargo จะสร้างรายงาน:

```text
target/cargo-timings/
```

ใช้ดูว่า dependency ตัวไหน compile นาน

เช่น:

```text
opencv
tokio
serde
bindgen
...
```

ตัวนี้มีประโยชน์มากเวลาคุณเจอปัญหา:

```text
cargo build ทำไมช้า?
```

Cargo ระบุว่า `--timings` จะสร้าง `cargo-timing.html` เพื่อดูเวลาการ compile และ concurrency ของ build ([Rust Documentation][2])

---

# 31. Verbose

```powershell
cargo build -v
```

หรือ:

```powershell
cargo build --verbose
```

ละเอียดมากขึ้น:

```powershell
cargo build -vv
```

`-vv` มีประโยชน์มากเวลา debug:

```text
build.rs
linker
native library
C/C++
FFI
OpenCV
CUDA
ONNX Runtime
```

---

# 32. Quiet

```powershell
cargo build -q
```

หรือ:

```powershell
cargo build --quiet
```

เหมาะกับ script/CI

---

# 33. Target Architecture

ดู target ที่ Rust รองรับ:

```powershell
rustc --print target-list
```

Build target:

```powershell
cargo build --target x86_64-pc-windows-msvc
```

เช่น Linux:

```powershell
cargo build --target x86_64-unknown-linux-gnu
```

หรือ ARM:

```powershell
cargo build --target aarch64-unknown-linux-gnu
```

อันนี้สำคัญถ้าคุณจะทำ:

```text
Windows
Linux
ARM
Docker
Embedded
Cross compilation
```

---

# 34. `cargo rustc`

ตัวนี้คือการเข้าถึง `rustc` ผ่าน Cargo:

```powershell
cargo rustc
```

ส่ง flags ให้ rustc:

```powershell
cargo rustc -- -C target-cpu=native
```

เช่น performance tuning:

```powershell
cargo rustc --release -- -C target-cpu=native
```

แต่โดยทั่วไปถ้าจะ configure production build ควรใช้ Cargo profiles/configuration มากกว่ายัด flags ทุกครั้ง เพราะ Cargo เองจัดการ compiler flags จาก profiles ได้เหมาะสมกว่า ([Rust Documentation][7])

---

# 35. `cargo fix`

ให้ Cargo/Rust compiler ช่วยแก้บาง warning:

```powershell
cargo fix
```

ตัวอย่าง:

```powershell
cargo fix --allow-dirty
```

หรือ:

```powershell
cargo fix --allow-staged
```

Cargo documentation ระบุว่า `cargo fix` ใช้ compiler suggestions เพื่อแก้ source code บางประเภทโดยอัตโนมัติ ([Rust Documentation][8])

---

# 36. Alias

คุณสามารถสร้าง command alias ใน:

```text
.cargo/config.toml
```

ตัวอย่าง:

```toml
[alias]
b = "build"
c = "check"
t = "test"
r = "run"
rr = "run --release"
```

แล้วใช้:

```powershell
cargo b
cargo c
cargo t
cargo r
cargo rr
```

Cargo รองรับ `[alias]` ใน configuration โดยตรง ([Rust Documentation][7])

---

# 37. Configuration

ไฟล์:

```text
.cargo/config.toml
```

สามารถตั้ง:

```toml
[build]
jobs = 8
```

กำหนด target:

```toml
[build]
target = "x86_64-pc-windows-msvc"
```

กำหนด target directory:

```toml
[build]
target-dir = "build"
```

กำหนด rustflags:

```toml
[build]
rustflags = [
    "-C",
    "target-cpu=native"
]
```

Cargo configuration รองรับ build jobs, rustc, target, target-dir, rustflags, incremental compilation และอื่น ๆ ([Rust Documentation][7])

---

# 38. Toolchain

Cargo ทำงานร่วมกับ `rustup`

ตัวอย่าง:

```powershell
cargo +stable build
```

Nightly:

```powershell
cargo +nightly build
```

ระบุ version:

```powershell
cargo +1.97.1 build
```

ดู Rust:

```powershell
rustc --version
```

ดู rustup:

```powershell
rustup --version
```

ดู toolchain:

```powershell
rustup show
```

Cargo รองรับ syntax `+toolchain` เมื่อ Cargo ถูกติดตั้งผ่าน rustup ([Rust Documentation][2])

---

# 39. Command ที่ผมจัดว่า "Advanced"

ยังมีอีกหลายตัวที่ไม่ได้ใช้ทุกวัน:

```text
cargo metadata
cargo locate-project
cargo package
cargo publish
cargo install
cargo search
cargo info
cargo tree
cargo vendor
cargo fetch
cargo update
cargo fix
cargo rustc
cargo doc
cargo bench
```

รวมถึงบาง command ที่เป็น nightly/unstable เช่น:

```powershell
cargo -Z help
```

แต่ไม่ควรใช้ `-Z` ใน production โดยไม่เข้าใจว่ามันเป็น unstable feature เพราะ Cargo ระบุชัดว่า `-Z` เป็น nightly-only ([Rust Documentation][9])

---

# 40. Command ที่คุณควรจำจริง ๆ

ถ้าผมจัดจาก "ต้องรู้" → "ขั้นสูง" สำหรับคุณ ผมจะจัดแบบนี้

ระดับ 1 — ใช้ทุกวัน:

```powershell
cargo new
cargo init
cargo check
cargo build
cargo run
cargo test
cargo fmt
cargo clippy
cargo clean
```

ระดับ 2 — Project จริง:

```powershell
cargo add
cargo remove
cargo update
cargo tree
cargo fetch
cargo doc
cargo bench
cargo build --release
cargo test --workspace
cargo build --workspace
```

ระดับ 3 — Performance / Production:

```powershell
cargo build --release
cargo build --timings
cargo tree -e features
cargo tree -d
cargo tree -i <package>
cargo build --locked
cargo build --offline
cargo build --frozen
cargo build --target <target>
cargo rustc
```

ระดับ 4 — Library / Distribution:

```powershell
cargo package
cargo publish
cargo install
cargo search
cargo info
cargo metadata
```

ระดับ 5 — Tooling:

```powershell
cargo locate-project
cargo metadata
cargo -vv
cargo fix
cargo +nightly ...
cargo -Z help
```

---

## และถ้าคุณทำ Rust + C++/OpenCV/YOLO

สำหรับแนวงานของคุณ ผมจะจำชุดนี้เป็นพิเศษ:

```powershell
cargo check
cargo build
cargo build --release
cargo run --release
cargo test
cargo bench
cargo fmt
cargo clippy
cargo tree
cargo tree -i <dependency>
cargo tree -e features
cargo build --timings
cargo build -vv
cargo build --locked
cargo build --offline
cargo clean
```

โดยเฉพาะเวลาคุณใช้ crate ที่มี native dependency เช่น OpenCV, ONNX Runtime, CUDA, FFmpeg หรือ C/C++ FFI ให้จำ:

```powershell
cargo build -vv
```

เพราะมันช่วยดูรายละเอียดของ build script และ native linking ได้ละเอียดมาก

และถ้า compile ช้า:

```powershell
cargo build --timings
```

ถ้าสงสัยว่า dependency ตัวไหนลาก library เข้ามา:

```powershell
cargo tree -i <library>
```

ถ้าสงสัยว่า feature อะไรถูกเปิด:

```powershell
cargo tree -e features
```

สามตัวนี้มีประโยชน์มากสำหรับ project Rust ขนาดใหญ่

แหล่งอ้างอิงหลักคือ Cargo Book ของ Rust ซึ่งเป็น documentation ทางการ และรายการ command/รายละเอียดด้าน dependency, features, workspace, configuration และ profiles มีการอัปเดตตาม Cargo รุ่นปัจจุบัน ([Rust Documentation][1])

[1]: https://doc.rust-lang.org/cargo/reference/?utm_source=chatgpt.com "Cargo Reference - The Cargo Book"
[2]: https://doc.rust-lang.org/cargo/commands/cargo-run.html?utm_source=chatgpt.com "cargo run - The Cargo Book"
[3]: https://doc.rust-lang.org/stable/cargo/reference/features.html?utm_source=chatgpt.com "Features - The Cargo Book"
[4]: https://doc.rust-lang.org/cargo/reference/workspaces.html?utm_source=chatgpt.com "Workspaces - The Cargo Book"
[5]: https://doc.rust-lang.org/cargo/commands/cargo-doc.html?utm_source=chatgpt.com "cargo doc - The Cargo Book"
[6]: https://doc.rust-lang.org/cargo/commands/cargo-help.html?utm_source=chatgpt.com "cargo help - The Cargo Book"
[7]: https://doc.rust-lang.org/cargo/reference/config.html?highlight=configu&utm_source=chatgpt.com "Configuration - The Cargo Book"
[8]: https://doc.rust-lang.org/cargo/commands/cargo-fix.html?utm_source=chatgpt.com "cargo fix - The Cargo Book"
[9]: https://doc.rust-lang.org/cargo/reference/unstable.html?utm_source=chatgpt.com "Unstable Features - The Cargo Book"
