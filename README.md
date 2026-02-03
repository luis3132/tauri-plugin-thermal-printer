# Tauri Plugin thermal-printer

| Platform | Supported |
| -------- | --------- |
| Linux    | x         |
| Windows  | x         |
| macOS    | x         |
| Android  | x         |
| iOS      | x         |


The plugin is actually in develop...



## Functions

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
  - `name` (string): Name of the rinter
  - `description` (string): Descripción completa de la impresora
  - `status` (string): Estado actual (`IDLE`, `PROCESSING`, `STOPPED`, `UNKNOWN`)
  - `isDefault` (boolean): Indica si es la impresora predeterminada del sistema

---

### 2. Test Printer

Sent a print test to a specific printer to proof the functionality

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

#### Parámetros de la petición:

| Parámetro | Tipo | Requerido | Descripción |
|-----------|------|-----------|-------------|
| `printerName` | string | ✅ Sí | Nombre de la impresora (obtenido del endpoint `/plugin/ThermalPrinter`) |
| `includeText` | boolean | ❌ No | Prueba de texto básico (default: `true`) |
| `includeTextStyles` | boolean | ❌ No | Prueba de estilos (negrita, subrayado, invertido) (default: `true`) |
| `includeAlignment` | boolean | ❌ No | Prueba de alineación (izquierda, centro, derecha) (default: `true`) |
| `includeColumns` | boolean | ❌ No | Prueba de tablas con columnas (default: `true`) |
| `includeSeparators` | boolean | ❌ No | Prueba de líneas separadoras (default: `true`) |
| `includeBarcode` | boolean | ❌ No | Prueba de código de barras (default: `true`) |
| `includeBarcodeTypes` | boolean | ❌ No | Prueba de múltiples tipos de códigos de barras (default: `false`) |
| `includeQR` | boolean | ❌ No | Prueba de código QR (default: `true`) |
| `includeImage` | boolean | ❌ No | Prueba de impresión de imagen (default: `false`) |
| `imageBase64` | string | ❌ No | Imagen en Base64 para probar (solo si `includeImage` es `true`) |
| `includeBeep` | boolean | ❌ No | Prueba de señal acústica (default: `true`) |
| `testCashDrawer` | boolean | ❌ No | Prueba de apertura de cajón de dinero (default: `false`) |
| `cutPaper` | boolean | ❌ No | Cortar papel al finalizar (default: `true`) |
| `testFeed` | boolean | ❌ No | Prueba de alimentación de papel (default: `true`) |
| `testAllFonts` | boolean | ❌ No | Prueba de todas las fuentes disponibles (default: `false`) |
| `testInvert` | boolean | ❌ No | Prueba de texto invertido (default: `false`) |
| `testRotate` | boolean | ❌ No | Prueba de rotación de texto (default: `false`) |

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
This sent a basic test using default options.

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
    {"subtittle": {"text": "Date: 01/01/2000"}},
    {"type": "line", "content": {"char": "="}},
    {"type": "bold", "content": "TOTAL: $500.00"},
    {"type": "qr", "content": {"data": "https://ejemplo.com", "size": 5, "correction": "M"}},
    {"type": "feed", "content": {"lines": 3}}
  ]
}
```

#### Parámetros principales:

| Parámetro | Tipo | Requerido | Descripción |
|-----------|------|-----------|-------------|
| `printer` | string | ✅ Sí | Nombre de la impresora |
| `paperSize` | string | ❌ No | Tamaño del papel: `"58mm"` o `"80mm"` (default: `"80mm"`) |
| `printOptions` | object | ❌ No | Opciones de configuración |
| `printOptions.autoNewline` | boolean | ❌ No | Agregar salto de línea automático después de cada sección (default: `true`) |
| `sections` | array | ✅ Sí | Array de secciones a imprimir (ver [Tipos de secciones](#tipos-de-secciones)) |