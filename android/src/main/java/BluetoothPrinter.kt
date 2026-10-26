package com.luis3132.thermal_printer

import android.Manifest
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import android.bluetooth.BluetoothSocket
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import android.util.Log
import androidx.core.app.ActivityCompat
import java.io.IOException
import java.util.UUID

class BluetoothPrinter(private val context: Context) {

    private val TAG = "BluetoothPrinter"
    private val SPP_UUID = UUID.fromString("00001101-0000-1000-8000-00805F9B34FB")

    private val bluetoothAdapter: BluetoothAdapter? by lazy {
        val manager = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            context.getSystemService(BluetoothManager::class.java)
        } else {
            @Suppress("DEPRECATION")
            context.getSystemService(Context.BLUETOOTH_SERVICE) as? BluetoothManager
        }
        manager?.adapter
    }

    fun printRawData(macAddress: String, data: ByteArray) {
        val adapter = bluetoothAdapter
            ?: throw IllegalStateException("Bluetooth adapter not available")

        if (!adapter.isEnabled) {
            throw IllegalStateException("Bluetooth is disabled")
        }

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            if (ActivityCompat.checkSelfPermission(
                    context,
                    Manifest.permission.BLUETOOTH_CONNECT
                ) != PackageManager.PERMISSION_GRANTED
            ) {
                throw SecurityException("BLUETOOTH_CONNECT permission not granted")
            }
        }

        Log.d(TAG, "Connecting to $macAddress (${data.size} bytes)")
        val device = adapter.getRemoteDevice(macAddress)
        // Discovery slows down / breaks an active RFCOMM connection.
        adapter.cancelDiscovery()

        val socket = connectSocket(device)
        try {
            Log.d(TAG, "Connected, sending ${data.size} bytes")
            val started = System.currentTimeMillis()
            val out = socket.outputStream
            var offset = 0
            while (offset < data.size) {
                val len = minOf(WRITE_CHUNK_SIZE, data.size - offset)
                out.write(data, offset, len)
                out.flush()
                offset += len
            }
            waitUntilDelivered(socket, data.size)
            Log.d(TAG, "Print complete in ${System.currentTimeMillis() - started} ms")
        } finally {
            try { socket.close() } catch (_: Exception) {}
        }
    }

    /**
     * `write()` only copies into the Bluetooth stack's buffer and returns long before the
     * bytes reach the printer; closing the socket right away discards whatever is still
     * queued, so large jobs (images) print partially. RFCOMM delivers in order, so we send
     * a real-time status request (`DLE EOT 1`) and wait for its reply: once it arrives the
     * printer has received everything before it. Printers that never answer fall back to
     * a timeout proportional to the job size.
     */
    private fun waitUntilDelivered(socket: BluetoothSocket, size: Int) {
        val input = socket.inputStream
        try {
            while (input.available() > 0) input.read()
        } catch (_: IOException) {}

        socket.outputStream.write(byteArrayOf(0x10, 0x04, 0x01))
        socket.outputStream.flush()

        val timeoutMs = (MIN_DRAIN_MS + size * 1000L / SLOW_LINK_BYTES_PER_SEC)
            .coerceAtMost(MAX_DRAIN_MS)
        val deadline = System.currentTimeMillis() + timeoutMs
        while (System.currentTimeMillis() < deadline) {
            val available = try { input.available() } catch (_: IOException) { 0 }
            if (available > 0) {
                Log.d(TAG, "Printer acknowledged status request; data delivered")
                return
            }
            Thread.sleep(POLL_INTERVAL_MS)
        }
        Log.d(TAG, "No status reply after $timeoutMs ms; closing")
    }

    private companion object {
        const val WRITE_CHUNK_SIZE = 1024
        const val POLL_INTERVAL_MS = 20L
        const val MIN_DRAIN_MS = 1000L
        const val MAX_DRAIN_MS = 20_000L
        // Pessimistic SPP throughput used to size the fallback wait.
        const val SLOW_LINK_BYTES_PER_SEC = 5_000L
    }

    /**
     * Resolves a paired device's MAC address from its name (case-insensitive), so callers
     * that pass the printer name (as on desktop) still work. Returns null if not found.
     */
    fun findBondedAddressByName(name: String): String? {
        val adapter = bluetoothAdapter ?: return null
        if (!adapter.isEnabled) return null
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S &&
            ActivityCompat.checkSelfPermission(
                context,
                Manifest.permission.BLUETOOTH_CONNECT
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            return null
        }
        return adapter.bondedDevices
            ?.firstOrNull { it.name?.trim()?.equals(name.trim(), ignoreCase = true) == true }
            ?.address
    }

    /**
     * Opens an RFCOMM socket. Tries the standard secure SPP socket first and, if that
     * fails (common on cheap POS printers with a non-standard SDP record), falls back to
     * the well-known reflection-based `createRfcommSocket(1)` workaround.
     */
    private fun connectSocket(device: BluetoothDevice): BluetoothSocket {
        try {
            val socket = device.createRfcommSocketToServiceRecord(SPP_UUID)
            socket.connect()
            return socket
        } catch (primary: IOException) {
            Log.w(TAG, "Standard RFCOMM connect failed, trying reflection fallback: ${primary.message}")
            try {
                val method = device.javaClass.getMethod(
                    "createRfcommSocket",
                    Int::class.javaPrimitiveType
                )
                val socket = method.invoke(device, 1) as BluetoothSocket
                socket.connect()
                return socket
            } catch (fallback: Exception) {
                throw IOException(
                    "Bluetooth connection failed " +
                        "(standard: ${primary.message}; fallback: ${fallback.message})"
                )
            }
        }
    }
}
