# 📄 PDFtoText

A simple Rust tool to extract text from PDF files and output it to stdout.

## 🚀 Getting Started

### Prerequisites

Make sure you have [Rust](https://www.rust-lang.org/) installed on your system. You can install Rust using [rustup](https://rustup.rs/).

### 📦 Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/yourusername/pdftotext.git
cd pdftotext
```

### ⚙️ Build

Build the project using Cargo:

```bash
cargo build --release
```

### ▶️ Usage

Run the program from the command line, specifying the path to the PDF file as an argument:

```bash
./target/release/pdftotext path/to/your/file.pdf
```

### 🧪 Example

```bash
./target/release/pdftotext tests/docs/simple.pdf
```

## 📚 Dependencies

This project uses the following crate:

- [`pdf-extract`](https://crates.io/crates/pdf-extract) - A crate for extracting text from PDF files.

Add it to your `Cargo.toml`:

```toml
[dependencies]
pdf-extract = "0.7.7"
```

## 🙏 Acknowledgments

- Inspired by various open-source projects and the Rust community.

---

Happy coding! 😊