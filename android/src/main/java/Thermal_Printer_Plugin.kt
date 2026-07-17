package com.luis3132.thermal_printer

import android.app.Activity
import android.util.Base64
import android.util.Log
import app.tauri.PermissionState
import app.tauri.annotation.Command
import app.tauri.annotation.Permission
import app.tauri.annotation.PermissionCallback
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import android.Manifest
import android.os.Build

@TauriPlugin(
    permissions = [
        // Solo runtime permissions realmente necesarios para leer dispositivos
        // emparejados y conectar. La ubicación NO se pide: solo leemos bondedDevices,
        // que no la requiere. USB y red no necesitan permisos runtime aquí.
        Permission(
            strings = [
                Manifest.permission.BLUETOOTH_CONNECT,
                Manifest.permission.BLUETOOTH_SCAN
            ],
            alias = "bluetooth"
        )
    ]
)
class Thermal_Printer_Plugin(private val activity: Activity) : Plugin(activity) {

    private val TAG = "ThermalPrinterPlugin"

    // Formatos de identificador: MAC (Bluetooth), "VID:.../PID:..." (USB), "host:port" (Red).
    private val macRegex = Regex("^([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}$")
    private val networkRegex = Regex("^\\d{1,3}(\\.\\d{1,3}){3}:\\d+$")

    private fun isBluetoothIdentifier(identifier: String) = macRegex.matches(identifier)

    // BLUETOOTH_CONNECT/SCAN solo son permisos runtime en Android 12+ (API 31).
    // En versiones anteriores el Bluetooth clásico se concede en tiempo de instalación.
    private fun needsBluetoothRuntimePermission() =
        Build.VERSION.SDK_INT >= Build.VERSION_CODES.S &&
            getPermissionState("bluetooth") != PermissionState.GRANTED

    // ─────────────────────────────────────────────────────────────
    // list_thermal_printers
    // ─────────────────────────────────────────────────────────────

    @Command
    fun list_thermal_printers(invoke: Invoke) {
        Log.d(TAG, "list_thermal_printers called")
        if (needsBluetoothRuntimePermission()) {
            Log.d(TAG, "Requesting bluetooth permissions")
            requestPermissionForAlias("bluetooth", invoke, "bluetoothListPermissionCallback")
            return
        }
        doListPrinters(invoke)
    }

    @PermissionCallback
    fun bluetoothListPermissionCallback(invoke: Invoke) {
        // Se procede aunque el Bluetooth se haya denegado: USB y red siguen funcionando,
        // y el descubrimiento Bluetooth verifica el permiso por su cuenta.
        if (getPermissionState("bluetooth") != PermissionState.GRANTED) {
            Log.w(TAG, "Bluetooth permission denied — listing USB/Network only")
        }
        doListPrinters(invoke)
    }

    private fun doListPrinters(invoke: Invoke) {
        Thread {
            try {
                val discovery = PrinterDiscovery(activity.applicationContext)
                val printers = discovery.discoverAllPrinters()

                Log.d(TAG, "Total printers found: ${printers.size}")

                val printersArray = JSArray()
                for (printer in printers) {
                    val obj = JSObject().apply {
                        put("name", printer.name)
                        put("interface_type", printer.interfaceType)
                        put("identifier", printer.identifier)
                        put("status", printer.status)
                    }
                    printersArray.put(obj)
                }

                val result = JSObject()
                result.put("printers", printersArray)

                Log.d(TAG, "Resolving with ${printers.size} printers")
                invoke.resolve(result)

            } catch (e: Exception) {
                Log.e(TAG, "Error listing printers: ${e.message}", e)
                invoke.reject("Error listing printers: ${e.message}")
            }
        }.start()
    }

    // ─────────────────────────────────────────────────────────────
    // print_raw_data — enruta por tipo de conexión
    // ─────────────────────────────────────────────────────────────

    @Command
    fun print_raw_data(invoke: Invoke) {
        Log.d(TAG, "print_raw_data called")

        val identifier = try {
            invoke.getArgs().getString("identifier", null)
        } catch (e: Exception) {
            null
        }

        // Solo la ruta Bluetooth necesita permiso runtime; USB usa su propio diálogo
        // y la red no necesita ninguno.
        if (identifier != null && isBluetoothIdentifier(identifier) &&
            needsBluetoothRuntimePermission()
        ) {
            Log.d(TAG, "Requesting bluetooth permissions for printing")
            requestPermissionForAlias("bluetooth", invoke, "bluetoothPrintPermissionCallback")
            return
        }
        doPrintRawData(invoke)
    }

    @PermissionCallback
    fun bluetoothPrintPermissionCallback(invoke: Invoke) {
        if (getPermissionState("bluetooth") != PermissionState.GRANTED) {
            Log.w(TAG, "Bluetooth permission denied for printing")
            invoke.reject("Bluetooth permission denied")
            return
        }
        doPrintRawData(invoke)
    }

    private fun doPrintRawData(invoke: Invoke) {
        Thread {
            try {
                val args = invoke.getArgs()

                val identifier = args.getString("identifier", null)
                    ?: return@Thread invoke.reject("Missing printer identifier")

                val dataB64 = args.getString("data", null)
                    ?: return@Thread invoke.reject("Missing print data")

                val bytes = try {
                    Base64.decode(dataB64, Base64.DEFAULT)
                } catch (e: IllegalArgumentException) {
                    return@Thread invoke.reject("Invalid print data encoding")
                }

                Log.d(TAG, "Printing to $identifier (${bytes.size} bytes)")

                val context = activity.applicationContext
                when {
                    identifier.startsWith("VID:") -> {
                        UsbPrinter(context).printRawData(identifier, bytes)
                    }
                    networkRegex.matches(identifier) -> {
                        NetworkPrinter().printRawData(identifier, bytes)
                    }
                    macRegex.matches(identifier) -> {
                        BluetoothPrinter(context).printRawData(identifier, bytes)
                    }
                    else -> {
                        return@Thread invoke.reject("Unrecognized printer identifier: $identifier")
                    }
                }

                invoke.resolve()

            } catch (e: Exception) {
                Log.e(TAG, "Print error: ${e.message}", e)
                invoke.reject("Print error: ${e.message}")
            }
        }.start()
    }
}
