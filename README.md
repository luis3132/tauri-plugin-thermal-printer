# Tauri Plugin thermal-printer

This plugin provides thermal printer functionality for Tauri applications, allowing you to print documents, test printers, and list available printers.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✅        |
| macOS    | ✅        |
| Windows  | ✅        |
| Android  | ?        |
| iOS      | ❌        |

## Table of Contents

- [How it Works](#how-it-works)
- [Installation](#installation)
  - [Rust](#rust)
  - [Bun / NPM / PNPM](#bun--npm--pnpm)
  - [lib.rs](#librs)
  - [Permission](#permission)
- [Functions](#functions)
  - [List Printers](#list-printers)
  - [Test Printer](#test-printer)
  - [Print Document](#print-document)
  - [Paper Size Helpers (TypeScript)](#paper-size-helpers-typescript)
  - [Error Handling](#error-handling)
- [Section Types](#section-types)
  - [Title](#title)
  - [Subtitle](#subtitle)
  - [Text](#text)
  - [Feed](#feed)
  - [Cut](#cut)
  - [Beep](#beep)
  - [Drawer](#drawer)
  - [Qr](#qr)
  - [Barcode](#barcode)
  - [Table](#table)
  - [DataMatrix](#datamatrix)
  - [Pdf417](#pdf417)
  - [Image](#Image)
  - [Logo](#logo)
  - [Line](#line)
  - [GlobalStyles](#globalstyles)
- [TypeScript Constants & Helpers](#typescript-constants--helpers)
  - [CodePage](#codepage)
  - [Style constants](#style-constants)
  - [Section builder helpers](#section-builder-helpers)
  - [Helper example (all builders)](#helper-example-all-builders)
- [Examples](#examples)

## How it Works

This plugin acts as a **translator** between a user-friendly JavaScript/TypeScript API and the low-level ESC/POS binary commands that thermal printers understand.

### Architecture

```
Frontend (JavaScript/TypeScript)
    ↓ (IPC Commands)
Tauri Core (Rust)  ←— ESC/POS generation (shared across all platforms)
    ↓ (Platform-specific implementations)
    ├── Desktop: Operating System (Linux/macOS/Windows)
    │       ↓ (Raw binary data)
    │   Thermal Printer (ESC/POS protocol)
    │
    └── Android: Kotlin Plugin
            ↓ (Bluetooth SPP / RFCOMM)
        Thermal Printer (ESC/POS protocol)
```

### Core Components

#### 1. **Data Models** (`src/models/`)
- **`PrintJobRequest`**: Main structure defining a print job
- **`PrintSections`**: Enum with all printable content types (Title, Text, Table, QR, etc.)
- **`GlobalStyles`**: Formatting styles (bold, alignment, size, etc.)

#### 2. **Tauri Commands** (`src/commands.rs`)
Three main functions exposed to the frontend:
- `list_thermal_printers()`: Lists available printers
- `print_thermal_printer()`: Prints a document
- `test_thermal_printer()`: Runs functionality tests

#### 3. **Print Processing** (`src/process/process_print.rs`)
Converts data structures into ESC/POS binary commands:
```rust
pub fn generate_document(&mut self, print_job: &PrintJobRequest) -> Result<Vec<u8>, String>
```

#### 4. **OS Integration** (`src/desktop_printers/` and `android/`)
- **Linux/macOS**: Uses CUPS system (`lpstat`, `lp` commands)
- **Windows**: Uses WinAPI (Windows API) to directly access system printers via functions such as EnumPrintersW for listing printers, OpenPrinterW for opening printer handles, and WritePrinter for sending raw data
- **Android**: Kotlin plugin with Bluetooth SPP and USB printer discovery and printing

### Workflow

#### Printing a Document (Desktop):

1. **Frontend** sends `PrintJobRequest` with sections and configuration
2. **Tauri** receives the command and processes it in Rust
3. **`ProcessPrint`** converts each section into ESC/POS commands
4. **Operating System** sends binary data to the printer
5. **Thermal Printer** interprets ESC/POS commands and prints

#### Printing a Document (Android):

1. **Frontend** sends `PrintJobRequest` with sections and configuration
2. **Rust** generates ESC/POS binary data using the same `ProcessPrint` pipeline
3. **Kotlin plugin** receives the binary data and the printer MAC address
4. **Bluetooth SPP** connection is established to the printer
5. **Thermal Printer** interprets ESC/POS commands and prints

#### Print Structure Example:
```json
{
  "printer": "TM-T20II",
  "paper_size": "Mm80",
  "options": {
    "cut_paper": true,
    "beep": false
  },
  "sections": [
    {"Title": {"text": "My Title"}},
    {"Text": {"text": "Normal content"}},
    {"Table": {"columns": 3, "body": [["A", "B", "C"]]}}
  ]
}
```

### ESC/POS Protocol

The plugin translates all sections into **ESC/POS** (Escape Sequence for Point of Sale) commands, the de facto standard for thermal printers:

- `\x1B\x40` - Initialize printer
- `\x1B\x61\x01` - Center text
- `\x1B\x45\x01` - Enable bold
- etc.

### Supported Content Types

- **Text**: Title, Subtitle, Text with optional styles
- **Codes**: QR, Barcode, DataMatrix, PDF417
- **Media**: Images, Logos
- **Control**: Feed, Cut, Beep, Cash Drawer
- **Tables**: Configurable columns
- **Lines**: Horizontal separators

### Platform Status

- ✅ **Linux**: Fully functional (CUPS)
- ✅ **macOS**: Fully functional (CUPS)
- ✅ **Windows**: Fully functional (WinAPI)
- ✅ **Android**: Bluetooth and USB printer discovery and printing
- ❌ **iOS**: Not implemented

### Supported Connections

| Connection | Linux | macOS | Windows | Android |
| ---------- | ----- | ----- | ------- | ------- |
| USB        | ✅    | ✅    | ✅      | ✅ (discovery only) |
| Network    | ✅    | ✅    | ✅      | ❌      |
| Bluetooth  | ❌    | ❌    | ❌      | ✅      |

> **Android note**: The `printer` field in `PrintJobRequest` must be the Bluetooth MAC address of the printer (e.g. `"AA:BB:CC:DD:EE:FF"`). The printer must be previously paired in the Android Bluetooth settings. Bluetooth permissions are requested automatically at runtime.

## Installation

### Rust

```bash
cargo add tauri-plugin-thermal-printer
```

### Bun / NPM / PNPM

```bash
bun add tauri-plugin-thermal-printer
```

This library not only contains the connector to the backend. Also adds the types for the print structure...

### lib.rs

Don't forget to add this line

```rust
.plugin(tauri_plugin_thermal_printer::init())
```

### Permission

Modify the file in /file/to/project/capabilities/default.json, and add:

```json
{
  "permissions": [
    "core:default",
    "thermal-printer:allow-list-thermal-printers",
    "thermal-printer:allow-print-thermal-printer",
    "thermal-printer:allow-test-thermal-printer"
  ]
}
```

## Alternative Installation

```bash
git clone https://github.com/luis3132/tauri-plugin-thermal-printer
cd tauri-plugin-thermal-printer
cargo build --release && bun i && bun run build
```

on src-tauri project file

```toml
[dependencies]
tauri-plugin-thermal-printer = { path = "../../tauri-plugin-thermal-printer" }
```

on package.json

```json
"dependencies": {
  "tauri-plugin-thermal-printer": "file:../tauri-plugin-thermal-printer"
}
```

## Functions

### List Printers

Get all printers available in the system. It just lists the configured printers...

#### Request:
```typescript
import { list_thermal_printers } from "tauri-plugin-thermal-printer";

try {
  const response = await list_thermal_printers();
} catch (error) {
  console.log("List printers failed: " + error)
}
```

#### Response
```json
[
  {
    "name": "TM-T20II",
    "interface_type": "USB",
    "identifier": "usb://EPSON/TM-T20II",
    "status": "IDLE"
  },
  {
    "name": "Star TSP143III",
    "interface_type": "NETWORK",
    "identifier": "192.168.1.100:9100",
    "status": "IDLE"
  }
]
```

#### Response fields (array of PrinterInfo):
- `name` (string): Name of the printer
- `interface_type` (string): Interface type (e.g., "USB", "NETWORK")
- `identifier` (string): Unique identifier (e.g., USB path or IP:PORT)
- `status` (string): Current status (e.g., "IDLE", "BUSY")

---

### Test Printer

Send a print test to a specific printer to verify functionality.

#### Request:
```typescript
import { ENCODE, test_thermal_printer, type TestPrintRequest } from "tauri-plugin-thermal-printer";

try { await test_thermal_printer({
  "printer_info": {
    "printer": "TM-T20II",
    "paper_size": "Mm80",
    "options": {
      "cut_paper": true,
      "beep": true,
      "open_cash_drawer": false,
      "code_page": {
        "code_page": 6,
        "encode": ENCODE.WINDOWS_1252,
        "use_gbk": false
      }
    },
    "sections": [] // it's not going to print anything
  },
  "include_text": true,
  "include_text_styles": true,
  "include_alignment": true,
  "include_columns": true,
  "include_separators": true,
  "include_barcode": true,
  "include_barcode_types": false,
  "include_qr": true,
  "include_image": false,
  "image_base64": null,
  "include_beep": true,
  "test_cash_drawer": false,
  "cut_paper": true,
  "test_feed": true,
  "test_all_fonts": false,
  "test_invert": false,
  "test_rotate": false
} as TestPrintRequest) } catch (error) { console.error("Test print failed:", error); }
```

#### Request parameters (TestPrintRequest):

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `printer_info` | PrintJobRequest | ✅ Yes | Printer configuration (see Print Document) |
| `include_text` | boolean | ❌ No | Basic text test (default: `true`) |
| `include_text_styles` | boolean | ❌ No | Text styles test (bold, underline, inverted) (default: `true`) |
| `include_alignment` | boolean | ❌ No | Alignment test (left, center, right) (default: `true`) |
| `include_columns` | boolean | ❌ No | Column tables test (default: `true`) |
| `include_separators` | boolean | ❌ No | Separator lines test (default: `true`) |
| `include_barcode` | boolean | ❌ No | Barcode test (default: `true`) |
| `include_barcode_types` | boolean | ❌ No | Multiple barcode types test (default: `false`) |
| `include_qr` | boolean | ❌ No | QR code test (default: `true`) |
| `include_image` | boolean | ❌ No | Image printing test (default: `false`) |
| `image_base64` | string | ❌ No | Base64 image for testing (only if `include_image` is `true`) |
| `include_beep` | boolean | ❌ No | Acoustic signal test (default: `true`) |
| `test_cash_drawer` | boolean | ❌ No | Cash drawer opening test (default: `false`) |
| `cut_paper` | boolean | ❌ No | Cut paper at the end (default: `true`) |
| `test_feed` | boolean | ❌ No | Paper feed test (default: `true`) |
| `test_all_fonts` | boolean | ❌ No | Test all available fonts (default: `false`) |
| `test_invert` | boolean | ❌ No | Inverted text test (default: `false`) |
| `test_rotate` | boolean | ❌ No | Text rotation test (default: `false`) |

#### Response:
Returns `Promise<void>`. Resolves when the test print completes successfully. **Throws a `string`** with the error message if it fails. Use `try/catch` to handle errors — see [Error Handling](#error-handling).

---

### Print Document

Print a personalized document with the specified sections.

#### Request:
```typescript
import { ENCODE, print_thermal_printer, type PrintJobRequest } from "tauri-plugin-thermal-printer";

try { await print_thermal_printer({
  "printer": "TM-T20II",
  "paper_size": "Mm80",
  "options": {
    "cut_paper": true,
    "beep": false,
    "open_cash_drawer": false,
    "code_page": {
      "code_page": 6,
      "encode": ENCODE.WINDOWS_1252,
      "use_gbk": false
    }
  },
  "sections": [
    {"Title": {"text": "My Business"}},
    {"Subtitle": {"text": "Date: 01/01/2000"}},
    {"Text": {"text": "Normal text", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Feed": {"feed_type": "lines", "value": 3}},
    {"Cut": {"mode": "full", "feed": 3}},
    {"Beep": {"times": 1, "duration": 100}},
    {"Drawer": {"pin": 2, "pulse_time": 100}},
    {"Qr": {"data": "https://example.com", "size": 5, "error_correction": "M", "model": 2}},
    {"Barcode": {"data": "123456789012", "barcode_type": "CODE128", "width": 2, "height": 100, "text_position": "below"}},
    {"Table": {"columns": 3, "column_widths": [16, 16, 16], "header": [{"text": "Col1"}, {"text": "Col2"}, {"text": "Col3"}], "body": [[{"text": "Data1"}, {"text": "Data2"}, {"text": "Data3"}]], "truncate": false}},
    {"DataMatrix": {"data": "DataMatrix data", "size": 5}},
    {"Pdf417": {"data": "PDF417 data", "columns": 2, "rows": 5, "width": 3, "height": 5, "error_correction": 2}},
    {"Image": {"data": "{base64 image data}", "max_width": 384, "align": "center", "dithering": true, "size": "normal"}},
    {"Logo": {"key_code": 1, "mode": "normal"}},
    {"Line": {"character": "="}}
  ]
} as PrintJobRequest) } catch (error) { console.error("Print failed:", error); }
```

#### Response:
Returns `Promise<void>`. Resolves when printing completes successfully. **Throws a `string`** with the error message if the job fails (e.g., printer not found, invalid barcode data, QR data too long). Use `try/catch` to handle errors — see [Error Handling](#error-handling).

#### Main parameters (PrintJobRequest):

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `printer` | string | ✅ Yes | Printer name |
| `paper_size` | PaperSize | ❌ No | Paper size (default: `"Mm80"`) — see [Paper Sizes](#paper-sizes) |
| `options` | PrinterOptions | ❌ No | Configuration options |
| `options.cut_paper` | boolean | ❌ No | Append a tail `Cut` section equivalent to `{ "Cut": { "mode": "partial", "feed": 0 } }` (default: `true`) |
| `options.beep` | boolean | ❌ No | Append a tail `Beep` section equivalent to `{ "Beep": { "times": 1, "duration": 3 } }` (default: `false`) |
| `options.open_cash_drawer` | boolean | ❌ No | Append a tail `Drawer` section equivalent to `{ "Drawer": { "pin": 2, "pulse_time": 100 } }` (default: `false`) |
| `options.code_page` | CodePage | ✅ Yes | Required ESC/POS page selection plus host-side encoding strategy — see [CodePage](#codepage) |
| `sections` | array | ✅ Yes | Array of sections to print (see [Section Types](#section-types)) |

`options.cut_paper`, `options.beep`, and `options.open_cash_drawer` are tail-action shorthands. They append sections at the end of the document in this fixed order: `Beep` → `Cut` → `Drawer`. They do **not** disable or replace manually declared sections, so if you specify both, both actions are emitted.

#### Paper Sizes

| Value | Paper width | Chars/line | Typical use |
|-------|-------------|------------|-------------|
| `"Mm40"` | 40mm | 21 | Handheld ticket printers |
| `"Mm44"` | 44mm | 24 | Compact POS |
| `"Mm58"` | 58mm | 32 | Small format (most common portable) |
| `"Mm72"` | 72mm | 42 | Mid-range printers |
| `"Mm80"` | 80mm | 48 | Standard large format (default) |
| `"Mm104"` | 104mm | 62 | Wide format |

#### Paper Size Helpers (TypeScript)

The TypeScript package exports constants and helper functions with the same values used by the Rust backend.

```typescript
import {
  PAPER_SIZE_CHARS_PER_LINE,
  PAPER_SIZE_PIXELS_WIDTH,
  DEFAULT_PAPER_SIZE,
  getPaperSizeCharsPerLine,
  getPaperSizePixelsWidth,
  type PaperSize,
} from "tauri-plugin-thermal-printer";

const size: PaperSize = "Mm58";

console.log(DEFAULT_PAPER_SIZE); // "Mm80"
console.log(PAPER_SIZE_CHARS_PER_LINE[size]); // 32
console.log(PAPER_SIZE_PIXELS_WIDTH[size]); // 384

// Equivalent helper functions:
console.log(getPaperSizeCharsPerLine(size)); // 32
console.log(getPaperSizePixelsWidth(size)); // 384
```

Values per paper size:

| PaperSize | Chars/line | Pixels width |
|-----------|------------|--------------|
| `"Mm40"` | 21 | 256 |
| `"Mm44"` | 24 | 288 |
| `"Mm58"` | 32 | 384 |
| `"Mm72"` | 42 | 512 |
| `"Mm80"` | 48 | 576 |
| `"Mm104"` | 62 | 752 |

---

### Error Handling

`print_thermal_printer` and `test_thermal_printer` now return `Promise<void>` and **throw** a descriptive `string` when something fails. Always wrap calls in `try/catch`:

```typescript
import { print_thermal_printer, type PrintJobRequest } from "tauri-plugin-thermal-printer";

try {
  await print_thermal_printer(job);
} catch (error) {
  // `error` is a string describing what went wrong, e.g.:
  // "Printer not specified"
  // "Barcode data cannot be empty"
  // "Barcode type 'EAN13' only accepts numeric digits"
  // "QR data length 5000 exceeds maximum 4296 for error correction level 'M'"
  // "Table row 2 has 2 cells but 3 columns declared"
  // "column_widths sum (45) must equal paper chars_per_line (48)"
  // "Image data cannot be empty"
  console.error("Print failed:", error);
}
```

`list_thermal_printers` also throws on failure (e.g., CUPS not available):

```typescript
try {
  const printers = await list_thermal_printers();
} catch (error) {
  console.error("Could not list printers:", error);
}
```

---

#### Section Types

Sections are defined as objects in the `sections` array. Each section is an enum variant with its data. Below are all supported section types:

##### Title
Prints a title with forced double size and center alignment.

```json
{
  "Title": {
    "text": "My Title",
    "styles": {
      "bold": false,
      "underline": false,
      "align": "center",
      "italic": false,
      "invert": false,
      "font": "A",
      "rotate": false,
      "upside_down": false,
      "size": "Double"
    }
  }
}
```

Or simply:

```json
{
  "Title": {
    "text": "My Title"
  }
}
```

- `text` (string, required): Title text
- `styles` (GlobalStyles, optional): Applied styles

##### Subtitle
Prints a subtitle with forced bold and increased height.

```json
{
  "Subtitle": {
    "text": "My Subtitle",
    "styles": {
      "bold": true,
      "underline": false,
      "align": "left",
      "italic": false,
      "invert": false,
      "font": "A",
      "rotate": false,
      "upside_down": false,
      "size": "height"
    }
  }
}
```

Or simply:

```json
{
  "Subtitle": {
    "text": "My Subtitle"
  }
}
```

- `text` (string, required): Subtitle text
- `styles` (GlobalStyles, optional): Applied styles

##### Text
Prints simple text with optional styles.

```json
{
  "Text": {
    "text": "Normal text",
    "styles": {
      "bold": false,
      "underline": false,
      "align": "left",
      "italic": false,
      "invert": false,
      "font": "A",
      "rotate": false,
      "upside_down": false,
      "size": "normal"
    }
  }
}
```

Or 

```json
{
  "Text": {
    "text": "My Subtitle",
    "styles": {
      "bold": true,
      "underline": true
    }
  }
}
```

Or simple version

```json
{
  "Text": {
    "text": "My Subtitle"
  }
}
```

You can pick the style that you need, it's not necessary to declared all of them.

- `text` (string, required): Text to print
- `styles` (GlobalStyles, optional): Applied styles (defaults to current global styles)

##### Feed
Advances the paper by a specific number of lines.

```json
{
  "Feed": {
    "feed_type": "lines",
    "value": 3
  }
}
```

- `feed_type` (string, required): Feed type:
  - `"lines"` — advance N lines (`ESC d n`)
  - `"dots"` — advance N dot rows (`ESC J n`)
  - `"line_feed"` — send N raw LF characters
- `value` (number, required): Amount to advance

##### Cut
Cuts the paper.

```json
{
  "Cut": {
    "mode": "full",
    "feed": 3
  }
}
```

- `mode` (string, required): Cut mode:
  - `"full"` — full cut
  - `"partial"` — partial cut (default fallback)
  - `"partial_alt"` — partial cut (alternate)
  - `"partial_alt2"` — full cut (alternate)
- `feed` (number, required): Lines to advance before cutting

##### Beep
Emits a beep.

```json
{
  "Beep": {
    "times": 1,
    "duration": 100
  }
}
```

- `times` (number, required): Number of beeps
- `duration` (number, required): Duration in milliseconds

##### Drawer
Opens the cash drawer.

```json
{
  "Drawer": {
    "pin": 2,
    "pulse_time": 100
  }
}
```

- `pin` (number, required): Drawer pin (2 or 5)
- `pulse_time` (number, required): Pulse time in milliseconds

##### Qr
Prints a QR code.

```json
{
  "Qr": {
    "data": "https://example.com",
    "size": 5,
    "error_correction": "M",
    "model": 2,
    "align": "center"
  }
}
```

- `data` (string, required): QR data. **Must not be empty.** Maximum length depends on error correction level — the backend will throw if exceeded.
- `size` (number, required): Module size (1–16)
- `error_correction` (string, required): `"L"` (7089 chars max) | `"M"` (4296, default) | `"Q"` (2953) | `"H"` (1817)
- `model` (number, required): QR model (`1` or `2`)
- `align` (string, optional): `"left"` | `"center"` | `"right"`

##### Barcode
Prints a barcode.

```json
{
  "Barcode": {
    "data": "123456789",
    "barcode_type": "CODE128",
    "width": 2,
    "height": 100,
    "text_position": "below",
    "align": "center"
  }
}
```

- `data` (string, required): Barcode data. **Must not be empty.** Numeric-only types (`UPC-A`, `UPC-E`, `EAN13`, `EAN8`, `ITF`) only accept digit characters — the backend will throw an error otherwise.
- `barcode_type` (string, required): `"UPC-A"` | `"UPC-E"` | `"EAN13"` | `"EAN8"` | `"CODE39"` | `"ITF"` | `"CODABAR"` | `"CODE93"` | `"CODE128"`
- `width` (number, required): Module width (1–6)
- `height` (number, required): Height in dots (must be > 0)
- `text_position` (string, required): `"none"` | `"above"` | `"below"` | `"both"`
- `align` (string, optional): `"left"` | `"center"` | `"right"` (default: current global alignment)

##### Table
Prints a table.

```json
{
  "Table": {
    "columns": 3,
    "column_widths": [10, 15, 10],
    "header": [
      {"text": "Col1"},
      {"text": "Col2"},
      {"text": "Col3"}
    ],
    "body": [
      [
        {"text": "Data1"},
        {"text": "Data2"},
        {"text": "Data3"}
      ]
    ],
    "truncate": false
  }
}
```

Or simply:

```json
{
  "Table": {
    "columns": 3,
    "body": [
      [
        {"text": "Data1"},
        {"text": "Data2"},
        {"text": "Data3"}
      ]
    ]
  }
}
```

- `columns` (number, required): Number of columns
- `column_widths` (array, optional): Widths of each column in characters. **When provided: length must equal `columns` and the sum must equal the paper's chars/line** (e.g., 48 for Mm80). If omitted, columns are distributed evenly.
- `header` (array, optional): Column headers — must have exactly `columns` elements if provided
- `body` (array, required): Data rows — each row must have exactly `columns` cells
- `truncate` (boolean, optional): Truncate long text instead of wrapping (default: `false`)

##### DataMatrix
Prints a DataMatrix code.

```json
{
  "DataMatrix": {
    "data": "DataMatrix data",
    "size": 5
  }
}
```

- `data` (string, required): DataMatrix data
- `size` (number, required): Module size (1-16)

##### Pdf417
Prints a PDF417 code.

```json
{
  "Pdf417": {
    "data": "PDF417 data",
    "columns": 2,
    "rows": 5,
    "width": 3,
    "height": 5,
    "error_correction": 2
  }
}
```

- `data` (string, required): PDF417 data
- `columns` (number, required): Number of data columns
- `rows` (number, required): Number of data rows
- `width` (number, required): Module width
- `height` (number, required): Module height
- `error_correction` (number, required): Error correction level (0-8)

##### Image
Prints an image.

```json
{
  "Image": {
    "data": "base64_encoded_image",
    "max_width": 384,
    "align": "center",
    "dithering": true,
    "size": "normal"
  }
}
```

- `data` (string, required): Base64 encoded image. **Must not be empty.**
- `max_width` (number, required): Maximum width in pixels (0 or values larger than the paper width are clamped to the paper width automatically)
- `align` (string, required): `"left"` | `"center"` | `"right"`
- `dithering` (boolean, required): Apply Floyd-Steinberg dithering for better quality on monochrome printers
- `size` (string, required): `"normal"` | `"double_width"` | `"double_height"` | `"quadruple"`

##### Logo
Prints a logo stored in the printer.

```json
{
  "Logo": {
    "key_code": 1,
    "mode": "normal"
  }
}
```

- `key_code` (number, required): Logo key code (1-255)
- `mode` (string, required): Print mode ("normal", "double_width", "double_height", "quadruple")

##### Line
Prints a separator line.

```json
{
  "Line": {
    "character": "="
  }
}
```

- `character` (string, required): Character for the line (e.g., "=", "-", "_")


##### GlobalStyles
Changes the current global styles that will be applied to subsequent text sections. This allows you to set default styles without specifying them for each text element.

```json
{
  "GlobalStyles": {
    "bold": false,
    "underline": false,
    "align": "left",
    "italic": false,
    "invert": false,
    "font": "A",
    "rotate": false,
    "upside_down": false,
    "size": "normal"
  }
}
```

- `bold` (boolean, optional): Bold text (default: `false`)
- `underline` (boolean, optional): Underlined text (default: `false`)
- `align` (string, optional): Alignment ("left", "center", "right") (default: `"left"`)
- `italic` (boolean, optional): Italic text (default: `false`)
- `invert` (boolean, optional): Inverted text (black background) (default: `false`)
- `font` (string, optional): Font ("A", "B", "C") (default: `"A"`)
- `rotate` (boolean, optional): Text rotated 90 degrees (default: `false`)
- `upside_down` (boolean, optional): Upside down text (default: `false`)
- `size` (string, optional): Size ("normal", "height", "width", "double") (default: `"normal"`)

---

## TypeScript Constants & Helpers

The plugin exports typed constants and builder functions so you never have to type raw strings.

### CodePage

Set the character encoding once in `PrinterOptions.code_page` and all text sections (`Title`, `Subtitle`, `Text`, `Table`) will use it automatically.

Each printer model assigns its own `ESC t n` values, so `CodePage.code_page` accepts the raw page number directly. `CodePage.encode` controls the host-side encoding used before bytes are sent to the printer. `CodePage.use_gbk` explicitly controls whether characters that the selected `encode` cannot represent should be retried with GBK before falling back to the original UTF-8 bytes.

```typescript
import { ENCODE, type CodePage } from "tauri-plugin-thermal-printer";

const options = {
  cut_paper: true,
  beep: false,
  open_cash_drawer: false,
  code_page: {
    code_page: 6,
    encode: ENCODE.WINDOWS_1252,
    use_gbk: false,
  }, // sends ESC t 6
};
```

`CodePage` fields:

| Field | Required | Description |
|---|---|---|
| `code_page` | ✅ Yes | Raw `ESC t n` value sent to the printer. |
| `encode` | ❌ No | Host-side encoding strategy. Defaults to `ENCODE.ACCENT_REMOVER`. |
| `use_gbk` | ❌ No | Retries GBK for characters that `encode` cannot represent before falling back to the original UTF-8 bytes. Defaults to `false`. |

`ENCODE.ACCENT_REMOVER`:

- Transliterates accented characters to ASCII before any optional GBK retry
  and final UTF-8 passthrough.
- Useful when the printer does not have a reliable legacy code page for your
  text.
- Examples: `á -> a`, `ß -> ss`, `€ -> EUR`.

All other `ENCODE.*` values come directly from
[`encoding_rs` statics](https://docs.rs/encoding_rs/latest/encoding_rs/#statics).
Use them with the same uppercase names exposed by the package.

Examples:

- `ENCODE.WINDOWS_1252` for Western European text
- `ENCODE.GBK` for GBK output
- `ENCODE.SHIFT_JIS` for Shift JIS output

> **Note**: `options.code_page` is required. If the selected `encode` cannot represent a character, the plugin retries GBK only when `use_gbk` is `true`; otherwise it silently emits the original UTF-8 bytes for that character.

---

### Style constants

Instead of typing raw strings you can import typed constant objects:

```typescript
import {
  TEXT_ALIGN,
  TEXT_SIZE,
  TEXT_FONT,
  BARCODE_TYPE,
  BARCODE_TEXT_POSITION,
  QR_ERROR_CORRECTION,
  IMAGE_MODE,
  CUT_MODE,
} from "tauri-plugin-thermal-printer";

// Examples:
const styles = {
  align: TEXT_ALIGN.CENTER,   // "center"
  size: TEXT_SIZE.DOUBLE,     // "double"
  font: TEXT_FONT.B,          // "B"
  bold: true,
};

const barcode = {
  barcode_type: BARCODE_TYPE.EAN13,             // "EAN13"
  text_position: BARCODE_TEXT_POSITION.BELOW,   // "below"
};

const qr = {
  error_correction: QR_ERROR_CORRECTION.M,      // "M"
};
```

| Export | Values |
|---|---|
| `TEXT_ALIGN` | `LEFT` `CENTER` `RIGHT` |
| `TEXT_SIZE` | `NORMAL` `HEIGHT` `WIDTH` `DOUBLE` |
| `TEXT_FONT` | `A` `B` `C` |
| `BARCODE_TYPE` | `UPC_A` `UPC_E` `EAN13` `EAN8` `CODE39` `ITF` `CODABAR` `CODE93` `CODE128` |
| `BARCODE_TEXT_POSITION` | `NONE` `ABOVE` `BELOW` `BOTH` |
| `QR_ERROR_CORRECTION` | `L` `M` `Q` `H` |
| `IMAGE_MODE` | `NORMAL` `DOUBLE_WIDTH` `DOUBLE_HEIGHT` `QUADRUPLE` |
| `CUT_MODE` | `FULL` `PARTIAL` |

---

### Section builder helpers

Short helper functions to build section types without enum wrapper boilerplate:

```typescript
import {
  title, subtitle, text, line, feed, cut, globalStyles,
  beep, drawer, table, qr, barcode, dataMatrix, pdf417, image, logo,
  TEXT_ALIGN, TEXT_SIZE, BARCODE_TYPE, QR_ERROR_CORRECTION,
} from "tauri-plugin-thermal-printer";

const sections = [
  title("My Business"),
  subtitle("Receipt #001"),
  text("Thank you for your purchase!", { align: TEXT_ALIGN.CENTER }),
  line("="),
  qr("https://example.com/order/123", {
    size: 6,
    error_correction: QR_ERROR_CORRECTION.M,
  }),
  barcode("123456789012", BARCODE_TYPE.EAN13),
  beep(),
  text("Total: $50.00", { bold: true, size: TEXT_SIZE.DOUBLE }),
  line("-"),
  feed(3),
  cut(),
];
```

| Helper | Description |
|---|---|
| `title(text, styles?)` | Creates a `{ Title: ... }` section |
| `subtitle(text, styles?)` | Creates a `{ Subtitle: ... }` section |
| `text(text, styles?)` | Creates a `{ Text: ... }` section |
| `line(character?)` | Creates a `{ Line: ... }` section (default `"-"`) |
| `feed(value, type?)` | Creates a `{ Feed: ... }` section (default `"lines"`) |
| `cut(mode?, feedLines?)` | Creates a `{ Cut: ... }` section (default `"partial"`, 4 lines) |
| `globalStyles(styles)` | Creates a `{ GlobalStyles: ... }` section |
| `beep(times?, duration?)` | Creates a `{ Beep: ... }` section (default `1`, `3`) |
| `drawer(pin?, pulse_time?)` | Creates a `{ Drawer: ... }` section (default `2`, `120`) |
| `table(columns, body, options?)` | Creates a `{ Table: ... }` section (`truncate` default `true`) |
| `qr(data, options?)` | Creates a `{ Qr: ... }` section (`size=6`, `error_correction="M"`, `model=2`) |
| `barcode(data, barcode_type?, options?)` | Creates a `{ Barcode: ... }` section (`CODE128`, `width=3`, `height=80`, `text_position="below"`) |
| `dataMatrix(data, size?)` | Creates a `{ DataMatrix: ... }` section (default `size=6`) |
| `pdf417(data, options?)` | Creates a `{ Pdf417: ... }` section (`columns=0`, `rows=0`, `width=2`, `height=3`, `error_correction=2`) |
| `image(data, options?)` | Creates a `{ Image: ... }` section (`max_width=0`, `align="center"`, `dithering=true`, `size="normal"`) |
| `logo(key_code, mode?)` | Creates a `{ Logo: ... }` section (default `mode="normal"`) |

### Helper example (all builders)

```typescript
import {
  print_thermal_printer,
  type PrintJobRequest,
  title,
  subtitle,
  text,
  line,
  feed,
  cut,
  globalStyles,
  beep,
  drawer,
  table,
  qr,
  barcode,
  dataMatrix,
  pdf417,
  image,
  logo,
  ENCODE,
  TEXT_ALIGN,
  TEXT_SIZE,
  BARCODE_TYPE,
  BARCODE_TEXT_POSITION,
  QR_ERROR_CORRECTION,
  IMAGE_MODE,
} from "tauri-plugin-thermal-printer";

const job: PrintJobRequest = {
  printer: "TM-T20II",
  paper_size: "Mm80",
  options: {
    cut_paper: true,
    beep: false,
    open_cash_drawer: false,
    code_page: {
      code_page: 6,
      encode: ENCODE.WINDOWS_1252,
      use_gbk: false,
    },
  },
  sections: [
    globalStyles({ align: TEXT_ALIGN.LEFT }),
    title("DEMO STORE"),
    subtitle("Receipt #A-1001"),
    text("Date: 2026-03-30 14:22"),
    line("="),
    table(
      3,
      [
        [text("1"), text("Americano"), text("$2.50", { align: TEXT_ALIGN.RIGHT })],
        [text("2"), text("Croissant"), text("$7.00", { align: TEXT_ALIGN.RIGHT })],
      ],
      {
        column_widths: [6, 28, 14],
        header: [
          text("QTY", { bold: true }),
          text("ITEM", { bold: true }),
          text("TOTAL", { bold: true, align: TEXT_ALIGN.RIGHT }),
        ],
        truncate: true,
      },
    ),
    line("-"),
    text("Grand total: $9.50", { bold: true, size: TEXT_SIZE.DOUBLE, align: TEXT_ALIGN.RIGHT }),
    qr("https://example.com/r/A-1001", {
      size: 6,
      error_correction: QR_ERROR_CORRECTION.M,
      model: 2,
      align: TEXT_ALIGN.CENTER,
    }),
    barcode("123456789012", BARCODE_TYPE.EAN13, {
      width: 3,
      height: 70,
      text_position: BARCODE_TEXT_POSITION.BELOW,
      align: TEXT_ALIGN.CENTER,
    }),
    dataMatrix("A-1001", 6),
    pdf417("A-1001|TOTAL=9.50|PAID", {
      columns: 0,
      rows: 0,
      width: 2,
      height: 3,
      error_correction: 2,
    }),
    image("<BASE64_IMAGE>", {
      max_width: 0,
      align: TEXT_ALIGN.CENTER,
      dithering: true,
      size: IMAGE_MODE.NORMAL,
    }),
    logo(1, IMAGE_MODE.NORMAL),
    drawer(2, 120),
    beep(1, 3),
    feed(3),
    cut(),
  ],
};

await print_thermal_printer(job);
```

---

## Examples

This section contains practical examples for different use cases. Each example demonstrates how to structure print jobs for various business scenarios.

> **NOTE:** With default options, a tail `Cut` section is appended automatically as `{ "Cut": { "mode": "partial", "feed": 0 } }`. Add your own `Cut` section when you need an extra cut or different parameters.

### 🛒 Long Receipt (Supermarket - 80mm)

```typescript
import { print_thermal_printer, type PrintJobRequest } from "tauri-plugin-thermal-printer";

const receipt: PrintJobRequest = {
  "printer": "TM-T20II",
  "paper_size": "Mm80",
  "options": {
    "cut_paper": true,
    "beep": false,
    "open_cash_drawer": false
  },
  "sections": [
    {"Title": {"text": "SUPERMERCADO LA ECONOMÍA", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "Double"}}},
    {"Text": {"text": "Sucursal Centro", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Av. Juárez #1234, Col. Centro", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Tel: (555) 123-4567", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "RFC: SUPE850101ABC", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "TICKET DE COMPRA", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Fecha: 14/10/2025 15:45:30", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Ticket: #0012345", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Cajero: María González", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Caja: 03", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Table": {
      "columns": 4,
      "column_widths": [5, 20, 11, 12],
      "header": [
        {"text": "CANT", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
        {"text": "DESCRIPCIÓN", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
        {"text": "P.U.", "styles": {"bold": true, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
        {"text": "TOTAL", "styles": {"bold": true, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
      ],
      "body": [
        [
          {"text": "2", "styles": null},
          {"text": "Leche Lala 1L", "styles": null},
          {"text": "$22.50", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
          {"text": "$45.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ],
        [
          {"text": "1", "styles": null},
          {"text": "Pan Bimbo Blanco", "styles": null},
          {"text": "$38.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
          {"text": "$38.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ],
        [
          {"text": "3", "styles": null},
          {"text": "Coca Cola 600ml", "styles": null},
          {"text": "$16.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
          {"text": "$48.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ],
        [
          {"text": "1", "styles": null},
          {"text": "Cereal Zucaritas", "styles": null},
          {"text": "$75.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
          {"text": "$75.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ],
        [
          {"text": "1", "styles": null},
          {"text": "Azúcar 1kg", "styles": null},
          {"text": "$25.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
          {"text": "$25.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ]
      ],
      "truncate": false
    }},
    {"Line": {"character": "="}},
    {"Table": {
      "columns": 2,
      "column_widths": [32, 16],
      "header": [],
      "body": [
        [
          {"text": "SUBTOTAL:", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
          {"text": "$1,280.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ],
        [
          {"text": "IVA (16%):", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
          {"text": "$204.80", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ]
      ],
      "truncate": false
    }},
    {"Line": {"character": "="}},
    {"Text": {"text": "TOTAL: $1,484.80", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "Forma de Pago: EFECTIVO", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Table": {
      "columns": 2,
      "column_widths": [32, 16],
      "header": [],
      "body": [
        [
          {"text": "Pago con:", "styles": null},
          {"text": "$1,500.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ],
        [
          {"text": "Cambio:", "styles": null},
          {"text": "$15.20", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ]
      ],
      "truncate": false
    }},
    {"Line": {"character": "-"}},
    {"Text": {"text": "Artículos: 25", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Ahorro total: $85.50", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "¡GRACIAS POR SU COMPRA!", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Vuelva pronto", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "www.supereconomia.com", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Qr": {"data": "https://supereconomia.com/ticket/0012345", "size": 5, "error_correction": "M", "model": 2}},
    {"Barcode": {"data": "0012345", "barcode_type": "CODE128", "width": 2, "height": 50, "text_position": "below"}},
    {"Feed": {"feed_type": "lines", "value": 3}}
  ]
};

await print_thermal_printer(receipt);
```

---

### 🍕 Restaurant Ticket (80mm)

```typescript
const restaurantTicket: PrintJobRequest = {
  "printer": "TM-T20II",
  "paper_size": "Mm80",
  "options": {
    "cut_paper": true,
    "beep": false,
    "open_cash_drawer": false
  },
  "sections": [
    {"Title": {"text": "RESTAURANTE EL BUEN SABOR", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "Double"}}},
    {"Text": {"text": "Comida Mexicana", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Tel: (555) 987-6543", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "ORDEN #145", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Mesa: 12", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Mesero: Carlos Ruiz", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Fecha: 14/10/2025 14:30", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Table": {
      "columns": 3,
      "column_widths": [5, 28, 15],
      "header": [
        {"text": "CANT", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
        {"text": "PLATILLO", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}},
        {"text": "PRECIO", "styles": {"bold": true, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
      ],
      "body": [
        [
          {"text": "2", "styles": null},
          {"text": "Tacos al Pastor", "styles": null},
          {"text": "$45.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ],
        [
          {"text": "1", "styles": null},
          {"text": "Enchiladas Verdes", "styles": null},
          {"text": "$85.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ],
        [
          {"text": "1", "styles": null},
          {"text": "Pozole Grande", "styles": null},
          {"text": "$95.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ],
        [
          {"text": "3", "styles": null},
          {"text": "Refresco 600ml", "styles": null},
          {"text": "$36.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ],
        [
          {"text": "1", "styles": null},
          {"text": "Agua de Horchata", "styles": null},
          {"text": "$25.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ]
      ],
      "truncate": false
    }},
    {"Line": {"character": "="}},
    {"Table": {
      "columns": 2,
      "column_widths": [32, 16],
      "header": [],
      "body": [
        [
          {"text": "SUBTOTAL:", "styles": null},
          {"text": "$286.00", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ],
        [
          {"text": "Propina Sugerida (10%):", "styles": null},
          {"text": "$28.60", "styles": {"bold": false, "underline": false, "align": "right", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}
        ]
      ],
      "truncate": false
    }},
    {"Line": {"character": "="}},
    {"Text": {"text": "TOTAL: $314.60", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "¡Gracias por su visita!", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Esperamos verlo pronto", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Feed": {"feed_type": "lines", "value": 3}}
  ]
};
```

---

### 👨‍🍳 Kitchen Order (80mm)

```typescript
const kitchenOrder: PrintJobRequest = {
  "printer": "TM-T20II",
  "paper_size": "Mm80",
  "options": {
    "cut_paper": true,
    "beep": true,
    "open_cash_drawer": false
  },
  "sections": [
    {"Title": {"text": "*** COMANDA COCINA ***", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "Double"}}},
    {"Text": {"text": "Orden: #145", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Mesa: 12", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Hora: 14:30", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "2x TACOS AL PASTOR", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "   - Sin cebolla", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "   - Extra cilantro", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "1x ENCHILADAS VERDES", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "   - Término medio", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "1x POZOLE GRANDE", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "   - Extra rábanos", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "   - Sin orégano", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "Mesero: Carlos", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Notas: Cliente regular", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Feed": {"feed_type": "lines", "value": 3}},
    {"Beep": {"times": 2, "duration": 100}}
  ]
};
```

---

### 🏷️ Product Label (58mm)

```typescript
const productLabel: PrintJobRequest = {
  "printer": "TM-T20II",
  "paper_size": "Mm58",
  "options": {
    "cut_paper": true,
    "beep": false,
    "open_cash_drawer": false
  },
  "sections": [
    {"Title": {"text": "PRODUCTO", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "Double"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "Nombre: Laptop HP", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Modelo: 15-dy2021la", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "UPC: 7501234567890", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "PRECIO: $12,999.00", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Barcode": {"data": "7501234567890", "barcode_type": "EAN13", "width": 2, "height": 50, "text_position": "below"}},
    {"Feed": {"feed_type": "lines", "value": 2}}
  ]
};
```

---

### 🎟️ Service Turn Ticket (58mm)

```typescript
const turnTicket: PrintJobRequest = {
  "printer": "TM-T20II",
  "paper_size": "Mm58",
  "options": {
    "cut_paper": true,
    "beep": true,
    "open_cash_drawer": false
  },
  "sections": [
    {"Title": {"text": "TICKET DE TURNO", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "Double"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "A-123", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "Double"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "Servicio: Cajas", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Fecha: 14/10/2025", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Hora: 15:45", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "En espera: 8 turnos", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Tiempo aprox: 20 min", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Qr": {"data": "A-123", "size": 4, "error_correction": "L", "model": 2}},
    {"Text": {"text": "Escanea para consultar", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Feed": {"feed_type": "lines", "value": 2}},
    {"Beep": {"times": 1, "duration": 100}}
  ]
};
```

---

### 🚗 Parking Ticket (80mm)

```typescript
const parkingTicket: PrintJobRequest = {
  "printer": "TM-T20II",
  "paper_size": "Mm80",
  "options": {
    "cut_paper": true,
    "beep": false,
    "open_cash_drawer": false
  },
  "sections": [
    {"Title": {"text": "ESTACIONAMIENTO", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "Double"}}},
    {"Subtitle": {"text": "PLAZA COMERCIAL", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "Ticket: E-5678", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Entrada: 14/10/2025 10:15", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Caseta: A-01", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "Vehículo: ABC-1234", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Nivel: 2 - Zona B", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "TARIFAS:", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Primera hora: $20.00", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Hora adicional: $15.00", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Máximo 24hrs: $180.00", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "CONSERVE SU TICKET", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Para salida y pago", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Barcode": {"data": "E5678", "barcode_type": "CODE128", "width": 2, "height": 60, "text_position": "below"}},
    {"Feed": {"feed_type": "lines", "value": 3}}
  ]
};
```

---

### 🎫 Event Ticket (80mm)

```typescript
const eventTicket: PrintJobRequest = {
  "printer": "TM-T20II",
  "paper_size": "Mm80",
  "options": {
    "cut_paper": true,
    "beep": false,
    "open_cash_drawer": false
  },
  "sections": [
    {"Title": {"text": "CONCIERTO 2025", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "Double"}}},
    {"Subtitle": {"text": "BANDA ROCK NACIONAL", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "Boleto: #A-1234567", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Fecha: 25/10/2025", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Hora: 20:00 hrs", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Lugar: Auditorio Nacional", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Table": {
      "columns": 2,
      "column_widths": [24, 24],
      "header": [],
      "body": [
        [
          {"text": "Zona:", "styles": null},
          {"text": "Preferente A", "styles": null}
        ],
        [
          {"text": "Fila:", "styles": null},
          {"text": "12", "styles": null}
        ],
        [
          {"text": "Asiento:", "styles": null},
          {"text": "45", "styles": null}
        ]
      ],
      "truncate": false
    }},
    {"Line": {"character": "="}},
    {"Text": {"text": "PRECIO: $850.00", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "Titular: Juan Pérez", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "ID: 1234567890", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "IMPORTANTE:", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "- Presentar identificación", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "- Llegar 30 min antes", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "- No se permiten reembolsos", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Qr": {"data": "TICKET-A1234567-CONCIERTO2025", "size": 6, "error_correction": "H", "model": 2}},
    {"Barcode": {"data": "A1234567", "barcode_type": "CODE128", "width": 2, "height": 60, "text_position": "below"}},
    {"Feed": {"feed_type": "lines", "value": 3}}
  ]
};
```

---

### 💳 Payment Receipt (80mm)

```typescript
const paymentReceipt: PrintJobRequest = {
  "printer": "TM-T20II",
  "paper_size": "Mm80",
  "options": {
    "cut_paper": true,
    "beep": false,
    "open_cash_drawer": false
  },
  "sections": [
    {"Title": {"text": "COMPROBANTE DE PAGO", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "Double"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "Banco Nacional", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Sucursal Centro", "styles": {"bold": false, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Operación: 987654321", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Fecha: 14/10/2025 16:23:45", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "TRANSFERENCIA ELECTRÓNICA", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "De:", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "  Cuenta: ****5678", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "  Nombre: Juan Pérez", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "Para:", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "  Cuenta: ****9012", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "  Nombre: María López", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "MONTO: $5,000.00 MXN", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "Concepto:", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Pago de renta mensual", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "-"}},
    {"Text": {"text": "Comisión: $0.00", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "IVA: $0.00", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "TOTAL: $5,000.00", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Line": {"character": "="}},
    {"Text": {"text": "Estado: EXITOSA", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Referencia: 123456789012345", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Text": {"text": "Folio fiscal: ABCD-1234-EFGH", "styles": {"bold": false, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
    {"Feed": {"feed_type": "lines", "value": 3}}
  ]
};
```



---

### 📋 Summary

#### Tables
- **Columns**: Supports 2, 3, or 4 columns
- **Column widths** are optional - distributed automatically if not specified
- **Important**: Sum of widths should not exceed 48 characters (80mm) or 32 characters (58mm)

#### QR Codes
- **Size**: 1-10 (recommended: 5-6)
- **Error correction**: `"L"` (7%), `"M"` (15%), `"Q"` (25%), `"H"` (30%)
- **Model**: 1 or 2 (recommended: 2)

#### Barcodes
- **Types**: UPC-A, UPC-E, EAN13, EAN8, CODE39, ITF, CODABAR, CODE93, CODE128
- **Height**: 30-100 dots (recommended: 50-60)
- **Text position**: `"not_printed"`, `"above"`, `"below"`, `"both"`

#### Styles
- Bold, underline, and invert styles are automatically reset after each section
- No need to manually reset styles

#### Paper Cutting
- **Automatic**: `options.cut_paper = true` appends a tail `Cut` section with `mode: "partial"` and `feed: 0`
- **Manual**: Add a `Cut` section anywhere in `sections` to issue explicit cut commands
- **Combined**: Automatic and manual cuts are both emitted when both are present
