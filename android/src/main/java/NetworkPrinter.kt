package com.luis3132.thermal_printer

import android.util.Log
import java.net.InetSocketAddress
import java.net.Socket

class NetworkPrinter {

    private val TAG = "NetworkPrinter"

    fun printRawData(ip: String, port: Int, data: ByteArray) {
        Log.d(TAG, "Connecting to network printer $ip:$port (${data.size} bytes)")

        var socket: Socket? = null
        try {
            socket = Socket()
            // 5 seconds connection timeout
            socket.connect(InetSocketAddress(ip, port), 5000)
            
            Log.d(TAG, "Connected, sending ${data.size} bytes")
            socket.getOutputStream().write(data)
            socket.getOutputStream().flush()
            Log.d(TAG, "Print complete")
        } finally {
            try { socket?.close() } catch (_: Exception) {}
        }
    }
}
