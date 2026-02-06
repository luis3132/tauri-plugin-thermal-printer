# Tauri Plugin thermal-printer

This plugin provides thermal printer functionality for Tauri applications, allowing you to print documents, test printers, and list available printers.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| macOS    | ?         |
| Windows  | ?         |
| Android  | x         |
| iOS      | x         |

## Installation

### Rust

```toml
[dependencies]
tauri-plugin-thermal-printer = { git } # it's not published yet
```

### Bun / NPN / PNPM

```bash
# it's not published yet
```

### Lib.rs

Don't forget to add this line

```rust
.plugin(tauri_plugin_thermal_printer::init())
```

### Permission

Modify the file in /file/to/proyect/capabilities/default.json, and add:

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

## Functions

This plugin has 3 functions:

### 1. List Printers

Get all printers available in the system. It's just list the configurated printers...

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

### 2. Test Printer

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
    "sections": [] // it's not going to print something
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

### 3. Print Document

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
    {"Title": {"text": "My Business", "styles": {"bold": true, "underline": false, "align": "center", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "Double"}}},
    {"Subtitle": {"text": "Date: 01/01/2000", "styles": {"bold": true, "underline": false, "align": "left", "italic": false, "invert": false, "font": "A", "rotate": false, "upside_down": false, "size": "normal"}}},
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
    {"Imagen": {"data": "base64_encoded_image", "max_width": 384, "align": "center", "dithering": true, "size": "normal"}},
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

- `text` (string, required): Title text
- `styles` (GlobalStyles, required): Applied styles

##### Subtitle
Prints a subtitle with forced bold and normal size.

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
      "size": "normal"
    }
  }
}
```

- `text` (string, required): Subtitle text
- `styles` (GlobalStyles, required): Applied styles

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
    "model": 2
  }
}
```

- `data` (string, required): QR data
- `size` (number, required): Module size (1-16)
- `error_correction` (string, required): Error correction level ("L", "M", "Q", "H")
- `model` (number, required): QR model (1 or 2)

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

- `columns` (number, required): Number of columns
- `column_widths` (array, required): Widths of each column
- `header` (array, required): Column headers (array of Text objects)
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

##### Imagen
Prints an image.

```json
{
  "Imagen": {
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

- `bold` (boolean, required): Bold text
- `underline` (boolean, required): Underlined text
- `align` (string, required): Alignment ("left", "center", "right")
- `italic` (boolean, required): Italic text
- `invert` (boolean, required): Inverted text (black background)
- `font` (string, required): Font ("A", "B", "C")
- `rotate` (boolean, required): Text rotated 90 degrees
- `upside_down` (boolean, required): Upside down text
- `size` (string, required): Size ("normal", "double_height", "double_width", "double_size")