package com.luis3132.thermal_printer

import android.Manifest
import android.bluetooth.BluetoothClass
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.content.Context
import android.content.pm.PackageManager
import android.hardware.usb.UsbDevice
import android.hardware.usb.UsbManager
import android.net.nsd.NsdManager
import android.net.nsd.NsdServiceInfo
import android.os.Build
import android.util.Log
import androidx.core.app.ActivityCompat
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

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

        safeDiscover("Network") { discoverNetworkPrinters() }.let { printers.addAll(it) }

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
    // Red / Network — descubrimiento automático vía NSD (mDNS/DNS-SD)
    // ─────────────────────────────────────────────────────────────

    /**
     * Descubre impresoras de red anunciadas por mDNS/Bonjour sin escanear la subred.
     * Prioriza `_pdl-datastream._tcp` (RAW/JetDirect, el que entiende ESC/POS por socket).
     * Es sincrónico (bloquea con latches) porque se invoca desde un Thread de fondo.
     */
    private fun discoverNetworkPrinters(): List<ThermalPrinterInfo> {
        val nsdManager = context.getSystemService(Context.NSD_SERVICE) as? NsdManager
                ?: run {
                    Log.w(TAG, "NSD service not available")
                    return emptyList()
                }

        val discovered = java.util.Collections.synchronizedList(mutableListOf<NsdServiceInfo>())
        val listeners = mutableListOf<NsdManager.DiscoveryListener>()

        for (serviceType in NETWORK_SERVICE_TYPES) {
            try {
                val listener = object : NsdManager.DiscoveryListener {
                    override fun onDiscoveryStarted(type: String) {}
                    override fun onServiceFound(info: NsdServiceInfo) {
                        Log.d(TAG, "NSD service found: ${info.serviceName} (${info.serviceType})")
                        discovered.add(info)
                    }
                    override fun onServiceLost(info: NsdServiceInfo) {}
                    override fun onDiscoveryStopped(type: String) {}
                    override fun onStartDiscoveryFailed(type: String, code: Int) {
                        Log.w(TAG, "NSD start failed for $type (code $code)")
                    }
                    override fun onStopDiscoveryFailed(type: String, code: Int) {}
                }
                nsdManager.discoverServices(serviceType, NsdManager.PROTOCOL_DNS_SD, listener)
                listeners.add(listener)
            } catch (e: Exception) {
                Log.w(TAG, "NSD discovery error for $serviceType: ${e.message}")
            }
        }

        // Ventana de descubrimiento acotada.
        try { Thread.sleep(DISCOVERY_WINDOW_MS) } catch (_: InterruptedException) {}

        for (listener in listeners) {
            try { nsdManager.stopServiceDiscovery(listener) } catch (_: Exception) {}
        }

        // host -> (nombre, puerto). El puerto RAW (9100) tiene prioridad sobre 631/515.
        val byHost = LinkedHashMap<String, ThermalPrinterInfo>()
        for (service in discovered.toList()) {
            val resolved = resolveService(nsdManager, service) ?: continue
            @Suppress("DEPRECATION")
            val host = resolved.host?.hostAddress ?: continue
            val isRaw = service.serviceType.contains("pdl-datastream", ignoreCase = true)
            val port = if (isRaw && resolved.port > 0) resolved.port else RAW_PRINT_PORT
            val identifier = "$host:$port"

            val existing = byHost[host]
            if (existing == null || (isRaw && !existing.identifier.endsWith(":$RAW_PRINT_PORT"))) {
                byHost[host] = ThermalPrinterInfo(
                        name = resolved.serviceName?.ifBlank { "Network Printer" }
                                ?: "Network Printer @ $host",
                        interfaceType = "Network",
                        identifier = identifier,
                        status = "Available"
                )
            }
        }

        return byHost.values.toList()
    }

    private fun resolveService(
            nsdManager: NsdManager,
            service: NsdServiceInfo
    ): NsdServiceInfo? {
        val latch = CountDownLatch(1)
        var result: NsdServiceInfo? = null
        try {
            nsdManager.resolveService(service, object : NsdManager.ResolveListener {
                override fun onResolveFailed(info: NsdServiceInfo, code: Int) {
                    Log.w(TAG, "NSD resolve failed for ${info.serviceName} (code $code)")
                    latch.countDown()
                }
                override fun onServiceResolved(info: NsdServiceInfo) {
                    result = info
                    latch.countDown()
                }
            })
            latch.await(RESOLVE_TIMEOUT_SECONDS, TimeUnit.SECONDS)
        } catch (e: Exception) {
            // resolveService lanza IllegalArgumentException si ya hay un resolve activo;
            // los resolvemos en serie, así que esto es defensivo.
            Log.w(TAG, "NSD resolve error: ${e.message}")
        }
        return result
    }

    companion object {
        private const val USB_CLASS_PRINTER = 7
        private const val BT_PRINTER_CLASS = 1664 // 0x0680 — Imaging / Printer
        private const val RAW_PRINT_PORT = 9100
        private const val DISCOVERY_WINDOW_MS = 3500L
        private const val RESOLVE_TIMEOUT_SECONDS = 3L
        private val NETWORK_SERVICE_TYPES = listOf(
                "_pdl-datastream._tcp.", // RAW / JetDirect (puerto 9100) — ESC/POS
                "_printer._tcp.",        // LPD
                "_ipp._tcp."             // IPP
        )
    }
}
