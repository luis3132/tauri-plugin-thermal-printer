# Driver Usage Guide

Este plugin soporta múltiples formas de conectarse a impresoras térmicas. El usuario debe especificar qué tipo de conexión desea usar.

## Tipos de Conexión Disponibles

### 1. Sistema (Default) - Usar impresoras del sistema operativo

Usa el sistema de impresión del OS (comando `lp` en Linux/macOS, acceso directo en Windows).

**Listar impresoras del sistema:**
```rust
// Rust
let printers = app.thermal_printer().list_system_printers()?;

// JavaScript/TypeScript
import { invoke } from '@tauri-apps/api/core';
const printers = await invoke('plugin:thermal-printer|list_system_printers');
```

**Imprimir usando impresora del sistema:**
```javascript
await invoke('plugin:thermal-printer|print', {
  request: {
    printer: "thermal_printer_name",
    sections: [...],
    options: {...},
    paper_size: "mm80"
  }
});
```

O especificar explícitamente la conexión:
```javascript
await invoke('plugin:thermal-printer|print', {
  request: {
    printer: "", // opcional cuando se usa connection
    connection: {
      type: "system",
      printer_name: "thermal_printer_name"
    },
    sections: [...],
    options: {...},
    paper_size: "mm80"
  }
});
```

### 2. Red (Network) - TCP/IP

Conectarse directamente a una impresora de red vía TCP/IP (generalmente puerto 9100).

**Imprimir vía red:**
```javascript
await invoke('plugin:thermal-printer|print', {
  request: {
    printer: "",
    connection: {
      type: "network",
      host: "192.168.1.100",
      port: 9100,
      timeout_secs: 5
    },
    sections: [...],
    options: {...},
    paper_size: "mm80"
  }
});
```

### 3. USB (requiere feature `usb`)

Conectarse directamente a un dispositivo USB usando vendor_id y product_id.

**Habilitar en Cargo.toml:**
```toml
[dependencies]
tauri-plugin-thermal-printer = { version = "0.1.0", features = ["usb"] }
```

**Listar dispositivos USB:**
```javascript
// Retorna array de { vendor_id, product_id, name }
const usbDevices = await invoke('plugin:thermal-printer|list_usb_devices');
```

**Imprimir vía USB:**
```javascript
await invoke('plugin:thermal-printer|print', {
  request: {
    printer: "",
    connection: {
      type: "usb",
      vendor_id: 0x0525,
      product_id: 0xa700,
      timeout_secs: 5
    },
    sections: [...],
    options: {...},
    paper_size: "mm80"
  }
});
```

### 4. Puerto Serial (requiere feature `serial_port`)

Conectarse a través de puerto serial/COM.

**Habilitar en Cargo.toml:**
```toml
[dependencies]
tauri-plugin-thermal-printer = { version = "0.1.0", features = ["serial_port"] }
```

**Listar puertos seriales:**
```javascript
// Retorna array de strings con nombres de puertos
const ports = await invoke('plugin:thermal-printer|list_serial_ports');
// Ejemplo: ["/dev/ttyUSB0", "/dev/ttyUSB1"] en Linux
// Ejemplo: ["COM1", "COM3"] en Windows
```

**Imprimir vía serial:**
```javascript
await invoke('plugin:thermal-printer|print', {
  request: {
    printer: "",
    connection: {
      type: "serial",
      port: "/dev/ttyUSB0", // o "COM1" en Windows
      baud_rate: 115200,
      timeout_secs: 5
    },
    sections: [...],
    options: {...},
    paper_size: "mm80"
  }
});
```

### 5. Archivo (File) - Acceso directo a dispositivo

Escribir directamente a un archivo de dispositivo (Linux/Unix).

**Imprimir vía archivo:**
```javascript
await invoke('plugin:thermal-printer|print', {
  request: {
    printer: "",
    connection: {
      type: "file",
      path: "/dev/usb/lp0"
    },
    sections: [...],
    options: {...},
    paper_size: "mm80"
  }
});
```

### 6. Consola (Console) - Para debugging

Enviar salida a la consola en lugar de una impresora real.

**Imprimir a consola:**
```javascript
await invoke('plugin:thermal-printer|print', {
  request: {
    printer: "",
    connection: {
      type: "console",
      show_output: true
    },
    sections: [...],
    options: {...},
    paper_size: "mm80"
  }
});
```

## Ejemplo Completo de Flujo

```javascript
import { invoke } from '@tauri-apps/api/core';

// 1. Primero, el usuario elige el tipo de conexión
const connectionType = "network"; // o "usb", "serial", "system", etc.

// 2. Listar dispositivos disponibles según el tipo
let availableDevices = [];

switch(connectionType) {
  case "system":
    availableDevices = await invoke('plugin:thermal-printer|list_system_printers');
    break;
  case "usb":
    availableDevices = await invoke('plugin:thermal-printer|list_usb_devices');
    break;
  case "serial":
    availableDevices = await invoke('plugin:thermal-printer|list_serial_ports');
    break;
  case "network":
    // Para red, el usuario debe ingresar IP manualmente
    availableDevices = [{ host: "192.168.1.100", port: 9100 }];
    break;
}

// 3. Usuario selecciona dispositivo y configura conexión
const connection = {
  type: "network",
  host: "192.168.1.100",
  port: 9100,
  timeout_secs: 5
};

// 4. Imprimir
await invoke('plugin:thermal-printer|print', {
  request: {
    printer: "",
    connection: connection,
    sections: [
      {
        Title: {
          text: "TICKET DE VENTA",
          styles: {
            align: "center",
            bold: true,
            size: "double"
          }
        }
      },
      {
        Text: {
          text: "Gracias por su compra\n",
          styles: {
            align: "center",
            bold: false,
            size: "normal"
          }
        }
      },
      {
        Cut: {
          mode: "partial",
          feed: 4
        }
      }
    ],
    options: {
      cut_paper: true,
      beep: false,
      open_cash_drawer: false
    },
    paper_size: "mm80"
  }
});
```

## Notas de Plataforma

- **Linux**: Sistema, Red, USB, Serial, y File funcionan
- **macOS**: Sistema, Red funcionan; USB y Serial requieren permisos especiales
- **Windows**: Sistema (limitado), Red, USB y Serial funcionan; File no soportado

## Habilitar Features

Por defecto, solo las conexiones Sistema, Red, File y Console están disponibles.

Para habilitar USB y/o Serial, modifica tu `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-thermal-printer = { version = "0.1.0", features = ["usb", "serial_port"] }
```
