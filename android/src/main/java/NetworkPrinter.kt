package com.luis3132.thermal_printer

import android.util.Log
import java.net.InetSocketAddress
import java.net.Socket

/**
 * Sends a raw ESC/POS payload to a network printer over a RAW/JetDirect TCP socket
 * (typically port 9100). The [identifier] must be `"<host>:<port>"`, as produced by
 * [PrinterDiscovery] network discovery.
 */
class NetworkPrinter {

    private val TAG = "NetworkPrinter"

    fun printRawData(identifier: String, data: ByteArray) {
        val host = identifier.substringBeforeLast(':')
        val port = identifier.substringAfterLast(':').toIntOrNull()
            ?: throw IllegalArgumentException("Invalid network identifier: $identifier")

        Log.d(TAG, "Connecting to $host:$port (${data.size} bytes)")
        Socket().use { socket ->
            socket.connect(InetSocketAddress(host, port), CONNECT_TIMEOUT_MS)
            socket.getOutputStream().apply {
                write(data)
                flush()
            }
            Log.d(TAG, "Network print complete")
        }
    }

    companion object {
        private const val CONNECT_TIMEOUT_MS = 5000
    }
}
