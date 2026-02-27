# Tauri Plugin thermal-printer

This plugin provides thermal printer functionality for Tauri applications, allowing you to print documents, test printers, and list available printers.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✅        |
| macOS    | ✅        |
| Windows  | ✅        |
| Android  | ❌        |
| iOS      | ❌        |

For mobile applications, this plugin is currently working on this...

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
- [Examples](#examples)

## How it Works

This plugin acts as a **translator** between a user-friendly JavaScript/TypeScript API and the low-level ESC/POS binary commands that thermal printers understand.

### Architecture

```
Frontend (JavaScript/TypeScript) 
    ↓ (IPC Commands)
Tauri Core (Rust)
    ↓ (Platform-specific implementations)
Operating System (Linux/macOS/Windows)
    ↓ (Raw binary data)
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

#### 4. **OS Integration** (`src/desktop_printers/`)
- **Linux/macOS**: Uses CUPS system (`lpstat`, `lp` commands)
- **Windows**: Uses WinAPI (Windows API) to directly access system printers via functions such as EnumPrintersW for listing printers, OpenPrinterW for opening printer handles, and WritePrinter for sending raw data
- **Android**: Basic structure present, not yet implemented

### Workflow

#### Printing a Document:

1. **Frontend** sends `PrintJobRequest` with sections and configuration
2. **Tauri** receives the command and processes it in Rust
3. **`ProcessPrint`** converts each section into ESC/POS commands
4. **Operating System** sends binary data to the printer
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
- ❌ **Android**: Basic structure present, not implemented
- ❌ **iOS**: Not implemented

### Supported Connections

- **USB**: Direct USB port connection
- **Network**: TCP/IP (port 9100 typical)
- **Serial**: RS-232 (less common)
- **Bluetooth**: For Android (when implemented)

## Installation

### Rust

```toml
[dependencies]
tauri-plugin-thermal-printer = {} # it's not published yet
```

### Bun / NPM / PNPM

```bash
# it's not published yet
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
  "tauri-plugin-thermal-printer-api": "file:../tauri-plugin-thermal-printer"
}
```

## Functions

### List Printers

Get all printers available in the system. It just lists the configured printers...

#### Request:
```typescript
import { list_thermal_printers } from "tauri-plugin-thermal-printer-api";

const response = await list_thermal_printers();
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
import { test_thermal_printer, type TestPrintRequest } from "tauri-plugin-thermal-printer-api";

const response = await test_thermal_printer({
  "printer_info": {
    "printer": "TM-T20II",
    "paper_size": "Mm80",
    "options": {
      "cut_paper": true,
      "beep": true,
      "open_cash_drawer": false
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
} as TestPrintRequest)
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
Returns `boolean`:
- `true`: Test completed successfully
- `false`: Test failed

---

### Print Document

Print a personalized document with the specified sections.

#### Request:
```typescript
import { print_thermal_printer, type PrintJobRequest } from "tauri-plugin-thermal-printer-api";

const response = await print_thermal_printer({
  "printer": "TM-T20II",
  "paper_size": "Mm80",
  "options": {
    "cut_paper": true,
    "beep": false,
    "open_cash_drawer": false
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
    {"Barcode": {"data": "123456789", "barcode_type": "CODE128", "width": 2, "height": 100, "text_position": "below"}},
    {"Table": {"columns": 3, "column_widths": [10, 15, 10], "header": [{"text": "Col1"}, {"text": "Col2"}, {"text": "Col3"}], "body": [[{"text": "Data1"}, {"text": "Data2"}, {"text": "Data3"}]], "truncate": false}},
    {"DataMatrix": {"data": "DataMatrix data", "size": 5}},
    {"Pdf417": {"data": "PDF417 data", "columns": 2, "rows": 5, "width": 3, "height": 5, "error_correction": 2}},
    {"Image": {"data": "{please introduce a base64 data image}", "max_width": 384, "align": "center", "dithering": true, "size": "normal"}},
    {"Logo": {"key_code": 1, "mode": "normal"}},
    {"Line": {"character": "="}}
  ]
} as PrintJobRequest)
```

#### Response:
Returns `boolean`:
- `true`: Print completed successfully
- `false`: Print failed

#### Main parameters (PrintJobRequest):

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `printer` | string | ✅ Yes | Printer name |
| `paper_size` | string | ❌ No | Paper size: `"Mm58"` or `"Mm80"` (default: `"Mm80"`) |
| `options` | PrinterOptions | ❌ No | Configuration options |
| `options.cut_paper` | boolean | ❌ No | Cut paper after printing (default: `true`) |
| `options.beep` | boolean | ❌ No | Beep after printing (default: `false`) |
| `options.open_cash_drawer` | boolean | ❌ No | Open cash drawer after printing (default: `false`) |
| `sections` | array | ✅ Yes | Array of sections to print (see [Section Types](#section-types)) |

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

- `feed_type` (string, required): Feed type ("lines")
- `value` (number, required): Number of lines to advance

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

- `mode` (string, required): Cut mode ("full", "partial")
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

- `data` (string, required): QR data
- `size` (number, required): Module size (1-16)
- `error_correction` (string, required): Error correction level ("L", "M", "Q", "H")
- `model` (number, required): QR model (1 or 2)
- `align` (string, optional): Alignment ("left", "center", "right")

##### Barcode
Prints a barcode.

```json
{
  "Barcode": {
    "data": "123456789",
    "barcode_type": "CODE128",
    "width": 2,
    "height": 100,
    "text_position": "below"
  }
}
```

- `data` (string, required): Barcode data
- `barcode_type` (string, required): Type ("UPC-A", "UPC-E", "EAN13", "EAN8", "CODE39", "ITF", "CODABAR", "CODE93", "CODE128")
- `width` (number, required): Module width
- `height` (number, required): Height in dots
- `text_position` (string, required): Text position ("not_printed", "above", "below", "both")

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
- `column_widths` (array, optional): Widths of each column. If not provided, columns will be evenly distributed across the paper width
- `header` (array, optional): Column headers (array of Text objects)
- `body` (array, required): Data rows (array of arrays of Text objects)
- `truncate` (boolean, optional): Truncate long text (default: false)

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

- `data` (string, required): Base64 encoded image
- `max_width` (number, required): Maximum width in pixels
- `align` (string, required): Alignment ("left", "center", "right")
- `dithering` (boolean, required): Apply dithering
- `size` (string, required): Size ("normal", "double_width", "double_height", "quadruple")

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

## Examples

This section contains practical examples for different use cases. Each example demonstrates how to structure print jobs for various business scenarios.

> **NOTE:** Paper is automatically cut at the end of printing with a full cut. You don't need to add a `Cut` section manually unless you want a specific partial cut.

### 🛒 Long Receipt (Supermarket - 80mm)

```typescript
import { print_thermal_printer, type PrintJobRequest } from "tauri-plugin-thermal-printer-api";

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
- **Automatic**: Paper is automatically cut at the end with a full cut
- **Manual** (optional): Add a `Cut` section if you need a specific partial cut
