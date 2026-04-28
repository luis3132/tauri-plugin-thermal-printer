package com.luis3132.thermal_printer

import android.Manifest
import android.bluetooth.BluetoothClass
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.content.Context
import android.content.pm.PackageManager
import android.hardware.usb.UsbDevice
import android.hardware.usb.UsbManager
import android.os.Build
import android.util.Log
import androidx.core.app.ActivityCompat
import java.net.NetworkInterface
import java.net.InetSocketAddress
import java.net.Socket
import java.util.Collections
import java.util.concurrent.Callable
import java.util.concurrent.Executors
import java.util.concurrent.Future

/**
 * Información de una impresora descubierta. Los nombres de campo deben coincidir con el struct
 * PrinterInfo en models.rs
 */
data class ThermalPrinterInfo(
        val name: String,
        val interfaceType: String, // se serializa como "interface_type" vía JSObject
        val identifier: String,
        val status: String
)

class PrinterDiscovery(private val context: Context) {

    private val TAG = "PrinterDiscovery"

    private val usbManager: UsbManager by lazy {
        context.getSystemService(Context.USB_SERVICE) as UsbManager
    }

    private val bluetoothAdapter by lazy {
        val manager =
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
                    context.getSystemService(BluetoothManager::class.java)
                } else {
                    @Suppress("DEPRECATION")
                    context.getSystemService(Context.BLUETOOTH_SERVICE) as? BluetoothManager
                }
        manager?.adapter
    }

    // ─────────────────────────────────────────────────────────────
    // Punto de entrada principal — NO suspend, se llama desde Thread{}
    // ─────────────────────────────────────────────────────────────

    fun discoverAllPrinters(): List<ThermalPrinterInfo> {
        val printers = mutableListOf<ThermalPrinterInfo>()

        safeDiscover("USB") { discoverUsbPrinters() }.let { printers.addAll(it) }

        safeDiscover("Bluetooth") { discoverBluetoothPrinters() }.let { printers.addAll(it) }

        // Escaneo de red optimizado con ThreadPool
        safeDiscover("Network") { scanNetworkPrinters() }.let { printers.addAll(it) }

        Log.d(TAG, "Total printers found: ${printers.size}")
        return printers
    }

    private fun safeDiscover(
            type: String,
            block: () -> List<ThermalPrinterInfo>
    ): List<ThermalPrinterInfo> {
        return try {
            val result = block()
            Log.d(TAG, "[$type] Found ${result.size} printer(s)")
            result
        } catch (e: Exception) {
            Log.e(TAG, "[$type] Discovery error: ${e.message}", e)
            emptyList()
        }
    }

    // ─────────────────────────────────────────────────────────────
    // USB
    // ─────────────────────────────────────────────────────────────

    private fun discoverUsbPrinters(): List<ThermalPrinterInfo> {
        val printers = mutableListOf<ThermalPrinterInfo>()
        val deviceList: Map<String, UsbDevice> = usbManager.deviceList
        Log.d(TAG, "USB devices found: ${deviceList.size}")

        for (device in deviceList.values) {
            Log.d(
                    TAG,
                    "USB device — name=${device.productName} " +
                            "VID=${device.vendorId} PID=${device.productId} " +
                            "class=${device.deviceClass}"
            )

            if (device.deviceClass == USB_CLASS_PRINTER || hasUsbPrinterInterface(device)) {
                printers.add(
                        ThermalPrinterInfo(
                                name = device.productName
                                                ?: device.manufacturerName ?: "USB Printer",
                                interfaceType = "USB",
                                identifier = "VID:${device.vendorId}/PID:${device.productId}",
                                status =
                                        if (usbManager.hasPermission(device)) "Connected"
                                        else "Permission Required"
                        )
                )
            }
        }
        return printers
    }

    private fun hasUsbPrinterInterface(device: UsbDevice): Boolean {
        for (i in 0 until device.interfaceCount) {
            if (device.getInterface(i).interfaceClass == USB_CLASS_PRINTER) return true
        }
        return false
    }

    // ─────────────────────────────────────────────────────────────
    // Bluetooth
    // ─────────────────────────────────────────────────────────────

    private fun discoverBluetoothPrinters(): List<ThermalPrinterInfo> {
        val printers = mutableListOf<ThermalPrinterInfo>()
        val adapter =
                bluetoothAdapter
                        ?: run {
                            Log.w(TAG, "Bluetooth adapter not available")
                            return printers
                        }

        if (!adapter.isEnabled) {
            Log.w(TAG, "Bluetooth disabled")
            return printers
        }

        // Verificar permisos en Android 12+
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            if (ActivityCompat.checkSelfPermission(
                            context,
                            Manifest.permission.BLUETOOTH_CONNECT
                    ) != PackageManager.PERMISSION_GRANTED
            ) {
                Log.w(TAG, "BLUETOOTH_CONNECT permission not granted")
                return printers
            }
        }

        val pairedDevices: Set<BluetoothDevice> = adapter.bondedDevices ?: emptySet()
        Log.d(TAG, "Paired BT devices: ${pairedDevices.size}")

        for (device in pairedDevices) {
            try {
                val deviceClass = device.bluetoothClass?.deviceClass ?: -1
                val name = device.name ?: ""
                Log.d(TAG, "BT device — name=$name class=$deviceClass addr=${device.address}")

                val isPrinter =
                        deviceClass == BluetoothClass.Device.Major.IMAGING ||
                                deviceClass == BT_PRINTER_CLASS ||
                                name.contains("printer", ignoreCase = true) ||
                                name.contains("print", ignoreCase = true) ||
                                name.contains("POS", ignoreCase = true) ||
                                name.contains("thermal", ignoreCase = true)

                if (isPrinter) {
                    printers.add(
                            ThermalPrinterInfo(
                                    name = name.ifBlank { "Bluetooth Printer" },
                                    interfaceType = "Bluetooth",
                                    identifier = device.address,
                                    status =
                                            when (device.bondState) {
                                                BluetoothDevice.BOND_BONDED -> "Paired"
                                                BluetoothDevice.BOND_BONDING -> "Pairing"
                                                else -> "Not Paired"
                                            }
                            )
                    )
                }
            } catch (e: Exception) {
                Log.e(TAG, "Error processing BT device: ${e.message}")
            }
        }
        return printers
    }

    // ─────────────────────────────────────────────────────────────
    // Red / Network (puerto ESC/POS estándar 9100)
    // ─────────────────────────────────────────────────────────────

    /**
     * Escanea un rango de IPs buscando impresoras en el puerto 9100. Llama a este método en un
     * thread separado — puede tomar ~25 s para /24. Puedes reducir el rango para acelerar.
     */
    fun scanNetworkPrinters(
        subnet: String? = null,
        start: Int = 1,
        end: Int = 254,
        port: Int = 9100,
        connectTimeoutMs: Int = 200
    ): List<ThermalPrinterInfo> {
        val activeSubnet = subnet ?: getLocalSubnet() ?: "192.168.1"
        val printers = mutableListOf<ThermalPrinterInfo>()
        Log.d(TAG, "Network scan: $activeSubnet.$start-$end port $port")

        val executor = Executors.newFixedThreadPool(50) // Escaneo paralelo
        val futures = mutableListOf<Future<ThermalPrinterInfo?>>()

        for (i in start..end) {
            val ip = "$activeSubnet.$i"
            futures.add(executor.submit(Callable {
                try {
                    Socket().use { socket ->
                        socket.connect(InetSocketAddress(ip, port), connectTimeoutMs)
                        Log.d(TAG, "Network printer found at $ip:$port")
                        ThermalPrinterInfo(
                            name          = "Network Printer @ $ip",
                            interfaceType = "Network",
                            identifier    = "$ip:$port",
                            status        = "Available"
                        )
                    }
                } catch (_: Exception) {
                    null // Host inalcanzable o puerto cerrado
                }
            }))
        }

        executor.shutdown()
        
        for (future in futures) {
            try {
                val result = future.get()
                if (result != null) {
                    printers.add(result)
                }
            } catch (e: Exception) {
                // Ignorar
            }
        }

        Log.d(TAG, "Network scan finished. Found ${printers.size} printer(s)")
        return printers
    }

    private fun getLocalSubnet(): String? {
        try {
            val interfaces = NetworkInterface.getNetworkInterfaces()
            for (intf in Collections.list(interfaces)) {
                for (addr in Collections.list(intf.inetAddresses)) {
                    if (!addr.isLoopbackAddress && !addr.isLinkLocalAddress) {
                        val sAddr = addr.hostAddress
                        // Ignorar IPv6 (que contienen ':')
                        if (sAddr != null && sAddr.indexOf(':') < 0) {
                            Log.d(TAG, "Local IP detected: $sAddr")
                            val lastDot = sAddr.lastIndexOf('.')
                            if (lastDot > 0) {
                                return sAddr.substring(0, lastDot)
                            }
                        }
                    }
                }
            }
        } catch (e: Exception) {
            Log.e(TAG, "Error discovering local subnet: ${e.message}")
        }
        return null
    }

    companion object {
        private const val USB_CLASS_PRINTER = 7
        private const val BT_PRINTER_CLASS = 1664 // 0x0680 — Imaging / Printer
    }
}
