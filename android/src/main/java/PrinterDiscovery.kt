package com.luis3132.thermal_printer

import android.Manifest
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.content.Context
import android.content.pm.PackageManager
import android.hardware.usb.UsbDevice
import android.hardware.usb.UsbManager
import android.os.Build
import androidx.core.app.ActivityCompat
import kotlinx.coroutines.suspendCancellableCoroutine
import java.net.InetAddress
import kotlin.coroutines.resume

// Cambiar el nombre para evitar conflicto
data class ThermalPrinterInfo(
    val name: String,
    val interfaceType: String,
    val identifier: String,
    val status: String
)

class PrinterDiscovery(private val context: Context) {

    private val usbManager: UsbManager by lazy {
        context.getSystemService(Context.USB_SERVICE) as UsbManager
    }

    private val bluetoothManager: BluetoothManager? by lazy {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            context.getSystemService(BluetoothManager::class.java)
        } else {
            @Suppress("DEPRECATION")
            context.getSystemService(Context.BLUETOOTH_SERVICE) as? BluetoothManager
        }
    }

    /**
     * Lista todas las impresoras disponibles
     */
    suspend fun discoverAllPrinters(): List<ThermalPrinterInfo> {
        val printers = mutableListOf<ThermalPrinterInfo>()
        
        // Descubrir impresoras USB
        printers.addAll(discoverUsbPrinters())
        
        // Descubrir impresoras Bluetooth
        printers.addAll(discoverBluetoothPrinters())
        
        return printers
    }

    /**
     * Descubre impresoras USB conectadas
     */
    private fun discoverUsbPrinters(): List<ThermalPrinterInfo> {
        val printers = mutableListOf<ThermalPrinterInfo>()
        
        val deviceList: Map<String, UsbDevice> = usbManager.deviceList
        
        deviceList.values.forEach { device ->
            // Clase 7 = Impresoras (USB Device Class)
            if (device.deviceClass == 7 || hasUsbPrinterInterface(device)) {
                printers.add(
                    ThermalPrinterInfo(
                        name = device.productName ?: device.manufacturerName ?: "USB Printer",
                        interfaceType = "USB",
                        identifier = "VID:${device.vendorId}/PID:${device.productId}",
                        status = if (usbManager.hasPermission(device)) "Connected" else "Permission Required"
                    )
                )
            }
        }
        
        return printers
    }

    /**
     * Verifica si el dispositivo USB tiene una interfaz de impresora
     */
    private fun hasUsbPrinterInterface(device: UsbDevice): Boolean {
        for (i in 0 until device.interfaceCount) {
            val usbInterface = device.getInterface(i)
            // Clase 7 = Impresoras
            if (usbInterface.interfaceClass == 7) {
                return true
            }
        }
        return false
    }

    /**
     * Descubre impresoras Bluetooth emparejadas
     */
    private fun discoverBluetoothPrinters(): List<ThermalPrinterInfo> {
        val printers = mutableListOf<ThermalPrinterInfo>()
        
        val bluetoothAdapter = bluetoothManager?.adapter
        
        if (bluetoothAdapter == null || !bluetoothAdapter.isEnabled) {
            return printers
        }

        // Verificar permisos
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            if (ActivityCompat.checkSelfPermission(
                    context,
                    Manifest.permission.BLUETOOTH_CONNECT
                ) != PackageManager.PERMISSION_GRANTED
            ) {
                return printers
            }
        }

        // Obtener dispositivos emparejados
        val pairedDevices: Set<BluetoothDevice> = bluetoothAdapter.bondedDevices
        
        pairedDevices.forEach { device ->
            // Filtrar por clase de dispositivo (1664 = Impresoras)
            val deviceClass = device.bluetoothClass?.deviceClass
            
            val isPrinter = deviceClass == 1664 || 
                           device.name?.contains("printer", true) == true ||
                           device.name?.contains("print", true) == true
            
            if (isPrinter) {
                printers.add(
                    ThermalPrinterInfo(
                        name = device.name ?: "Bluetooth Printer",
                        interfaceType = "Bluetooth",
                        identifier = device.address,
                        status = when (device.bondState) {
                            BluetoothDevice.BOND_BONDED -> "Paired"
                            BluetoothDevice.BOND_BONDING -> "Pairing"
                            else -> "Not Paired"
                        }
                    )
                )
            }
        }
        
        return printers
    }

    /**
     * Escanear red para impresoras de red
     */
    suspend fun scanNetworkRange(
        baseIp: String = "192.168.1",
        startRange: Int = 1,
        endRange: Int = 254,
        port: Int = 9100
    ): List<ThermalPrinterInfo> {
        val printers = mutableListOf<ThermalPrinterInfo>()
        
        for (i in startRange..endRange) {
            try {
                val ip = "$baseIp.$i"
                val address = InetAddress.getByName(ip)
                
                if (address.isReachable(100)) {
                    try {
                        val socket = java.net.Socket()
                        socket.connect(
                            java.net.InetSocketAddress(address, port),
                            200
                        )
                        
                        printers.add(
                            ThermalPrinterInfo(
                                name = "Network Printer",
                                interfaceType = "Network",
                                identifier = "$ip:$port",
                                status = "Available"
                            )
                        )
                        
                        socket.close()
                    } catch (e: Exception) {
                        // Puerto no disponible
                    }
                }
            } catch (e: Exception) {
                // IP no alcanzable
            }
        }
        
        return printers
    }
}