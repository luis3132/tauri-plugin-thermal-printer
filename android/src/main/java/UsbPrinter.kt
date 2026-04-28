package com.luis3132.thermal_printer

import android.app.PendingIntent
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.hardware.usb.UsbConstants
import android.hardware.usb.UsbDevice
import android.hardware.usb.UsbManager
import android.hardware.usb.UsbEndpoint
import android.hardware.usb.UsbInterface
import android.os.Build
import android.util.Log

class UsbPrinter(private val context: Context) {

    private val TAG = "UsbPrinter"
    private val ACTION_USB_PERMISSION = "com.luis3132.thermal_printer.USB_PERMISSION"

    fun printRawData(vid: Int, pid: Int, data: ByteArray) {
        val usbManager = context.getSystemService(Context.USB_SERVICE) as UsbManager
        
        // Buscar el dispositivo USB por su VID y PID
        val deviceList = usbManager.deviceList
        var targetDevice: UsbDevice? = null
        for (device in deviceList.values) {
            if (device.vendorId == vid && device.productId == pid) {
                targetDevice = device
                break
            }
        }

        if (targetDevice == null) {
            throw IllegalStateException("USB device with VID:$vid PID:$pid not found")
        }

        // Solicitar permisos dinámicamente si no se tienen
        if (!usbManager.hasPermission(targetDevice)) {
            Log.d(TAG, "Requesting USB permission")
            val flags = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                PendingIntent.FLAG_MUTABLE or PendingIntent.FLAG_UPDATE_CURRENT
            } else {
                PendingIntent.FLAG_UPDATE_CURRENT
            }
            val permissionIntent = PendingIntent.getBroadcast(context, 0, Intent(ACTION_USB_PERMISSION), flags)
            
            val lock = Object()
            var permissionGranted = false
            
            val usbReceiver = object : BroadcastReceiver() {
                override fun onReceive(context: Context, intent: Intent) {
                    if (ACTION_USB_PERMISSION == intent.action) {
                        synchronized(this@UsbPrinter) {
                            val device: UsbDevice? = intent.getParcelableExtra(UsbManager.EXTRA_DEVICE)
                            if (intent.getBooleanExtra(UsbManager.EXTRA_PERMISSION_GRANTED, false)) {
                                if (device != null) {
                                    permissionGranted = true
                                }
                            } else {
                                Log.w(TAG, "Permission denied for device $device")
                            }
                            synchronized(lock) {
                                lock.notify()
                            }
                        }
                    }
                }
            }

            // Registrar y solicitar
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                context.registerReceiver(usbReceiver, IntentFilter(ACTION_USB_PERMISSION), Context.RECEIVER_NOT_EXPORTED)
            } else {
                context.registerReceiver(usbReceiver, IntentFilter(ACTION_USB_PERMISSION))
            }
            
            usbManager.requestPermission(targetDevice, permissionIntent)

            // Esperar a que el usuario interactúe con el diálogo de permisos
            synchronized(lock) {
                try {
                    lock.wait(30000) // Timeout de 30 segundos
                } catch (e: InterruptedException) {
                    Log.e(TAG, "Interrupted while waiting for USB permission")
                }
            }
            
            context.unregisterReceiver(usbReceiver)

            if (!permissionGranted) {
                throw SecurityException("USB permission denied by user")
            }
        }

        Log.d(TAG, "Opening connection to USB printer")
        val connection = usbManager.openDevice(targetDevice)
            ?: throw IllegalStateException("Could not open USB connection")

        var usbInterface: UsbInterface? = null
        var epOut: UsbEndpoint? = null

        // Buscar el endpoint de salida bulk
        for (i in 0 until targetDevice.interfaceCount) {
            val intf = targetDevice.getInterface(i)
            for (j in 0 until intf.endpointCount) {
                val ep = intf.getEndpoint(j)
                if (ep.direction == UsbConstants.USB_DIR_OUT && ep.type == UsbConstants.USB_ENDPOINT_XFER_BULK) {
                    usbInterface = intf
                    epOut = ep
                    break
                }
            }
            if (epOut != null) break
        }

        if (usbInterface == null || epOut == null) {
            connection.close()
            throw IllegalStateException("Could not find USB bulk out endpoint")
        }

        if (!connection.claimInterface(usbInterface, true)) {
            connection.close()
            throw IllegalStateException("Could not claim USB interface")
        }

        try {
            Log.d(TAG, "Connected, sending ${data.size} bytes via bulk transfer")
            // Enviar en partes si es muy grande (ej. max 16KB por transferencia)
            val chunkSize = 16384
            var offset = 0
            while (offset < data.size) {
                val length = Math.min(data.size - offset, chunkSize)
                val chunk = data.copyOfRange(offset, offset + length)
                val transferred = connection.bulkTransfer(epOut, chunk, length, 5000)
                if (transferred < 0) {
                    throw IllegalStateException("Bulk transfer failed at offset $offset")
                }
                offset += length
            }
            Log.d(TAG, "Print complete")
        } finally {
            connection.releaseInterface(usbInterface)
            connection.close()
        }
    }
}
