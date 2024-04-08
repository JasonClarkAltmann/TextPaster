# TextPaster

TextPaster is a simple Rust application that provides a user interface for entering text and then "pasting" that text elsewhere.

## Functionality

The application provides a text field for users to enter text. After entering the text, users can click the "Paste" button to "paste" the entered text elsewhere. The application uses the `Enigo` library to simulate keyboard inputs and paste the text.

The application waits three seconds before it "pastes" the text after the user clicks the "Paste" button. This gives the user time to set the focus to the window or field where the text should be pasted.

## Usage

To use the application, simply run the program and enter the text you want to "paste". Then click the "Paste" button and set the focus to the window or field where the text should be pasted within three seconds.

## Installation

To install the application, clone the repository and build the program with `cargo build`. You will need Rust and Cargo on your system to build the program.

```bash
git clone https://github.com/JasonClarkAltmann/text-paster.git
cd text-paster
cargo build --release
```

The built program will then be in the `target/release` directory.
