# PDS-ApiProgramming
# PDS - API Programming: Rust & Concurrent Programming

This repository contains my academic work for the **Programmazione di Sistema (System Programming)** course, focusing on **Rust programming language** and **concurrent programming** concepts. The materials include lecture examples, lab exercises, and exam preparations.

## 📚 Course Overview

This course covers advanced programming concepts in Rust, with a strong emphasis on:
- **Memory Safety** without garbage collection
- **Ownership, Borrowing, and Lifetimes**
- **Concurrent and Parallel Programming**
- **Error Handling and Type System**
- **Systems Programming Patterns**

---

## 📁 Repository Structure

### 🔤 Language Fundamentals

| Directory | Description |
|-----------|-------------|
| `Il_linguaggio/` | Core Rust language concepts and syntax |
| `Lifetime_Funzioni/` | Lifetime annotations and function signatures |
| `Tratti_generics/` | Traits, generics, and polymorphism |
| `Gestione_Errori/` | Error handling patterns (`Result`, `Option`, `?` operator) |
| `Collezioni/` | Collections: `Vec`, `HashMap`, `HashSet`, etc. |
| `Iteratori/` | Iterator patterns and adapters |
| `Input_Output_files/` | File I/O operations and stream handling |

### 🔄 Concurrency & Parallelism

| Directory | Description |
|-----------|-------------|
| `Concorrenza/` | Concurrency examples including threads, `Arc`, `Mutex`, `RwLock`, channels (`mpsc`), `Condvar`, and synchronization primitives |

### 📝 Lab Exercises (Esercitazioni)

| Directory | Topics |
|-----------|--------|
| `Esercitazione1/` | Naval battle game, clock implementation, slug formatting, preparatory exercises |
| `Esercitazione2/` | Circular buffer, complex numbers, slug processing |
| `Esercitazione3/` | Advanced Rust exercises |
| `Esercitazione4/` | Multi-part exercises on various topics |
| `Esercitazione5/` | Iterator and concurrency exercises |

### 📖 Course Materials (Rebaudengo)

Organized by units following the course curriculum:

| Unit | Topic |
|------|-------|
| `U1_ Introduzione al Linguaggio` | Introduction to Rust, ownership, borrowing |
| `U2_ Tipi composti, Polimorfismo, Tempi di vita e Chiusure` | Compound types, polymorphism, lifetimes, closures |
| `U3_ Error Handling` | Error handling techniques |
| `U4_ Iterators, Collections e I/O` | Iterators, collections, and I/O |
| `U5_ Smart Pointers` | `Box`, `Rc`, `RefCell`, and other smart pointers |
| `U6_ Moduli e Test` | Modules organization and testing |
| `U7_ Concorrenza` | Concurrency with threads and synchronization |
| `U8_ Processi` | Process management |

### 📋 Exams

| Directory | Description |
|-----------|-------------|
| `Esami/EserciziEsame/` | Practice exercises for exam preparation |
| `Esami/Esame25-06-2024/` | Exam materials from June 25, 2024 |

### 📂 Additional Materials

| Directory | Description |
|-----------|-------------|
| `2022/` | Previous year materials including polymorphism, traits, iterators, threads, and more |
| `rust1/` | Additional Rust examples and exercises |

---

## 🦀 Key Rust Concepts Covered

### Memory Management
- **Ownership**: Each value has a single owner
- **Borrowing**: References without taking ownership (`&T`, `&mut T`)
- **Lifetimes**: Compiler-enforced reference validity

### Concurrency Primitives
- **Threads**: `std::thread`, `spawn`, `join`, scoped threads
- **Shared State**: `Arc<Mutex<T>>`, `Arc<RwLock<T>>`
- **Message Passing**: Channels (`mpsc::channel`, `mpsc::sync_channel`)
- **Synchronization**: `Mutex`, `RwLock`, `Condvar`, `Barrier`
- **Atomic Types**: Lock-free concurrent programming

### Type System
- **Generics**: Type parameters and bounds
- **Traits**: Behavior definition and implementation
- **Associated Types**: Types associated with traits
- **Trait Objects**: Dynamic dispatch with `dyn Trait`

### Error Handling
- **Result<T, E>**: Recoverable errors
- **Option<T>**: Nullable values
- **Propagation**: `?` operator for error propagation
- **Custom Errors**: Implementing `Error` trait

---

## 🛠️ Building and Running

Each project directory contains a `Cargo.toml` file. To build and run:

```bash
# Navigate to a project directory
cd Concorrenza

# Build the project
cargo build

# Run the project
cargo run

# Run tests (if available)
cargo test
```

---

## 📖 Learning Path Recommendation

1. **Start with fundamentals**: `Il_linguaggio/`, `Lifetime_Funzioni/`
2. **Master the type system**: `Tratti_generics/`, `Collezioni/`
3. **Handle errors properly**: `Gestione_Errori/`
4. **Work with data streams**: `Iteratori/`, `Input_Output_files/`
5. **Dive into concurrency**: `Concorrenza/`, `Rebaudengo/U7_ Concorrenza/`
6. **Practice with exercises**: `Esercitazione1/` through `Esercitazione5/`
7. **Prepare for exams**: `Esami/`

---

## 📄 License

This repository contains academic work and study materials. Please use responsibly and in accordance with academic integrity policies.

---

## 👤 Author

Academic work completed as part of the System Programming (Programmazione di Sistema) course focusing on Rust and Concurrent Programming