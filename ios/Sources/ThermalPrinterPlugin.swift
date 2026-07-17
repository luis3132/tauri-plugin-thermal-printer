import SwiftRs
import Tauri
import UIKit
import WebKit

class PrintRawArgs: Decodable {
  let identifier: String
  /// Final ESC/POS payload, Base64-encoded by the Rust side (see `src/mobile.rs`).
  let data: String
}

/// iOS transport layer for the thermal-printer plugin.
///
/// The ESC/POS bytes are generated in Rust (shared with every platform); this plugin only
/// routes the final payload to the printer. iOS only allows two viable transports for
/// generic ESC/POS printers:
///   - **Network / WiFi**: RAW/JetDirect TCP socket (port 9100). Discovery via Bonjour.
///   - **BLE**: CoreBluetooth for BLE-capable printers.
/// Bluetooth Classic (SPP) and USB are not available to third-party iOS apps.
class ThermalPrinterPlugin: Plugin {

  // Retained so async discovery/printing is not deallocated mid-flight.
  private let networkDiscovery = NetworkDiscovery()
  private let bleDiscovery = BLEDiscovery()
  private var networkPrinter: NetworkPrinter?
  private var blePrinter: BLEPrinter?

  // ─────────────────────────────────────────────────────────────
  // list_thermal_printers — Network (Bonjour) + BLE (CoreBluetooth)
  // ─────────────────────────────────────────────────────────────

  @objc public func list_thermal_printers(_ invoke: Invoke) throws {
    let lock = NSLock()
    var results: [[String: Any]] = []
    let group = DispatchGroup()

    group.enter()
    networkDiscovery.discover { printers in
      lock.lock(); results.append(contentsOf: printers); lock.unlock()
      group.leave()
    }

    group.enter()
    bleDiscovery.discover { printers in
      lock.lock(); results.append(contentsOf: printers); lock.unlock()
      group.leave()
    }

    group.notify(queue: .main) {
      invoke.resolve(["printers": results])
    }
  }

  // ─────────────────────────────────────────────────────────────
  // print_raw_data — routes by identifier format
  // ─────────────────────────────────────────────────────────────

  @objc public func print_raw_data(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(PrintRawArgs.self)

    guard let data = Data(base64Encoded: args.data) else {
      invoke.reject("Invalid print data encoding")
      return
    }

    let identifier = args.identifier

    if identifier.hasPrefix("BLE:") {
      let printer = BLEPrinter()
      self.blePrinter = printer
      printer.printRawData(identifier: identifier, data: data) { error in
        if let error = error {
          invoke.reject("Print error: \(error.localizedDescription)")
        } else {
          invoke.resolve()
        }
      }
    } else if identifier.contains(":") {
      let printer = NetworkPrinter()
      self.networkPrinter = printer
      printer.printRawData(identifier: identifier, data: data) { error in
        if let error = error {
          invoke.reject("Print error: \(error.localizedDescription)")
        } else {
          invoke.resolve()
        }
      }
    } else {
      invoke.reject("Unrecognized printer identifier: \(identifier)")
    }
  }
}

@_cdecl("init_plugin_thermal_printer")
func initPlugin() -> Plugin {
  return ThermalPrinterPlugin()
}
