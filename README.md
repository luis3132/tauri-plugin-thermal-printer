# Tauri Plugin thermal-printer

This plugin is actually in develop...

| Platform | Supported |
| -------- | --------- |
| Linux    | x         |
| Windows  | x         |
| macOS    | x         |
| Android  | x         |
| iOS      | x         |



## Functions (not implemented)

This plugin has 3 functions:

### 1. List Printers

Get all printers available in the system with a specific interface.

#### Request:
```typescript
const response = await invoke("plugin:tauri-plugin-thermal-printer|list-thermal-printers", { interface: "network"})

```

#### Response
```json
{
  "count": 2,
  "printers": [
    {
      "name": "TM-T20II",
      "description": "EPSON TM-T20II Receipt Printer",
      "status": "IDLE",
      "isDefault": true
    },
    {
      "name": "Star TSP143III",
      "description": "Star Micronics TSP143III",
      "status": "IDLE",
      "isDefault": false
    }
  ]
}
```

#### Response fields:
- `count` (number): count of finding printers
- `printers` (array): available printers
  - `name` (string): Name of the printer
  - `description` (string): Complete description of the printer
  - `status` (string): Current status (`IDLE`, `PROCESSING`, `STOPPED`, `UNKNOWN`)
  - `isDefault` (boolean): Indicates if it is the system's default printer

---

### 2. Test Printer

Send a print test to a specific printer to verify functionality.

#### Request:
```typescript
const response = await invoke("plugin:tauri-plugin-thermal-printer|test-thermal-printer",
{
  "printerName": "TM-T20II",
  "includeText": true,
  "includeTextStyles": true,
  "includeAlignment": true,
  "includeColumns": true,
  "includeSeparators": true,
  "includeBarcode": true,
  "includeBarcodeTypes": false,
  "includeQR": true,
  "includeImage": false,
  "imageBase64": null,
  "includeBeep": true,
  "testCashDrawer": false,
  "cutPaper": true,
  "testFeed": true,
  "testAllFonts": false,
  "testInvert": false,
  "testRotate": false
});
```

#### Request parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `printerName` | string | ✅ Yes | Printer name (obtained from the `/plugin/ThermalPrinter` endpoint) |
| `includeText` | boolean | ❌ No | Basic text test (default: `true`) |
| `includeTextStyles` | boolean | ❌ No | Text styles test (bold, underline, inverted) (default: `true`) |
| `includeAlignment` | boolean | ❌ No | Alignment test (left, center, right) (default: `true`) |
| `includeColumns` | boolean | ❌ No | Column tables test (default: `true`) |
| `includeSeparators` | boolean | ❌ No | Separator lines test (default: `true`) |
| `includeBarcode` | boolean | ❌ No | Barcode test (default: `true`) |
| `includeBarcodeTypes` | boolean | ❌ No | Multiple barcode types test (default: `false`) |
| `includeQR` | boolean | ❌ No | QR code test (default: `true`) |
| `includeImage` | boolean | ❌ No | Image printing test (default: `false`) |
| `imageBase64` | string | ❌ No | Base64 image for testing (only if `includeImage` is `true`) |
| `includeBeep` | boolean | ❌ No | Acoustic signal test (default: `true`) |
| `testCashDrawer` | boolean | ❌ No | Cash drawer opening test (default: `false`) |
| `cutPaper` | boolean | ❌ No | Cut paper at the end (default: `true`) |
| `testFeed` | boolean | ❌ No | Paper feed test (default: `true`) |
| `testAllFonts` | boolean | ❌ No | Test all available fonts (default: `false`) |
| `testInvert` | boolean | ❌ No | Inverted text test (default: `false`) |
| `testRotate` | boolean | ❌ No | Text rotation test (default: `false`) |

#### Response (200 OK):
```json
{
  "success": true,
  "message": "Test print sent successfully to TM-T20II"
}
```

#### Response (Error):
```json
{
  "success": false,
  "message": "Printer not found: TM-T20II"
}
```

**💡 Minimum example:**
```json
{
  "printerName": "TM-T20II"
}
```
This sends a basic test using default options.

### 3. Print Document

Print a personalized document with the specified sections.

