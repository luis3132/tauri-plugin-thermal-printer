package com.luis3132.thermal_printer

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import com.google.gson.Gson

@InvokeArg
class PingArgs {
    var value: String? = null
}

@TauriPlugin
class Thermal_Printer_Plugin(private val activity: Activity): Plugin(activity) {
    
    @Command
    suspend fun list_thermal_printers(invoke: Invoke) {
        try {
            val printerDiscovery = PrinterDiscovery(activity.applicationContext)
            val printers = printerDiscovery.discoverAllPrinters()
            
            // Convertir a formato JSON usando Gson
            val gson = Gson()
            val printersJson = printers.map { printer ->
                mapOf(
                    "name" to printer.name,
                    "interfaceType" to printer.interfaceType,
                    "identifier" to printer.identifier,
                    "status" to printer.status
                )
            }
            
            val result = JSObject()
            result.put("printers", gson.toJson(printersJson))
            invoke.resolve(result)
        } catch (e: Exception) {
            invoke.reject("Error listing printers: ${e.message}")
        }
    }
}