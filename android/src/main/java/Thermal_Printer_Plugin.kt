package com.luis3132.thermal_printer

import android.app.Activity
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import com.google.gson.Gson
import org.json.JSONArray
import org.json.JSONObject

@InvokeArg
class PingArgs {
    var value: String? = null
}

@TauriPlugin
class Thermal_Printer_Plugin(private val activity: Activity): Plugin(activity) {
    
    private val TAG = "ThermalPrinterPlugin"
    
    @Command
    suspend fun list_thermal_printers(invoke: Invoke) {
        Log.d(TAG, "Starting list_thermal_printers command")
        try {
            Log.d(TAG, "Creating PrinterDiscovery instance")
            val printerDiscovery = PrinterDiscovery(activity.applicationContext)
            
            Log.d(TAG, "Discovering printers...")
            val printers = printerDiscovery.discoverAllPrinters()
            Log.d(TAG, "Found ${printers.size} printers")
            
            // Crear un array JSON directamente
            val printersArray = JSONArray()
            printers.forEach { printer ->
                Log.d(TAG, "Processing printer: ${printer.name} (${printer.interfaceType})")
                val printerObj = JSONObject().apply {
                    put("name", printer.name)
                    put("interfaceType", printer.interfaceType)
                    put("identifier", printer.identifier)
                    put("status", printer.status)
                }
                printersArray.put(printerObj)
            }
            
            Log.d(TAG, "Printers JSON Array: ${printersArray.toString()}")
            
            // Retornar el array directamente como string JSON
            val result = JSObject()
            result.put("printers", printersArray.toString())
            
            Log.d(TAG, "Resolving with result")
            invoke.resolve(result)
        } catch (e: Exception) {
            Log.e(TAG, "Error listing printers", e)
            Log.e(TAG, "Error message: ${e.message}")
            Log.e(TAG, "Stack trace: ${e.stackTraceToString()}")
            invoke.reject("Error listing printers: ${e.message}\nStack: ${e.stackTraceToString()}")
        }
    }
}