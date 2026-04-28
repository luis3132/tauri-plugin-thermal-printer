package com.luis3132.thermal_printer

import android.app.Activity
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
import org.json.JSONArray
import org.json.JSONException

@TauriPlugin(
    permissions = [
        Permission(
            strings = [
                Manifest.permission.BLUETOOTH,
                Manifest.permission.BLUETOOTH_ADMIN,
                Manifest.permission.BLUETOOTH_SCAN,
                Manifest.permission.BLUETOOTH_CONNECT,
                Manifest.permission.ACCESS_FINE_LOCATION,
                Manifest.permission.ACCESS_COARSE_LOCATION
            ],
            alias = "bluetooth"
        )
    ]
)
class Thermal_Printer_Plugin(private val activity: Activity) : Plugin(activity) {

    private val TAG = "ThermalPrinterPlugin"

    // ─────────────────────────────────────────────────────────────
    // list_thermal_printers
    // ─────────────────────────────────────────────────────────────

    @Command
    fun list_thermal_printers(invoke: Invoke) {
        Log.d(TAG, "list_thermal_printers called")
        if (getPermissionState("bluetooth") != PermissionState.GRANTED) {
            Log.d(TAG, "Requesting bluetooth permissions")
            requestPermissionForAlias("bluetooth", invoke, "bluetoothListPermissionCallback")
            return
        }
        doListPrinters(invoke)
    }

    @PermissionCallback
    fun bluetoothListPermissionCallback(invoke: Invoke) {
        if (getPermissionState("bluetooth") != PermissionState.GRANTED) {
            Log.w(TAG, "Bluetooth permission denied")
            invoke.reject("Bluetooth permission denied")
            return
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
    // print_raw_data
    // ─────────────────────────────────────────────────────────────

    @Command
    fun print_raw_data(invoke: Invoke) {
        Log.d(TAG, "print_raw_data called")
        if (getPermissionState("bluetooth") != PermissionState.GRANTED) {
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

                val dataArray: JSONArray = try {
                    args.getJSONArray("data")
                } catch (e: JSONException) {
                    return@Thread invoke.reject("Missing print data")
                }

                val bytes = ByteArray(dataArray.length()) { i -> dataArray.getInt(i).toByte() }

                Log.d(TAG, "Printing to $identifier (${bytes.size} bytes)")

                if (identifier.startsWith("VID:")) {
                    // USB format: VID:1234/PID:5678
                    val parts = identifier.split("/")
                    val vid = parts[0].removePrefix("VID:").toIntOrNull() ?: 0
                    val pid = parts[1].removePrefix("PID:").toIntOrNull() ?: 0
                    
                    val printer = UsbPrinter(activity.applicationContext)
                    printer.printRawData(vid, pid, bytes)
                    
                } else if (identifier.contains(".") && identifier.contains(":")) {
                    // Network format: 192.168.1.100:9100
                    val parts = identifier.split(":")
                    val ip = parts[0]
                    val port = parts[1].toIntOrNull() ?: 9100
                    
                    val printer = NetworkPrinter()
                    printer.printRawData(ip, port, bytes)
                    
                } else {
                    // Default to Bluetooth
                    val printer = BluetoothPrinter(activity.applicationContext)
                    printer.printRawData(identifier, bytes)
                }

                invoke.resolve()

            } catch (e: Exception) {
                Log.e(TAG, "Print error: ${e.message}", e)
                invoke.reject("Print error: ${e.message}")
            }
        }.start()
    }
}
