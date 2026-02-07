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
import android.util.Log
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

    private val TAG = "PrinterDiscovery"

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
        Log.d(TAG, "Starting printer discovery")
        val printers = mutableListOf<ThermalPrinterInfo>()
        
        try {
            // Descubrir impresoras USB
            Log.d(TAG, "Discovering USB printers...")
            val usbPrinters = discoverUsbPrinters()
            Log.d(TAG, "Found ${usbPrinters.size} USB printers")
            printers.addAll(usbPrinters)
        } catch (e: Exception) {
            Log.e(TAG, "Error discovering USB printers", e)
        }
        
        try {
            // Descubrir impresoras Bluetooth
            Log.d(TAG, "Discovering Bluetooth printers...")
            val btPrinters = discoverBluetoothPrinters()
            Log.d(TAG, "Found ${btPrinters.size} Bluetooth printers")
            printers.addAll(btPrinters)
        } catch (e: Exception) {
            Log.e(TAG, "Error discovering Bluetooth printers", e)
        }
        
        Log.d(TAG, "Total printers found: ${printers.size}")
        return printers
    }

    /**
     * Descubre impresoras USB conectadas
     */
    private fun discoverUsbPrinters(): List<ThermalPrinterInfo> {
        Log.d(TAG, "Starting USB printer discovery")
        val printers = mutableListOf<ThermalPrinterInfo>()
        
        try {
            val deviceList: Map<String, UsbDevice> = usbManager.deviceList
            Log.d(TAG, "Found ${deviceList.size} USB devices")
            
            deviceList.values.forEach { device ->
                Log.d(TAG, "Checking USB device: VID=${device.vendorId}, PID=${device.productId}, Class=${device.deviceClass}")
                // Clase 7 = Impresoras (USB Device Class)
                if (device.deviceClass == 7 || hasUsbPrinterInterface(device)) {
                    val printerInfo = ThermalPrinterInfo(
                        name = device.productName ?: device.manufacturerName ?: "USB Printer",
                        interfaceType = "USB",
                        identifier = "VID:${device.vendorId}/PID:${device.productId}",
                        status = if (usbManager.hasPermission(device)) "Connected" else "Permission Required"
                    )
                    Log.d(TAG, "Added USB printer: ${printerInfo.name}")
                    printers.add(printerInfo)
                }
            }
        } catch (e: Exception) {
            Log.e(TAG, "Error in USB discovery", e)
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
        Log.d(TAG, "Starting Bluetooth printer discovery")
        val printers = mutableListOf<ThermalPrinterInfo>()
        
        val bluetoothAdapter = bluetoothManager?.adapter
        
        if (bluetoothAdapter == null) {
            Log.w(TAG, "Bluetooth adapter is null")
            return printers
        }
        
        if (!bluetoothAdapter.isEnabled) {
            Log.w(TAG, "Bluetooth is not enabled")
            return printers
        }

        // Verificar permisos
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

        try {
            // Obtener dispositivos emparejados
            val pairedDevices: Set<BluetoothDevice> = bluetoothAdapter.bondedDevices
            Log.d(TAG, "Found ${pairedDevices.size} paired Bluetooth devices")
            
            pairedDevices.forEach { device ->
                try {
                    val deviceClass = device.bluetoothClass?.deviceClass
                    Log.d(TAG, "Checking BT device: ${device.name}, class=$deviceClass, address=${device.address}")
                    
                    val isPrinter = deviceClass == 1664 || 
                               device.name?.contains("printer", true) == true ||
                               device.name?.contains("print", true) == true
                    
                    if (isPrinter) {
                        val printerInfo = ThermalPrinterInfo(
                            name = device.name ?: "Bluetooth Printer",
                            interfaceType = "Bluetooth",
                            identifier = device.address,
                            status = when (device.bondState) {
                                BluetoothDevice.BOND_BONDED -> "Paired"
                                BluetoothDevice.BOND_BONDING -> "Pairing"
                                else -> "Not Paired"
                            }
                        )
                        Log.d(TAG, "Added Bluetooth printer: ${printerInfo.name}")
                        printers.add(printerInfo)
                    }
                } catch (e: Exception) {
                    Log.e(TAG, "Error processing Bluetooth device", e)
                }
            }
        } catch (e: Exception) {
            Log.e(TAG, "Error getting bonded devices", e)
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