#### Request:
```typescript
const response = await invoke("plugin:tauri-plugin-thermal-printer|print-thermal-printer",
{
  "printer": "TM-T20II",
  "paperSize": "80mm",
  "printOptions": {
    "autoNewline": true
  },
  "sections": [
    {"title": {"text": "My business"}},
    {"subtitle": {"text": "Date: 01/01/2000"}},
    {"type": "line", "content": {"char": "="}},
    {"type": "bold", "content": "TOTAL: $500.00"},
    {"type": "qr", "content": {"data": "https://example.com", "size": 5, "correction": "M"}},
    {"type": "feed", "content": {"lines": 3}}
  ]
}
```

#### Main parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `printer` | string | ✅ Yes | Printer name |
| `paperSize` | string | ❌ No | Paper size: `"58mm"` or `"80mm"` (default: `"80mm"`) |
| `printOptions` | object | ❌ No | Configuration options |
| `printOptions.autoNewline` | boolean | ❌ No | Add automatic newline after each section (default: `true`) |
| `sections` | array | ✅ Yes | Array of sections to print (see [Section Types](#section-types)) |

#### Section Types

Sections are defined as objects in the `sections` array. Each section has a specific type and corresponding content. Below are all supported section types:

##### Title
Prints a title with optional styles.

```json
{
  "title": {
    "text": "My Title",
    "styles": {
      "bold": false,
      "underline": false,
      "align": "center", // Doesn't work on title
      "italic": false,
      "invert": false,
      "font": "A",
      "rotate": false,
      "upside_down": false,
      "size": "normal" // Doesn't work on title
    }
  }
}
```

- `text` (string, required): Title text
- `styles` (object, optional): Applied styles (see [Global Styles](#global-styles))

##### Subtitle
Prints a subtitle with optional styles.

```json
{
  "subtitle": {
    "text": "My Subtitle",
    "styles": {
      "bold": false, // Doesn't work on subtitle
      "underline": false,
      "align": "center",
      "italic": false,
      "invert": false,
      "font": "A",
      "rotate": false,
      "upside_down": false,
      "size": "normal" // Doesn't work on subtitle
    }
  }
}
```

- `text` (string, required): Subtitle text
- `styles` (object, optional): Applied styles

##### Text
Prints simple text with optional styles.

```json
{
  "text": {
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
- `styles` (object, optional): Applied styles

##### Feed
Advances the paper by a specific number of lines.

```json
{
  "feed": {
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
  "cut": {
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
  "beep": {
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
  "drawer": {
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
  "qr": {
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
  "barcode": {
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
  "table": {
    "columns": 3,
    "column_widths": [10, 15, 10],
    "header": [
      {"text": "Col1", "styles": {}},
      {"text": "Col2", "styles": {}},
      {"text": "Col3", "styles": {}}
    ],
    "body": [
      [
        {"text": "Data1", "styles": {}},
        {"text": "Data2", "styles": {}},
        {"text": "Data3", "styles": {}}
      ]
    ],
    "truncate": false
  }
}
```

- `columns` (number, required): Number of columns
- `column_widths` (array, required): Widths of each column
- `header` (array, optional): Column headers (array of Text objects)
- `body` (array, required): Data rows (array of arrays of Text objects)
- `truncate` (boolean, optional): Truncate long text (default: false)

##### DataMatrix
Prints a DataMatrix code.

```json
{
  "data_matrix": {
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
  "pdf417": {
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
  "imagen": {
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
  "logo": {
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
  "line": {
    "character": "="
  }
}
```

- `character` (string, required): Character for the line (e.g., "=", "-", "_")

#### Global Styles

Styles apply to text sections (Title, Subtitle, Text) from where they are set and contain:

```json
{
    "global_styles": {
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

- `bold` (boolean): Bold text
- `underline` (boolean): Underlined text
- `align` (string): Alignment ("left", "center", "right")
- `italic` (boolean): Italic text
- `invert` (boolean): Inverted text (black background)
- `font` (string): Font ("A", "B", "C")
- `rotate` (boolean): Text rotated 90 degrees
- `upside_down` (boolean): Upside down text
- `size` (string): Size ("normal", "double_height", "double_width", "double_size")