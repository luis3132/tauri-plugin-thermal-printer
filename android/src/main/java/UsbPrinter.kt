package com.luis3132.thermal_printer

import android.app.PendingIntent
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.hardware.usb.UsbConstants
import android.hardware.usb.UsbDevice
import android.hardware.usb.UsbEndpoint
import android.hardware.usb.UsbInterface
import android.hardware.usb.UsbManager
import android.os.Build
import android.util.Log
import androidx.core.content.ContextCompat
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

/**
 * Sends a raw ESC/POS payload to a USB printer.
 *
 * The [identifier] must be the one produced by [PrinterDiscovery] for USB devices:
 * `"VID:<vendorId>/PID:<productId>"` (decimal). The only manual step Android cannot
 * skip is the one-time system permission dialog, which is handled here at runtime.
 */
class UsbPrinter(private val context: Context) {

    private val TAG = "UsbPrinter"

    private val usbManager: UsbManager
        get() = context.getSystemService(Context.USB_SERVICE) as UsbManager

    fun printRawData(identifier: String, data: ByteArray) {
        val (vendorId, productId) = parseIdentifier(identifier)

        val device = usbManager.deviceList.values.firstOrNull {
            it.vendorId == vendorId && it.productId == productId
        } ?: throw IllegalStateException("USB printer not connected: $identifier")

        if (!usbManager.hasPermission(device)) {
            Log.d(TAG, "Requesting USB permission for $identifier")
            requestPermissionBlocking(device)
        }
        if (!usbManager.hasPermission(device)) {
            throw SecurityException("USB permission denied for $identifier")
        }

        val (usbInterface, endpoint) = findPrinterEndpoint(device)
            ?: throw IllegalStateException("No USB printer bulk-OUT endpoint found on $identifier")

        val connection = usbManager.openDevice(device)
            ?: throw IllegalStateException("Cannot open USB device $identifier")

        try {
            if (!connection.claimInterface(usbInterface, true)) {
                throw IllegalStateException("Cannot claim USB interface on $identifier")
            }

            Log.d(TAG, "Sending ${data.size} bytes over USB to $identifier")
            var offset = 0
            val chunkSize = 16384
            while (offset < data.size) {
                val len = minOf(chunkSize, data.size - offset)
                val chunk = data.copyOfRange(offset, offset + len)
                val sent = connection.bulkTransfer(endpoint, chunk, len, TRANSFER_TIMEOUT_MS)
                if (sent < 0) {
                    throw IllegalStateException("USB bulkTransfer failed at offset $offset")
                }
                offset += sent
            }
            Log.d(TAG, "USB print complete")
        } finally {
            try { connection.releaseInterface(usbInterface) } catch (_: Exception) {}
            connection.close()
        }
    }

    private fun parseIdentifier(identifier: String): Pair<Int, Int> {
        // Expected: "VID:1234/PID:5678"
        try {
            val parts = identifier.removePrefix("VID:").split("/PID:")
            return parts[0].trim().toInt() to parts[1].trim().toInt()
        } catch (e: Exception) {
            throw IllegalArgumentException("Invalid USB identifier: $identifier")
        }
    }

    private fun findPrinterEndpoint(device: UsbDevice): Pair<UsbInterface, UsbEndpoint>? {
        // Prefer a real printer-class interface (class 7), then any bulk-OUT interface.
        for (preferPrinterClass in listOf(true, false)) {
            for (i in 0 until device.interfaceCount) {
                val usbInterface = device.getInterface(i)
                if (preferPrinterClass && usbInterface.interfaceClass != USB_CLASS_PRINTER) continue
                val endpoint = bulkOutEndpoint(usbInterface)
                if (endpoint != null) return usbInterface to endpoint
            }
        }
        return null
    }

    private fun bulkOutEndpoint(usbInterface: UsbInterface): UsbEndpoint? {
        for (e in 0 until usbInterface.endpointCount) {
            val endpoint = usbInterface.getEndpoint(e)
            if (endpoint.type == UsbConstants.USB_ENDPOINT_XFER_BULK &&
                endpoint.direction == UsbConstants.USB_DIR_OUT
            ) {
                return endpoint
            }
        }
        return null
    }

    private fun requestPermissionBlocking(device: UsbDevice) {
        val latch = CountDownLatch(1)
        val receiver = object : BroadcastReceiver() {
            override fun onReceive(ctx: Context, intent: Intent) {
                if (intent.action == ACTION_USB_PERMISSION) latch.countDown()
            }
        }

        val filter = IntentFilter(ACTION_USB_PERMISSION)
        // On API 33+ a receiver must declare its exported state explicitly.
        ContextCompat.registerReceiver(
            context, receiver, filter, ContextCompat.RECEIVER_NOT_EXPORTED
        )

        try {
            val piFlags =
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                    PendingIntent.FLAG_MUTABLE or PendingIntent.FLAG_UPDATE_CURRENT
                } else {
                    PendingIntent.FLAG_UPDATE_CURRENT
                }
            // Explicit (package-scoped) intent so the PendingIntent is not treated as implicit.
            val intent = Intent(ACTION_USB_PERMISSION).setPackage(context.packageName)
            val pendingIntent = PendingIntent.getBroadcast(context, 0, intent, piFlags)

            usbManager.requestPermission(device, pendingIntent)
            latch.await(PERMISSION_TIMEOUT_SECONDS, TimeUnit.SECONDS)
        } finally {
            try { context.unregisterReceiver(receiver) } catch (_: Exception) {}
        }
    }

    companion object {
        private const val USB_CLASS_PRINTER = 7
        private const val TRANSFER_TIMEOUT_MS = 5000
        private const val PERMISSION_TIMEOUT_SECONDS = 30L
        private const val ACTION_USB_PERMISSION = "com.luis3132.thermal_printer.USB_PERMISSION"
    }
}
