import Foundation
import CoreBluetooth

/// Scans for nearby BLE printers via CoreBluetooth. iOS does not expose device classes over
/// BLE, so printers are identified heuristically by advertised name (same approach as the
/// Android Bluetooth discovery). The returned identifier is `"BLE:<peripheral-uuid>"`.
///
/// > Note: the host app's `Info.plist` must declare `NSBluetoothAlwaysUsageDescription`.
final class BLEDiscovery: NSObject, CBCentralManagerDelegate {

  private let queue = DispatchQueue(label: "com.luis3132.thermal_printer.ble.discovery")

  private var central: CBCentralManager?
  private var found: [String: [String: Any]] = [:] // uuid -> PrinterInfo
  private var completion: (([[String: Any]]) -> Void)?
  private var finished = false

  private let nameHints = ["print", "pos", "thermal", "escpos", "receipt"]

  func discover(timeout: TimeInterval = 4.0, completion: @escaping ([[String: Any]]) -> Void) {
    queue.async {
      self.completion = completion
      self.central = CBCentralManager(delegate: self, queue: self.queue)

      self.queue.asyncAfter(deadline: .now() + timeout) { [weak self] in
        self?.finish()
      }
    }
  }

  func centralManagerDidUpdateState(_ central: CBCentralManager) {
    switch central.state {
    case .poweredOn:
      central.scanForPeripherals(withServices: nil, options: nil)
    case .unknown, .resetting:
      break // wait for a definitive state
    default:
      // unauthorized / poweredOff / unsupported — no BLE results available.
      finish()
    }
  }

  func centralManager(
    _ central: CBCentralManager,
    didDiscover peripheral: CBPeripheral,
    advertisementData: [String: Any],
    rssi RSSI: NSNumber
  ) {
    let name = peripheral.name
      ?? (advertisementData[CBAdvertisementDataLocalNameKey] as? String)
      ?? ""
    guard !name.isEmpty else { return }

    let lower = name.lowercased()
    guard nameHints.contains(where: { lower.contains($0) }) else { return }

    let uuid = peripheral.identifier.uuidString
    found[uuid] = [
      "name": name,
      "interface_type": "BLE",
      "identifier": "BLE:\(uuid)",
      "status": "Available"
    ]
  }

  private func finish() {
    if finished { return }
    finished = true
    central?.stopScan()
    completion?(Array(found.values))
    completion = nil
  }
}
